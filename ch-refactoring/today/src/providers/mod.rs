use crate::Event;
use crate::filters::EventFilter;

pub mod textfile;
pub mod csvfile;
pub mod sqlite;
pub mod web;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
    fn is_add_supported(&self) -> bool { false }
    fn add_event(&self, event: &Event) -> Result<(), EventProviderError>;
    fn kind(&self) -> String;
}

pub enum EventProviderError {
    OperationNotSupported,
    OperationFailed,
}
