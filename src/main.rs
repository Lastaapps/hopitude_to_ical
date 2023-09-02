
mod config;

fn main() {
    let config = config::load_config();

    let base_url = format!("https://admin.hopitude.com/api/v1/calendar/workout-events/club/{}/?from={}&to={}", config.cal_num, config.from, config.to);

    println!("{}", base_url);
}

