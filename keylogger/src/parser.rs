use chrono::prelude::*;

pub struct CalendarEvent {
    time: DateTime<Local>,
    description: String
}

pub fn parse(input : String) -> CalendarEvent {
    CalendarEvent { time : Local::now(), description : input }
}
