use eframe::egui;


struct DaApp {
    name: String,
    version: f32
}

impl Default for DaApp {
    fn default() -> Self {
        Self {
            name: "microlabel".to_owned(),
            version: 1.0
        }
    }
}

impl eframe::App for DaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mircolabel");
        });
        egui::SidePanel::left("Da Toolbar").show(ctx, |toolbar| {
            toolbar.button("rect1")
        });

    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native("microlabel", options, Box::new(|cc| {
        Box::<DaApp>::default()
    }))
}