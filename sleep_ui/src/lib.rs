use chrono::{DateTime, Local};
use eframe::egui;
use sleep_toul;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

#[derive(Default)]
pub struct MyApp {
    pub last_activity: Arc<AtomicU64>,

    pub idle: u64,
    pub clock: DateTime<Local>,
    pub nowclock: DateTime<Local>,
    pub clockfrg: bool,
    pub sleepidle: u64,

    pub msgfrg: bool,
    pub textfrg: bool,

    pub clockcp: DateTime<Local>,
    pub nowclockcp: DateTime<Local>,
    pub sleepidlecp: u64,
}

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.idle = sleep_toul::now() - self.last_activity.load(Ordering::Relaxed);
        // let nowtime = SystemTime::now();

        self.clock = sleep_toul::sleepclock(self.idle, self.clock);
        self.nowclock = sleep_toul::sleepclock(self.idle, self.clock);

        if self.idle >= 300 {
            self.textfrg = true;
            self.msgfrg = false;
            self.nowclockcp = self.nowclock;
        }

        if self.idle < 1 && self.clockfrg == true {
            self.msgfrg = true;
            self.clockcp = self.clock;
            self.sleepidlecp = self.sleepidle;
        }

        egui::Panel::top("header")
            .default_size(100.0)
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(116, 145, 195)))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Sleep_Checker")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                );
            });

        let panel_frame = egui::Frame::default().fill(egui::Color32::from_rgb(76, 102, 148)); // 任意の色

        let local_app_data = std::env::var("LOCALAPPDATA").expect("LOCALAPPDATAが取得できません");

        let filepath = PathBuf::from(local_app_data)
            .join("Sleep_Checker_csv")
            .join("sleep_data.csv");

        egui::CentralPanel::default()
            .frame(panel_frame) // 作成したフレームを適用
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("現在状態: ")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(209, 209, 209)),
                    );

                    if self.idle <= 1 {
                        ui.label(
                            egui::RichText::new("稼働中")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(255, 143, 0)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("停止中")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(90, 190, 163)),
                        );
                    }

                    ui.label(
                        egui::RichText::new("\n ")
                            .size(10.0)
                            .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                });
                if self.msgfrg == true {
                    ui.label(
                        egui::RichText::new(format!(
                            "スリープ状態を検知開始した日時: {}",
                            self.nowclockcp.format("%Y-%m-%d %H:%M:%S")
                        ))
                        .size(15.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "スリープ状態が解除された日時: {}",
                            self.clockcp.format("%Y-%m-%d %H:%M:%S")
                        ))
                        .size(15.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "スリープまたは停止時間: {}分{}秒",
                            self.sleepidlecp / 60,
                            self.sleepidlecp % 60
                        ))
                        .size(15.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );

                    if self.textfrg == true {
                        let category = sleep_toul::classify_week(self.clockcp);

                        let text = format!(
                            "{},{},{},{}",
                            category,
                            self.nowclockcp.format("%Y-%m-%d %H:%M:%S"),
                            self.clockcp.format("%Y-%m-%d %H:%M:%S"),
                            self.sleepidlecp
                        );

                        if let Err(e) = sleep_toul::write_append_file(&filepath, text) {
                            println!("Error: {e}");
                        }

                        self.textfrg = false;
                    }
                } else {
                    ui.label(
                        egui::RichText::new(format!(
                            "スリープ状態を検知開始した日時:"
                        ))
                        .size(15.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "スリープ状態が解除された日時:"
                        ))
                        .size(15.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "スリープまたは停止時間:"
                        ))
                        .size(15.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                }

                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(60, 80, 120))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("履歴").color(egui::Color32::from_rgb(209, 209, 209))
                        );

                        ui.separator();

                        egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .show(ui, |ui| {
                                if let Ok(lines) = sleep_toul::read_lines(&filepath) {
                                    for line in &lines {
                                        ui.label(
                                            egui::RichText::new(line).color(egui::Color32::from_rgb(209, 209, 209)),
                                        );
                                    }
                                }
                            });
                    });

                if let Ok(text) = sleep_toul::weekly_total(&filepath, "今週") {
                    ui.label(
                        egui::RichText::new(format!(
                            "今週の停止時間: {}時間{}分{}秒",
                            text / 3600,
                            (text % 3600) / 60,
                            text % 60
                        ))
                        .size(20.0)
                        .color(egui::Color32::from_rgb(209, 209, 209)),
                    );
                }
            });

        /* egui::Frame::default()
            .fill(egui::Color32::from_rgb(60, 80, 120))
            .show(ui, |ui| {
     
                if let Ok(lines) = sleep_toul::read_lines(&filepath) {
                    for line in &lines {
                        ui.label(
                            egui::RichText::new(line).color(egui::Color32::from_rgb(209, 209, 209)),
                        );
                    }
                }
            }); */

        self.clockfrg = sleep_toul::sleepfrg(self.idle);
        self.sleepidle = self.idle;
    }
}
