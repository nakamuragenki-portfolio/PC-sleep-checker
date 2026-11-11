use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local,Datelike,Duration};
use eframe::egui;
use image::load_from_memory;
use std::fs::{File, OpenOptions};
use std::io::{self,Write, BufRead, BufReader};
use std::path::PathBuf;
use csv::Reader;


pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn sleepclock(idle: u64, now: DateTime<Local>) -> DateTime<Local> {
    if idle < 1 {
        let now = Local::now();
        now
    } else {
        now
    }
}

pub fn sleepfrg(idle: u64) -> bool {
    if idle >= 300 {
        true
    } else {
        false
    }
}

pub fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../../assets/icon.png");

    let image = load_from_memory(bytes)
        .unwrap()
        .into_rgba8();

    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

pub fn write_new_file(filepath: &PathBuf, text: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filepath)?;

    writeln!(file, "{text}")?;

    Ok(())
}

pub fn write_append_file(filepath: &PathBuf, text: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filepath)?;

    writeln!(file, "{text}")?;

    Ok(())
}

pub fn read_lines(filepath: &PathBuf) -> io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut lines_vec = Vec::new();

    for line in reader.lines() {
        lines_vec.push(line?);
    }

    Ok(lines_vec)
}

pub fn classify_week(
    date: DateTime<Local>,
) -> String {

    let now = Local::now();

    let current_week = now.iso_week();

    let last_week =
        (now - Duration::weeks(1)).iso_week();

    let target = date.iso_week();

    let (last_year, last_month) = if now.month() == 1 {
        (now.year() - 1, 12)
    } else {
        (now.year(), now.month() - 1)
    };

    if target.year() == current_week.year()
        && target.week() == current_week.week()
    {
        "今週".to_string()
    } else if target.year() == last_week.year()
        && target.week() == last_week.week()
    {
        "先週".to_string()
    } else if date.year() == now.year() 
        && date.month() == now.month()
    {
        "今月".to_string()
    } else if date.year() == last_year
        && date.month() == last_month 
    {
        "先月".to_string()
    } else {
        "それ以前".to_string()
    }
}

pub fn total_sleep_seconds(
    filepath: &PathBuf
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(filepath)?;

    let mut total = 0;

    for record in rdr.records() {
        let record = record?;
        
        let seconds: u64 =
            record[3].parse()?;

        total += seconds;
    }

    Ok(total)        
}

pub fn weekly_total(
    filepath: &PathBuf,
    target: &str
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(filepath)?;

    let mut total = 0;

    for record in rdr.records() {
        let record = record?;

        if record[0] == target.to_string() {
            total += record[3].parse::<u64>()?;
        }
    }

    Ok(total)
}
