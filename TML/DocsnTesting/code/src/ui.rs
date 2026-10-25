use crate::{ui_postproc::{self, UPOpcode, UiPayload}, ui_preproc::UiLoad};

use std::{collections::{BTreeMap,VecDeque}, sync::mpsc, {mem::replace}};
use egui::{Color32, Rect, Vec2};
use eframe::egui;
use egui::Ui;

struct AppState{
    startup_rx: mpsc::Receiver<i64>,
    srx_green_flag: bool,
    initial_event_amount: i64,
    lime_flag: bool,

    request_tx: mpsc::Sender<UPOpcode>,
    payload_rx: mpsc::Receiver<UiPayload>,
    current_requests: Vec<UPOpcode>,
    current_payloads: Vec<Option<UiPayload>>,

    shown_length_map: BTreeMap<u64, i32>,
    fcs_lmc: u16,

    update_slm: bool,
    
    frame_delay_micros: std::time::Duration,
    frame_counter: u16,
}

impl AppState {
    fn new(_cc: &eframe::CreationContext<'_>,request_tx: mpsc::Sender<UPOpcode>,payload_rx: mpsc::Receiver<UiPayload>,startup_rx: mpsc::Receiver<i64>) -> Self{
        Self{
            startup_rx: startup_rx,
            srx_green_flag: false,
            initial_event_amount: 0,
            lime_flag: false,

            request_tx: request_tx,
            payload_rx: payload_rx,
            current_requests: Vec::new(),
            current_payloads: vec![None;255],

            shown_length_map: BTreeMap::new(),
            fcs_lmc: 250, //frame counter schedule - shown length map

            update_slm: false,

            frame_delay_micros: std::time::Duration::from_micros(16666),
            frame_counter: 10000
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

        let tokens: i64 = 10000;
        //////////

        if !self.srx_green_flag{
            if let Ok(iea) = self.startup_rx.try_recv(){
                self.initial_event_amount = iea;
                self.srx_green_flag = true;
            }
        }else{
            //INPUT

            ////
            ///
            //SCHEDULE
            if self.frame_counter % self.fcs_lmc == 0{
                self.current_requests.push(ui_postproc::UPOpcode{
                    xI: 5,
                    xII: 0,
                    xIII: 0,
                });
            }
            ////
            /// 
            //OTHER

            ////
            
            //REQUESTS
            if self.current_requests.len() > 0{
                for r in self.current_requests.drain(..){
                    self.request_tx.send(r).expect("Request TX failed");
                }
            }
            while tokens > 0{

            }
            ////
            /// 
            //PAYLOADS
            while let Ok(p) = self.payload_rx.try_recv(){
                let index = p.tag();
                self.current_payloads[index as usize] = Some(p);
            }

            if let Some(UiPayload::length_map_c(lmc)) = self.current_payloads[5].take(){
                for x in lmc{
                    *self.shown_length_map.entry(x.0).or_default() += x.1;
                }
                self.update_slm = true;
            }
            ////
            ////

        }

        egui::CentralPanel::default().show(ctx, |ui|{

            let painter = ui.painter();

            let square = Rect::from_min_size(
                egui::pos2(100.0, 100.0),
                Vec2::new(50.0, 50.0),
            );

            if !self.srx_green_flag{

                painter.rect_filled(
                    square,
                    0.0,
                    Color32::RED,
                );
                
            }else{

                painter.rect_filled(
                    square,
                    0.0,
                    Color32::BLUE,
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

pub fn start_egui(request_tx: mpsc::Sender<UPOpcode>,payload_rx: mpsc::Receiver<UiPayload>,startup_rx: mpsc::Receiver<i64>){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(AppState::new(_cc, request_tx, payload_rx, startup_rx)))
        }),
    ).unwrap();
}