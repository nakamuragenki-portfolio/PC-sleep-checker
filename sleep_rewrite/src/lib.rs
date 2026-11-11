use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use std::path::PathBuf;
use csv::{Reader, Writer};
use sleep_toul;

#[derive(Clone)]
pub struct SleepRecord {
    pub category: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub sleep_time: u64,
}

pub fn load_csv(
    filepath: &PathBuf,
) -> Result<Vec<SleepRecord>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(filepath)?;

    let mut records = Vec::new();

    for row in reader.records() {
        let row = row?;

        let start_naive = NaiveDateTime::parse_from_str(
            &row[1],
            "%Y-%m-%d %H:%M:%S"
        )?;
        
        let end_naive = NaiveDateTime::parse_from_str(
            &row[2],
            "%Y-%m-%d %H:%M:%S"
        )?;
        
        let start =
            Local.from_local_datetime(&start_naive)
                .single()
                .unwrap();
        
        let end =
            Local.from_local_datetime(&end_naive)
                .single()
                .unwrap();
        
        records.push(SleepRecord {
            category: row[0].to_string(),
            start,
            end,
            sleep_time: row[3].parse()?,
        });
    }

    Ok(records)
}

pub fn update_categories(
    records: &mut Vec<SleepRecord>
) {
    {
        update_sub(records);
    }

    records.retain(|record| {
        record.category != "それ以前"
    });
}

fn update_sub(
    records: &mut Vec<SleepRecord>
) {
    for record in records {
        record.category =
            sleep_toul::classify_week(record.start);
    }
}

pub fn rewrite_csv(
    filepath: &PathBuf,
    records: &[SleepRecord],
    ) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut writer =
        Writer::from_path(filepath)?;
    
    writer.write_record([
        "分類",
        "開始日時",
        "終了日時",
        "停止時間(秒)",
    ])?;
    
    for record in records {
    
    writer.write_record([
        &record.category,
        &record.start
        .format("%Y-%m-%d %H:%M:%S")
        .to_string(),
    
        &record.end
        .format("%Y-%m-%d %H:%M:%S")
        .to_string(),
    
        &record.sleep_time.to_string(),
    ])?;
    }
    
    writer.flush()?;
    
    Ok(())
}