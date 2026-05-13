use crate::events::{Event, MonthDay, Category};

pub struct EventFilter {
    month_day: Option<MonthDay>,
    description_contains: Option<String>,
    category_matches: Option<Category>,
}

impl EventFilter {
    pub fn month_day(&self) -> Option<MonthDay> {
        self.month_day.clone()
    }

    pub fn description_contains(&self) -> Option<String> {
        self.description_contains.clone()
    }

    pub fn category_matches(&self) -> Option<Category> {
        self.category_matches.clone()
    }
}

impl EventFilter {
    pub fn accepts(&self, event: &Event) -> bool {
        if let Some(month_day) = &self.month_day {
            if event.month_day() != *month_day {
                return false;
            }
        }

        if let Some(ref text) = self.description_contains {
            if !event.description().to_lowercase().contains(text) {
                return false;
            }
        }

        if let Some(ref filter_cat) = self.category_matches {
            if event.category() != *filter_cat {
                return false;
            }
        }

        true
    }
}

pub struct FilterBuilder {
    month_day: Option<MonthDay>,
    description_contains: Option<String>,
    category_matches: Option<Category>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            month_day: None,
            description_contains: None,
            category_matches: None,
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> Self {
        self.month_day = Some(month_day);
        self
    }

    pub fn description_contains(mut self, text: impl Into<String>) -> Self {
        self.description_contains = Some(text.into().to_lowercase());
        self
    }

    pub fn category_matches(mut self, category: &Category) -> Self {
        self.category_matches = Some(category.clone());
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            month_day: self.month_day,
            description_contains: self.description_contains,
            category_matches: self.category_matches,
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Event, EventDate, Category};
    use chrono::NaiveDate;

    #[test]
    fn filter_accepts_anything() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new(
            EventDate::Singular(NaiveDate::from_ymd_opt(2026, 3, 5).unwrap()), 
            "Rust 1.94.0 released".to_string(), 
            rust_category.clone());
        let filter = FilterBuilder::new()
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn build_filter_no_options() {
        let filter = FilterBuilder::new().build();
        let contains = (
            filter.month_day(),
            filter.category_matches(),
            filter.description_contains());
        assert_eq!(contains, (None, None, None));
    }    
}
