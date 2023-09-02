
mod config;
mod events;


fn main() {
    println!("Hello, there!");

    println!("Reading the config");
    let config = config::load_config();

    println!("Making request");
    let url = format!("https://admin.hopitude.com/api/v1/calendar/workout-events/club/{}/?from={}&to={}", config.cal_num, config.from, config.to);

    println!("Processing JSON");
    events::do_request_and_parse(url.as_str());
    
    println!("Exporting iCal");
    println!("Done, see you");
}

