use crate::{biner,ui_preproc::UiLoad,ui_postproc::MergeOpcode};

use std::{collections::{BTreeMap, HashMap}, sync::mpsc};
use eframe::egui;
use egui::Ui;

struct AppState{
    //Events
    shown_payload: UiLoad,
    request_tx: mpsc::Sender<MergeOpcode>,
    data_rx: mpsc::Receiver<UiLoad>,
    ////

    //Diagram state
    diagram_cursor_pos: f64,
    diagram_zoom_x: f32,
    diagram_zoom_y: f32,
    diagram_zoom_z: f32,
    ////

    //Event Viewer state
    written_event_selected: u32,
    ////
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<MergeOpcode>,data_rx: mpsc::Receiver<UiLoad>) -> Self{
        Self {
            shown_payload: UiLoad::default(),
            request_tx: request_tx,
            data_rx: data_rx,

            diagram_cursor_pos: 0.0,
            diagram_zoom_x: 1.0,
            diagram_zoom_y: 1.0,
            diagram_zoom_z: 1.0,

            written_event_selected: 0,

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

pub fn start_egui(request_tx: mpsc::Sender<MergeOpcode>,data_rx: mpsc::Receiver<UiLoad>){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AppState::new(_cc, request_tx, data_rx)))
        }),
    ).unwrap();
}