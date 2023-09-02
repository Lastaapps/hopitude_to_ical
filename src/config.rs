use chrono::{DateTime, Duration, NaiveDate, Utc};
use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigDto {
    cal_num: u32,
    from: String,
    to: String,
    filename: String,
}

impl From<ConfigDto> for Config {
    fn from(val: ConfigDto) -> Self {
        Config {
            cal_num: val.cal_num,
            from: ConfigDto::parse_date(&val.from),
            to: ConfigDto::parse_date(&val.to),
            filename: val.filename,
        }
    }
}
impl ConfigDto {
    fn parse_date(date: &str) -> DateTime<Utc> {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        let date_time = date.and_hms_milli_opt(0, 0, 0, 0).unwrap();
        DateTime::from_naive_utc_and_offset(date_time, Utc)
    }
}

pub struct Config {
    pub cal_num: u32,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub filename: String,
}

impl Config {
    fn default() -> Config {
        Config {
            cal_num: 66,
            // yeah, I know that this is not ideal, but good enough
            from: (Utc::now() - Duration::days(31)),
            to: Utc::now() + Duration::days(365),
            filename: String::from("hopitude.ical"),
        }
    }
}

pub fn load_config() -> Config {
    let directories = ["hopitude.toml"];

    let mut cfg_dto: Option<ConfigDto> = None;

    for dir in directories {
        println!("Trying {}", dir);
        match ConfigDto::from_config_file(dir) {
            Ok(cfg) => {
                println!("Config found!");
                cfg_dto = Some(cfg);
                break;
            }
            Err(e) => match &e {
                config_file::ConfigFileError::FileAccess(_) => continue,
                config_file::ConfigFileError::Toml(toml) => panic!("{}, {}", e, toml),
                config_file::ConfigFileError::UnsupportedFormat => panic!("{}", e),
            },
        };
    }

    if let Some(cfg) = cfg_dto {
        cfg.into()
    } else {
        println!("Using the default config");
        Config::default()
    }
}
