#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use rdev::{EventType, listen};
use chrono::Local;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use eframe::egui;
use std::fs;
use std::path::PathBuf;
use sleep_ui::MyApp;
use sleep_toul;
use sleep_rewrite;


fn get_dir() -> PathBuf {
    let local_app_data =
        std::env::var("LOCALAPPDATA")
            .expect("LOCALAPPDATAが取得できません");

    let dir = PathBuf::from(local_app_data)
        .join("Sleep_Checker_csv");

    dir.to_path_buf()
}

/* fn check_folder_exists() -> bool {
    let local_app_data =
        std::env::var("LOCALAPPDATA")
            .expect("LOCALAPPDATAが取得できません");

    let folder_path = PathBuf::from(local_app_data)
        .join("Sleep_Checker_csv");

    folder_path.exists()
} */

fn main() -> Result<(), eframe::Error> {
    let dir = get_dir();

    if dir.exists() && dir.is_dir() {
        println!("記録用ディレクトリは存在します。");

        let filepath = PathBuf::from(dir)
            .join("sleep_data.csv");

        if filepath.exists() {
            println!("記録用ファイルは存在します。");
        } else {
            let write_text = "分類,開始日時,終了日時,停止時間(秒)";

            if let Err(e) = sleep_toul::write_new_file(&filepath, write_text) {
                println!("Error: {e}");
            }
        }

    } else {
        println!("記録用ディレクトリは存在しません。");

        fs::create_dir(&dir)
            .expect("記録用ディレクトリ作成失敗");

        let filepath = PathBuf::from(dir)
            .join("sleep_data.csv");

        let write_text = "分類,開始日時,終了日時,停止時間(秒)";

        if let Err(e) = sleep_toul::write_new_file(&filepath, write_text) {
            println!("Error: {e}");
        }

    }

    //ファイル更新
    let filepath = PathBuf::from(get_dir())
        .join("sleep_data.csv");
    
    if let Ok(mut records) = sleep_rewrite::load_csv(&filepath) {
    
        sleep_rewrite::update_categories(&mut records);
    
        if let Err(err) =
            sleep_rewrite::rewrite_csv(&filepath, &records)
        {
            eprintln!(
                "CSV更新失敗: {}",
                err
            );
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Sleep_Checker")
            .with_icon(sleep_toul::load_icon())
            .with_inner_size([500.0, 400.0]),
            
        ..Default::default()
    };
    
    eframe::run_native(
        "Sleep_Checker",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);

            let last_activity = Arc::new(AtomicU64::new(sleep_toul::now()));

            {
                let activity = last_activity.clone();

                thread::spawn(move || {
                    let callback = move |event: rdev::Event| {
                        match event.event_type {
                            EventType::KeyPress(_)
                            | EventType::MouseMove { .. }
                            | EventType::ButtonPress(_)
                            | EventType::Wheel { .. } => {
                                activity.store(sleep_toul::now(),Ordering::Relaxed);
                            }
                            _ => {}
                        }
                    };

                    if let Err(e) = listen(callback) {
                        eprintln!("listen error: {:?}", e);
                    }
                });
            }
        
            Ok(Box::new(MyApp {
                last_activity,

                idle: 0,
                clock: Local::now(),
                nowclock: Local::now(),
                clockfrg: false,
                sleepidle: 0,

                msgfrg: false,
                textfrg: false,

                clockcp: Local::now(),
                nowclockcp: Local::now(),
                sleepidlecp: 0,
            }))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "japanese".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "NotoSansJP-Regular.ttf"
        ))
        .into(),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "japanese".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("japanese".to_owned());

    ctx.set_fonts(fonts);
}