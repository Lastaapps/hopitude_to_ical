use chrono::NaiveDate;
use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigDto {
    cal_num: u32,
    from: String,
    to: String,
    filename: String,
}

impl ConfigDto {
    fn to_domain(self) -> Config {
        Config {
            cal_num: self.cal_num,
            from: ConfigDto::parse_date(&self.from),
            to: ConfigDto::parse_date(&self.to),
            filename: self.filename,
        }
    }

    fn parse_date(date: &str) -> u64 {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        date.and_hms_milli_opt(0, 0, 0, 0)
            .unwrap()
            .timestamp_millis() as u64
    }
}

pub struct Config {
    pub cal_num: u32,
    pub from: u64,
    pub to: u64,
    pub filename: String,
}

pub fn load_config() -> Config {
    let directories = ["hopitude.toml", "~/.config/hopitude.toml"];

    let mut cfg_dto: Option<ConfigDto> = None;

    for dir in directories {
        match ConfigDto::from_config_file(dir) {
            Ok(cfg) => {
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
    let cfg = if let Some(cfg) = cfg_dto {
        cfg
    } else {
        panic!("No config file found!");
    };

    cfg.to_domain()
}
