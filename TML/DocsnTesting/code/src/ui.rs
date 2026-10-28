use crate::{ui_postproc::{self, UPOpcode, UiPayload}, ui_preproc::UiLoad};

use std::{collections::{BTreeMap,VecDeque}, sync::mpsc};

use eframe::egui::*;
use ntapi::ntrtl::PRTL_UNLOAD_EVENT_TRACE32;

struct AppState{
    green_flag: bool,
    lime_flag: bool,

    request_tx: mpsc::Sender<UPOpcode>,
    payload_rx: mpsc::Receiver<UiPayload>,
    current_requests: Vec<UPOpcode>,
    current_payloads: Vec<Option<UiPayload>>,

    initial_event_amount: i64,
    shown_length_map: BTreeMap<u64, i32>,
    shown_delta_map:BTreeMap<u64,i32>,
    frame_count_timeline_request: u16,

    update_slm: bool,
    update_sdm: bool,
    
    frame_delay_micros: std::time::Duration,
    frame_counter: u16,
    nano_tokens: u32,
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<UPOpcode>,payload_rx: mpsc::Receiver<UiPayload>) -> Self{
        Self{
            green_flag: false,
            lime_flag: false,

            request_tx: request_tx,
            payload_rx: payload_rx,
            current_requests: Vec::new(),
            current_payloads: vec![None;255],

            initial_event_amount: 0,
            shown_length_map: BTreeMap::new(),
            shown_delta_map: BTreeMap::new(),
            frame_count_timeline_request: 250,

            update_slm: false,
            update_sdm: false,

            frame_delay_micros: std::time::Duration::from_micros(16666),
            frame_counter: 10000,
            nano_tokens: 0,
        }
    }
}

impl AppState{
    pub fn pre_ui(&mut self){
        if self.green_flag{
            //INPUT

            //SCHEDULE
            if self.lime_flag{
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
        }

        //REQUEST SENDING
        if self.current_requests.len() > 0{
            for r in self.current_requests.drain(..){
                self.request_tx.send(r).expect("Request TX failed");
            }
        }

        //PAYLOAD HANDLING
        while let Ok(p) = self.payload_rx.try_recv(){
            let index = p.tag();
            self.current_payloads[index as usize] = Some(p);
        }

        if !self.green_flag{
            if let Some(UiPayload::iea(iea)) = self.current_payloads[1].take(){
                self.initial_event_amount = iea;
                self.green_flag = true;
            }
        }

        if let Some(UiPayload::length_map_c(lmc)) = self.current_payloads[5].take(){
            for x in lmc{
                *self.shown_length_map.entry(x.0).or_default() += x.1;
            }
            self.update_slm = true;
            self.lime_flag = true;
        }

        if let Some(UiPayload::delta_map_c(dmc)) = self.current_payloads[6].take(){
            for x in dmc{
                *self.shown_delta_map.entry(x.0).or_default() += x.1;
            }
            self.update_sdm = true;
        }
    }
    
    pub fn insert_lmc(&mut self,lmc_payload:&mut UiPayload){
        if let UiPayload::length_map_c(lmc) = lmc_payload{
            while let Some((time,delta)) = lmc.pop_first(){
                *self.shown_length_map.entry(time).or_default() += delta;
            }
        }
    }
}

impl eframe::App for AppState{
    fn ui(&mut self,ctx: &mut Ui,_frame: &mut eframe::Frame){
        let start_instant = std::time::Instant::now();

        if self.frame_counter > 10000{
            self.frame_counter = 1;
        }else{
            self.frame_counter += 1;
        }

        self.nano_tokens = 100_000;
        //////////
        CentralPanel::default().show(ctx, |ui|{

            let painter = ui.painter();
  
            let square = Rect::from_min_size(
                pos2(0.0, 0.0),
                Vec2::new(50.0, 50.0),
            );

            if !self.green_flag{
                painter.rect_filled(
                    square,
                    0.0,
                    Color32::RED,
                );
                
            }else{
                painter.rect_filled(
                    square,
                    0.0,
                    Color32::GREEN,
                );
                
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