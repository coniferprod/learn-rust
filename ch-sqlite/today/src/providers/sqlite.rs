use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use chrono::NaiveDate;

use crate::events::{Event, Category};
use crate::providers::EventProvider;

pub struct SQLiteProvider {
    name: String,
    path: PathBuf,
}

impl SQLiteProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf()
        }
    }
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) {
        let connection = Connection::open(self.path.clone()).unwrap();
        let mut db_events = get_events(&connection);
        events.append(&mut db_events);
    }
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

fn get_events(connection: &Connection) -> Vec<Event> {
    let category_map = get_categories(&connection);

    let event_query
        = "SELECT event_date, event_description, category_id FROM event".to_string();
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
        events.push(
            Event::new_singular(
                date, 
                r.event_description, 
                category.clone()));
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::sqlite;
    use crate::events::Category;
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
        let db_events = get_events(&connection);
        assert_eq!(db_events.len(), 1);
    }
}
