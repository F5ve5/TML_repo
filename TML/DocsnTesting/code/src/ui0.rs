struct MyApp{
    
}

impl eframe::App for MyApp{

        fn update(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {

    }
}

pub fn start_egui(){
    
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ProcessGaze",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {}))
        }),
    ).unwrap();

}