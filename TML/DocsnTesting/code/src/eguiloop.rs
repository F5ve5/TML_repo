use crate::biner;

use eframe::egui;
use egui::Ui;

struct AppState{
    events: Vec<biner::EventFinal>,
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            events: Vec<biner::EventFinal>,
        }
    }
}

impl eframe::App for AppState {
    fn ui(&mut self,ctx: &mut Ui,_frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Hello World!");

        });
    }
}

pub fn start_egui(){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(|_cc| {
            Ok(Box::new(AppState::new(_cc)))
        }),
    ).unwrap();
}
