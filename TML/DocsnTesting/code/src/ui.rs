use crate::{misc, ui_postproc::{self, UPOpcode, UiPayload}};

use std::{collections::BTreeMap, ops::Sub, sync::mpsc};

use eframe::egui;
use egui_plot::{self, BarChart};

use std::ops::RangeInclusive;

use eframe::egui::{Color32, Pos2, Response, Shape, Ui};

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
    first: u64
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<UPOpcode>,payload_rx: mpsc::Receiver<UiPayload>) -> Self{

        let mut bars = vec![ egui_plot::Bar::new(0.0, 0.0)];
        bars.clear();

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
            frame_count_timeline_request: 260,

            bars: Vec::new(),
            first: 0
        }
    }
}

impl AppState{
    pub fn pre_ui(&mut self){
        //REQUEST SCHEDULING
        if self.green_flag && self.lime_flag{
            if self.frame_counter % self.frame_count_timeline_request == 0{
                self.current_requests.push(ui_postproc::UPOpcode{
                    xI: 4,
                    xII: 0,
                    xIII: 0,
                });
            }
        }

        //REQUEST SENDING
        if self.current_requests.len() > 0{
            for r in self.current_requests.drain(..){
                self.request_tx.send(r).expect("Request TX failed");
            }
        }

        //PAYLOAD RECEIVING
        while let Ok(p) = self.payload_rx.try_recv(){
            let index = p.tag();
            self.current_payloads[index as usize] = Some(p);
        }

        //PAYLOAD HANDLING

        for p0 in self.current_payloads.iter().enumerate(){
            if let Some(p) = self.current_payloads[p0.0].as_ref(){

                //println!("Payload Received: {:?}", p);
            }
        };

        if let Some(UiPayload::iea(iea)) = self.current_payloads[1].take(){
            self.initial_event_amount = iea;
            self.green_flag = true;
        }


        if let Some(UiPayload::length_map(mut lm)) = self.current_payloads[4].take(){
            if self.first == 0 && let Some(kv) = lm.first_key_value(){
                self.first = kv.0.clone();
            }
            for kv in lm{
                *self.shown_length_map.entry((kv.0 - self.first)/10_000_000).or_default() = kv.1;
            }

            self.bars = self.shown_length_map
                .values()
                .enumerate()
                .map(|(i, &length)| {
                    egui_plot::Bar::new(i as f64, length as f64)
                        .width(0.75)
                        .fill(Color32::LIGHT_BLUE)
                })
                .collect();

            self.lime_flag = true;
        }

        if let Some(UiPayload::length_map_c(mut lmc)) = self.current_payloads[6].take(){
            self.insert_lmc(&mut lmc);
        }


        if let Some(UiPayload::delta_map_c(mut dmc)) = self.current_payloads[5].take(){
            self.insert_dmc(&mut dmc);

        }

        //INIMMEDIATE PAYLOAD LOGIC

    }
    
    pub fn insert_lmc(&mut self,lmc_payload:&mut BTreeMap<u64,i32>){

        self.shown_length_map.append(lmc_payload);

    }

    pub fn insert_dmc(&mut self,dmc_payload:&mut BTreeMap<u64,i32>){

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

            if self.green_flag && self.lime_flag{
                egui_plot::Plot::new("length_graph")
                    .height(self.initial_event_amount as f32 + 300f32)
                    .grid_color(Color32::DARK_BLUE)
                    .show(ui, |plot_ui| {
                        plot_ui.bar_chart(BarChart::new("my bc",self.bars.clone()));   

                        if plot_ui.response().clicked(){
                            if let Some(pointer) = plot_ui.pointer_coordinate(){
                                self.current_requests.push(UPOpcode { xI: 2, xII: pointer.x as u64 * 10_000_000 + self.first, xIII: 0 })
                            }
                        }    
                    });
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