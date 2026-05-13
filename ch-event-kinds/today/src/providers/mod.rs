use crate::Event;
use crate::filters::EventFilter;

pub mod textfile;
pub mod csvfile;
pub mod sqlite;
pub mod web;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
}
