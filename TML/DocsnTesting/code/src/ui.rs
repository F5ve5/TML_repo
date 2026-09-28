use crate::{ui_preproc::UiLoad,ui_postproc::{MergeOpcode,UiPayload}};

use std::{collections::{BTreeMap}, sync::mpsc};
use eframe::egui;
use egui::Ui;

struct AppState{
    //Events
    shown_payload: UiLoad,

    request_tx: mpsc::Sender<MergeOpcode>,
    payload_rx: mpsc::Receiver<UiPayload>,
    current_request: Option<MergeOpcode>,

    current_payload: Option<UiPayload>,
    ////
    
    //Dependent States
    length_map: Option<BTreeMap<u64,i32>>,
    ////

    //Diagram state
    diagram_cursor_pos: f64,
    diagram_zoom_x: f32,
    diagram_zoom_y: f32,
    diagram_zoom_z: f32,
    ////
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<MergeOpcode>,payload_rx: mpsc::Receiver<UiPayload>) -> Self{
        Self {
            shown_payload: UiLoad::default(),

            request_tx: request_tx,
            payload_rx: payload_rx,
            current_request: None,

            current_payload: None,

            length_map: None,

            diagram_cursor_pos: 0.0,
            diagram_zoom_x: 1.0,
            diagram_zoom_y: 1.0,
            diagram_zoom_z: 1.0,
        }
    } 
}


impl eframe::App for AppState {
    fn ui(&mut self,ctx: &mut Ui,_frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            if let Ok(payload) = self.payload_rx.try_recv(){
                self.current_payload = Some(payload);
            }

            if let Some(UiPayload::length_map(lm)) = &self.current_payload{
                
            }
            //Update dependent UI...

            //Change request according to user input...
            
            if !self.current_request.is_none(){
                self.request_tx.send(self.current_request.clone().unwrap()).unwrap();
            }
        });
    }
}

pub fn start_egui(request_tx: mpsc::Sender<MergeOpcode>,payload_rx: mpsc::Receiver<UiPayload>){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AppState::new(_cc, request_tx, payload_rx)))
        }),
    ).unwrap();
}