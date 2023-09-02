use std::error::Error;

use crate::events::export_events;

mod config;
mod events;
mod files;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello there!");

    println!("Reading the config");
    let config = config::load_config();

    println!("Making request and processing JSON");
    let url = events::create_url(config.cal_num, config.from, config.to);
    let events = events::do_request_and_parse(url.as_str());
    
    println!("Exporting iCal");
    let out_str = export_events(&events);

    println!("Saving the final iCal into {}", config.filename);
    files::save_calendar(&config.filename, out_str.as_str())?;

    println!("Done, see you later");
    println!("By LastaApps 2023");
    Ok(())
}

