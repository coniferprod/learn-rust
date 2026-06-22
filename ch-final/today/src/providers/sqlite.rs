use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, params};
use chrono::NaiveDate;

use crate::events::{Event, EventDate, Category};
use crate::providers::{EventProvider, EventProviderError};
use crate::filters::EventFilter;

pub struct SQLiteProvider {
    name: String,
    path: PathBuf,
    is_active: bool,
}

impl SQLiteProvider {
    pub fn new(name: &str, path: &Path, is_active: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            is_active,
        }
    }
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let connection = Connection::open(self.path.clone()).unwrap();
        let mut db_events = get_events(&connection, &filter);
        events.append(&mut db_events);
    }

    // Override the default implementation from the trait:
    fn is_add_supported(&self) -> bool { true }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        let connection = Connection::open(self.path.clone()).unwrap();

        // Find out if the category of the event is already there.
        match find_category_id(&event.category(), &connection) {
            Some(category_id) => {
                // Found it, now insert the event to the database with this category ID:
                insert_event(&connection, event, category_id);
            },
            None => {
                // Add a new category first, then add the event
                let category = event.category();
                let primary_str = category.primary();
                match category.secondary() {
                    Some(secondary) => connection.execute(
                        "INSERT INTO category (primary_name, secondary_name) VALUES (?1, ?2)",
                        params![primary_str, secondary]).unwrap(),
                    None => connection.execute(
                        "INSERT INTO category (primary_name) VALUES (?1)",
                        [primary_str]).unwrap()
                };

                let category_id = connection.last_insert_rowid();
                insert_event(&connection, event, category_id);

                return Ok(());
            }
        }

        Err(EventProviderError::OperationNotSupported)
    }

    fn kind(&self) -> String {
        String::from("SQLite")
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

fn insert_event(connection: &Connection, event: &Event, category_id: CategoryId) {
    let event_date_str = format!("{:04}-{}", event.year(), event.month_day());
    connection.execute(
        "INSERT INTO event (event_date, event_description, category_id) VALUES (?1, ?2, ?3)",
        params![event_date_str, event.description(), category_id]).unwrap();
}

type CategoryId = i64;
type CategoryMap = HashMap<CategoryId, Category>;

#[derive(Debug)]
pub struct CategoryRow {
    category_id: CategoryId,
    primary_name: String,
    secondary_name: Option<String>,
}

#[derive(Debug)]
struct EventRow {
    event_date: String,
    event_description: String,
    category_id: CategoryId,
}

fn get_categories(connection: &Connection) -> CategoryMap {
    let mut category_map: CategoryMap = HashMap::new();

    let category_query = "SELECT category_id, primary_name, secondary_name FROM category";
    let mut statement = connection.prepare(category_query).unwrap();
    let category_iter = statement.query_map([], |row| {
        Ok(CategoryRow {
            category_id: row.get_unwrap(0),
            primary_name: row.get_unwrap(1),
            secondary_name: row.get_unwrap(2),
        })
    }).unwrap();
    
    for row in category_iter {
        let r = row.unwrap();
        let category = match r.secondary_name {
            Some(secondary) => Category::new(&r.primary_name, &secondary),
            None => Category::from_primary(&r.primary_name),
        };
        category_map.insert(r.category_id, category);
    }
    category_map
}

fn find_category_id(category: &Category, connection: &Connection) -> Option<CategoryId> {
    // Get the categories currently in the database.
    let category_map = get_categories(connection);

    // Use a brute force method where you iterate the hash map keys,
    // and stop if one of the values matches the category.
    for (id, cat) in &category_map {
        if *cat == *category { // found it!
            return Some(*id);
        }
    }

    None
}

fn get_events(connection: &Connection, filter: &EventFilter) -> Vec<Event> {
    let category_map = get_categories(&connection);

    let mut event_query
        = "SELECT event_date, event_description, category_id FROM event".to_string();
    let where_clause = make_where_clause(filter, &category_map);
    if !where_clause.is_empty() {
        event_query.push(' ');
        event_query.push_str(&where_clause);
    }

    let mut statement = connection.prepare(&event_query).unwrap();
    let event_iter = statement.query_map([], |row| {
        Ok(EventRow {
            event_date: row.get_unwrap(0),
            event_description: row.get_unwrap(1),
            category_id: row.get_unwrap(2),
        })
    }).unwrap();

    let mut events: Vec<Event> = Vec::new();
    for row in event_iter {
        let r = row.unwrap();
        let date = NaiveDate::parse_from_str(&r.event_date, "%F").unwrap();
        let category = category_map.get(&r.category_id).unwrap();
        let event = Event::new(
            EventDate::Singular(date),
            r.event_description,
            category.clone());
        events.push(event);
    }

    events
}

fn make_where_clause(filter: &EventFilter, category_map: &CategoryMap) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(month_day) = filter.month_day() {
        let part = format!("strftime('%m-%d', event_date) = '{}'", month_day);
        parts.push(part);
    }

    if let Some(category) = filter.category_matches() {
        let mut filter_category_id: Option<CategoryId> = None;

        for (category_id, found_category) in category_map {
            if *found_category == category {
                filter_category_id = Some(*category_id);
                break;
            }
        }

        if let Some(id) = filter_category_id {
            parts.push(format!("category_id = {}", id))
        }
    }

    if let Some(text) = filter.description_contains() {
        parts.push(format!("event_description LIKE '%{}%'", text));
    }

    let mut result = "".to_string();
    if !parts.is_empty() {
        result.push_str("WHERE ");
        result.push_str(&parts.join(" AND "));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Category, MonthDay};
    use crate::filters::FilterBuilder;
    use chrono::{NaiveDate, Datelike};
    use rusqlite::Connection;

    // Creates an in-memory SQLite database with the event and category tables,
    // then inserts one category (id=1, primary=test, secondary=NULL),
    // and one event.
    fn create_memory_db() -> Connection {
        let connection = Connection::open_in_memory().unwrap();

        connection.execute(
            "CREATE TABLE IF NOT EXISTS category(
                category_id INTEGER PRIMARY KEY,
                primary_name TEXT NOT NULL,
                secondary_name TEXT DEFAULT NULL
            )",
            (),
        ).unwrap();

        connection.execute(
            "INSERT INTO category VALUES (1, 'test', NULL)",
            (),
        ).unwrap();

        connection.execute(
            "CREATE TABLE IF NOT EXISTS event(
                event_id INTEGER PRIMARY KEY,
                event_date DATE NOT NULL,
                event_description TEXT NOT NULL,
                category_id INTEGER NOT NULL,
                FOREIGN KEY (category_id) REFERENCES category(category_id))",
            (),
        ).unwrap();

        connection.execute(
            "INSERT INTO event (event_date, event_description, category_id)
                VALUES ('2026-03-07', 'Unit test for SQLiteProvider', 1)",
            (),
        ).unwrap();

        connection
    }

    #[test]
    fn get_categories_returns_one() {
        let connection = create_memory_db();
        let category_map = get_categories(&connection);
        assert_eq!(category_map.len(), 1);
    }

    #[test]
    fn get_events_returns_one() {
        let connection = create_memory_db();
        let filter = FilterBuilder::new().build();
        let db_events = get_events(&connection, &filter);
        assert_eq!(db_events.len(), 1);
    }

    #[test]
    fn make_where_clause_empty() {
        let mut category_map = CategoryMap::new();
        category_map.insert(1, Category::from_primary("test"));
        let filter = FilterBuilder::new().build();
        let where_clause = make_where_clause(&filter, &category_map);
        assert_eq!(where_clause, "");
    }

    #[test]
    fn make_where_clause_month_day() {
        let mut category_map = CategoryMap::new();
        category_map.insert(1, Category::from_primary("test"));
        let today = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let filter = FilterBuilder::new()
            .month_day(MonthDay::new(today.month(), today.day()))
            .build();
        let where_clause = make_where_clause(&filter, &category_map);
        assert_eq!(where_clause, "WHERE strftime('%m-%d', event_date) = '03-07'"); 
    }    
}
