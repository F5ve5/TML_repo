use crate::{ui_postproc::{self, UPOpcode, UiPayload}};

use std::{collections::BTreeMap, ops::Sub, sync::mpsc};

use eframe::egui;
use egui_plot;

struct AppState{
    backmost_layer: u8,
    //0 = main

    green_flag: bool,
    lime_flag: bool,

    request_tx: mpsc::Sender<UPOpcode>,
    payload_rx: mpsc::Receiver<UiPayload>,
    current_requests: Vec<UPOpcode>,
    current_payloads: Vec<Option<UiPayload>>,

    initial_event_amount: i32,
    shown_length_map: BTreeMap<u64, i32>,
    shown_delta_map:BTreeMap<u64,i32>,

    redraw_slm: bool,
    redraw_sdm: bool,
    
    nano_tokens: u32,
    frame_delay_micros: std::time::Duration,
    frame_counter: u16,
    frame_count_timeline_request: u16,

    bars: Vec<egui_plot::Bar>,
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<UPOpcode>,payload_rx: mpsc::Receiver<UiPayload>) -> Self{
        Self{
            backmost_layer: 0,

            green_flag: false,
            lime_flag: false,

            request_tx: request_tx,
            payload_rx: payload_rx,
            current_requests: Vec::new(),
            current_payloads: vec![None;255],

            initial_event_amount: 0,
            shown_length_map: BTreeMap::new(),
            shown_delta_map: BTreeMap::new(),

            redraw_slm: false,
            redraw_sdm: false,

            nano_tokens: 0,
            frame_delay_micros: std::time::Duration::from_micros(16666),
            frame_counter: 10000,
            frame_count_timeline_request: 300,

            bars: Vec::new(),
        }
    }
}

impl AppState{
    pub fn pre_ui(&mut self){
        //REQUEST SCHEDULES
        if self.green_flag && self.lime_flag{
            if self.frame_counter % self.frame_count_timeline_request == 0{
                self.current_requests.push(ui_postproc::UPOpcode{
                    xI: 5,
                    xII: 0,
                    xIII: 0,
                });

                self.current_requests.push(ui_postproc::UPOpcode{
                    xI: 6,
                    xII: 0,
                    xIII: 0,
                });
            }
        }

        //REQUEST SENDING
        if self.current_requests.len() > 0{
            for r in self.current_requests.drain(..){
                println!("sending request {:?}", r);
                self.request_tx.send(r).expect("Request TX failed");
            }
        }

        //PAYLOAD RECEIVING
        while let Ok(p) = self.payload_rx.try_recv(){
            let index = p.tag();
            self.current_payloads[index as usize] = Some(p);
        }

        //PAYLOAD HANDLING

        for x in self.current_payloads.iter().enumerate(){
            if !self.current_payloads[x.0].is_none(){
            }
        }

        if let Some(UiPayload::iea(iea)) = self.current_payloads[1].take(){
            self.initial_event_amount = iea;
            self.green_flag = true;
        }


        if let Some(UiPayload::length_map(lm)) = self.current_payloads[4].take(){
            self.shown_length_map = lm;
            self.lime_flag = true;
        }

        if let Some(UiPayload::length_map_c(mut lmc)) = self.current_payloads[5].take(){
            self.insert_lmc(&mut lmc);
            println!("lmc handled");
            
            self.redraw_slm;
        }


        if let Some(UiPayload::delta_map_c(mut dmc)) = self.current_payloads[6].take(){
            self.insert_dmc(&mut dmc);
            
            self.redraw_sdm;
        }

        //INIMMEDIATE PAYLOAD LOGIC
        if self.redraw_slm{

        }
    }
    
    pub fn insert_lmc(&mut self,lmc_payload:&mut BTreeMap<u64,i32>){

      
            self.bars.extend(lmc_payload
                .iter()
                .map(|(time, delta)| {
                    egui_plot::Bar::new(*time as f64, *delta as f64)
                })
            );


    }

    pub fn insert_dmc(&mut self,dmc_payload:&mut BTreeMap<u64,i32>){

        while let Some((time,delta)) = dmc_payload.pop_first(){
            *self.shown_delta_map.entry(time).or_default() += delta;
        }

    }
}

impl eframe::App for AppState{
    fn ui(&mut self,ctx: &mut egui::Ui,_frame: &mut eframe::Frame){
        let start_instant = std::time::Instant::now();

        if self.frame_counter > 10000{
            self.frame_counter = 1;
        }else{
            self.frame_counter += 1;
        }

        self.nano_tokens = 100_000;
        //////////
        
        self.pre_ui();
        
        egui::CentralPanel::default().show(ctx, |ui|{

            let painter = ui.painter();

            let window_size = ui.max_rect();
            let top = window_size.min.y;
            let left = window_size.min.x;
            let w = window_size.width();
            let h = window_size.height();

            if self.redraw_slm{
                egui_plot::Plot::new("length_graph")
                    .height(600.0)
                    .show(ui, |plot_ui| {
                    plot_ui.bar_chart(
                        egui_plot::BarChart::new("Lengths", self.bars.clone())
                    );
                });
                println!("graph drawn");
            }
        

            if !self.green_flag{

                
            }else{

                
            }
        });

        //////////
        let end_elapsed = start_instant.elapsed();
        if end_elapsed >= self.frame_delay_micros{
            ctx.request_repaint();
        }else{
            ctx.request_repaint_after(self.frame_delay_micros - end_elapsed);
        }
    }
}
pub fn start_egui(request_tx: mpsc::Sender<UPOpcode>,payload_rx: mpsc::Receiver<UiPayload>){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AppState::new(_cc, request_tx, payload_rx)))
        }),
    ).unwrap();
}