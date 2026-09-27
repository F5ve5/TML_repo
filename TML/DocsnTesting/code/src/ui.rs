use crate::{ui_preproc::UiLoad,ui_postproc::{MergeOpcode,ULWithContentLabel}};

use std::{collections::{BTreeMap, HashMap}, sync::mpsc};
use eframe::egui;
use egui::Ui;

struct AppState{
    //Events
    shown_payload: UiLoad,

    request_tx: mpsc::Sender<MergeOpcode>,
    payload_rx: mpsc::Receiver<ULWithContentLabel>,
    current_request: Option<MergeOpcode>,
    ////

    //Diagram state
    diagram_cursor_pos: f64,
    diagram_zoom_x: f32,
    diagram_zoom_y: f32,
    diagram_zoom_z: f32,
    ////
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<MergeOpcode>,payload_rx: mpsc::Receiver<ULWithContentLabel>) -> Self{
        Self {
            shown_payload: UiLoad::default(),

            request_tx: request_tx,
            payload_rx: payload_rx,
            current_request: Some(MergeOpcode::default()),

            diagram_cursor_pos: 0.0,
            diagram_zoom_x: 1.0,
            diagram_zoom_y: 1.0,
            diagram_zoom_z: 1.0,
        }
    }

    fn request_payload(&mut self){
        if !self.current_request.is_none(){
            self.request_tx.send(self.current_request.clone().unwrap()).unwrap();

        }
    }

}


impl eframe::App for AppState {
    fn ui(&mut self,ctx: &mut Ui,_frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Hello World!");
            std::thread::sleep(core::time::Duration::from_millis(2000));

            if !self.current_request.is_none(){
                self.request_tx.send(self.current_request.clone().unwrap()).unwrap();
            }
        });
    }
}

pub fn start_egui(request_tx: mpsc::Sender<MergeOpcode>,payload_rx: mpsc::Receiver<ULWithContentLabel>){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AppState::new(_cc, request_tx, payload_rx)))
        }),
    ).unwrap();
}