use chrono::prelude::Utc;
use chrono::{DateTime, NaiveDateTime};
use icalendar::{Calendar, Class, Component, EventLike, Property};
use serde::Deserialize;

#[derive(Deserialize)]
struct EventsDto {
    events: Vec<EventDto>,
}

#[derive(Deserialize)]
struct EventDto {
    id: u32,
    title: String,
    start: f64,
    end: f64,
    coach: String,
    room: Option<String>,
    free_seats: Option<u32>,
    total_seats: Option<u32>,
}

pub struct Event {
    pub id: u32,
    pub title: String,
    pub start: u64,
    pub end: u64,
    pub coach: String,
    pub room: Option<String>,
    pub free_seats: Option<u32>,
    pub total_seats: Option<u32>,
}

impl EventsDto {
    fn to_domain(self) -> Vec<Event> {
        self.events.into_iter().map(|e| e.to_domain()).collect()
    }
}

impl EventDto {
    fn to_domain(self) -> Event {
        Event {
            id: self.id,
            title: self.title,
            start: self.start as u64,
            end: self.end as u64,
            coach: self.coach,
            room: self.room,
            free_seats: self.free_seats,
            total_seats: self.total_seats,
        }
    }
}

pub fn do_request_and_parse(url: &str) -> Vec<Event> {
    let request = reqwest::blocking::get(url).unwrap();
    let json: EventsDto = request.json().unwrap();

    json.to_domain()
}

pub fn export_events(events: &Vec<Event>) -> String {
    let mut calendar = Calendar::new();

    for event in events.iter() {
        let seats_str = match (event.free_seats, event.total_seats) {
            (Some(f), Some(t)) => format!(" {}/{}", f, t),
            (Some(f), None) => format!(" {}", f),
            (None, Some(t)) => format!(" {}", t),
            (None, None) => String::new(),
        };

        let room_text = match &event.room {
            Some(room) => room.as_str(),
            None => "",
        };

        let start: DateTime<Utc> = {
            let datetime = NaiveDateTime::from_timestamp_millis(event.start as i64).unwrap();
            DateTime::from_naive_utc_and_offset(datetime, Utc)
        };
        let end: DateTime<Utc> = {
            let datetime = NaiveDateTime::from_timestamp_millis(event.end as i64).unwrap();
            DateTime::from_naive_utc_and_offset(datetime, Utc)
        };

        let mut cal_ev = icalendar::Event::new();
        cal_ev
            .summary(format!("{}{}", event.title, seats_str).as_str())
            .description(format!("{}{}", event.coach.as_str(), room_text,).as_str())
            .class(Class::Public)
            .starts(start)
            .ends(end);

        if let Some(room) = &event.room {
            cal_ev.append_property(Property::new("LOCATION", room).done());
        };

        calendar.push(cal_ev.done());
    }

    calendar.done().to_string()
}
