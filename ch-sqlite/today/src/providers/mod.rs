use crate::{Category, Event};
use chrono::{Datelike, Local, NaiveDate};

pub mod textfile;
pub mod csvfile;
pub mod sqlite;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}
