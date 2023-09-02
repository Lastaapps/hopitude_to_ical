use std::{fs::File, io::Write};

pub fn save_calendar(file_name: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
