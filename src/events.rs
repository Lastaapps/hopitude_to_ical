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
        self.events
            .into_iter()
            .map(|e| e.to_domain())
            .collect()
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

