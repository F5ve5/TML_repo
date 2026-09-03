use crate::biner;

use std::{collections::{BTreeMap, HashMap}, sync::mpsc};
use eframe::egui;
use egui::Ui;

struct AppState{
    //Events
    received_events: Vec<biner::EventFinal>,

    total_events: HashMap<u32, biner::EventFinal>,

    pids_at_given_time: BTreeMap<u64, Vec<u32>>,
    pids_by_start: BTreeMap<u64, Vec<u32>>,
    pids_by_end: BTreeMap<u64, Vec<u32>>,

    event_receiver_rx: std::sync::mpsc::Receiver<Vec<biner::EventFinal>>,
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
    
    //Other
    current_bucket: u64,
    ////
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,rx: mpsc::Receiver<Vec<biner::EventFinal>>,) -> Self{
        Self {
            received_events: Vec::new(),

            total_events: HashMap::new(),

            pids_at_given_time: BTreeMap::new(),

            pids_by_start: BTreeMap::new(),
            pids_by_end: BTreeMap::new(),

            event_receiver_rx: rx,

            diagram_cursor_pos: 0.0,
            diagram_zoom_x: 1.0,
            diagram_zoom_y: 1.0,
            diagram_zoom_z: 1.0,

            written_event_selected: 0,

            current_bucket: 0,
        }
    }

    fn process_events_loop(&mut self) {
        while let Ok(event_vec) = self.event_receiver_rx.try_recv() {
            for event in event_vec {
                match event.opcode {
                    1 => self.mark_start(event),
                    2 => self.mark_end(event.process_id,event.timestamp),
                    _ => {}
                }
            }
        }
    }

    fn mark_start(&mut self,e:biner::EventFinal) {    
    self.pids_by_start
        .entry(e.timestamp)
        .or_default()
        .push(e.process_id);
    }

    fn mark_end(&mut self,pid:u32,t:u64) {    
    self.pids_by_end
        .entry(t)
        .or_default()
        .push(pid);
    }

    fn fill_buckets_loop(&mut self,loop_lim:u64){
        if let Some((k, _)) = self.pids_by_end.first_key_value() && self.current_bucket == 0{
            self.current_bucket = *k;
        }

        let bucketeer = Vec<u32> = Vec::new();
    }
}


impl eframe::App for AppState {
    fn ui(&mut self,ctx: &mut Ui,_frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Hello World!");

        });
    }
}

pub fn start_egui(event_receiver_rx: mpsc::Receiver<Vec<biner::EventFinal>>){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AppState::new(_cc, event_receiver_rx)))
        }),
    ).unwrap();
}