#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Tephra",
        options,
        Box::new(|_cc| Ok(Box::<Tephra>::default())),
    )
}

struct Tephra {
    side_panel_active: bool,
}

impl Default for Tephra {
    fn default() -> Self {
        Self {
            side_panel_active: true,
        }
    }
}

impl Tephra {
    fn ui_side_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if ui.button("Close side panel").clicked() {
                self.side_panel_active = false;
            }
        });
    }
}

impl eframe::App for Tephra {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Search Bar").show(&ctx, |ui| ui_search_bar(ui));
        if self.side_panel_active {
            egui::CentralPanel::default().show(&ctx, |ui| self.ui_side_bar(ui));
        }
        egui::CentralPanel::default().show(&ctx, |ui| ui_main_window(ui));
    }
}

fn ui_main_window(ui: &mut egui::Ui) {
    ui.hyperlink("https://github.com/HenrySteinmetz/Tephra");
}

fn ui_search_bar(ui: &mut egui::Ui) {
    let mut text = String::new();
    ui.vertical_centered(|ui| {
        ui.text_edit_singleline(&mut text);
        ui.separator();
    });
}
