use chrono::prelude::Utc;
use chrono::TimeZone;
use chrono::{DateTime, NaiveDateTime};
use chrono_tz::Europe::Tallinn;
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

impl From<EventsDto> for Vec<Event> {
    fn from(val: EventsDto) -> Self {
        val.events.into_iter().map(|e| e.into()).collect()
    }
}

impl From<EventDto> for Event {
    fn from(val: EventDto) -> Self {
        Event {
            id: val.id,
            title: val.title,
            start: val.start as u64,
            end: val.end as u64,
            coach: val.coach,
            room: val.room,
            free_seats: val.free_seats,
            total_seats: val.total_seats,
        }
    }
}

pub fn create_url(cal_num: u32, from: DateTime<Utc>, to: DateTime<Utc>) -> String {
    format!(
        "https://admin.hopitude.com/api/v1/calendar/workout-events/club/{}/?from={}&to={}",
        cal_num,
        from.timestamp_millis(),
        to.timestamp_millis()
    )
}

pub fn do_request_and_parse(url: &str) -> Vec<Event> {
    let request = reqwest::blocking::get(url).unwrap();
    let json: EventsDto = request.json().unwrap();

    json.into()
}

pub fn export_events(events: &[Event]) -> String {
    let mut calendar = Calendar::new();

    for event in events.iter() {
        let seats_str = match (event.free_seats, event.total_seats) {
            (Some(f), Some(t)) => format!(" {}/{}", t - f, t),
            (Some(f), None) => format!(" {} free", f),
            (None, Some(t)) => format!(" {} places", t),
            (None, None) => String::new(),
        };

        let room_text = match &event.room {
            Some(room) => room.as_str(),
            None => "",
        };

        let start: DateTime<Utc> = {
            let datetime = NaiveDateTime::from_timestamp_millis(event.start as i64).unwrap();
            Tallinn
                .from_local_datetime(&datetime)
                .unwrap()
                .with_timezone(&Utc)
        };
        let end: DateTime<Utc> = {
            let datetime = NaiveDateTime::from_timestamp_millis(event.end as i64).unwrap();
            Tallinn
                .from_local_datetime(&datetime)
                .unwrap()
                .with_timezone(&Utc)
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
