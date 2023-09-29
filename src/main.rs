use eframe::egui;
use egui_plot::{Line, PlotPoints};
use tinyfiledialogs::{YesNo, MessageBoxIcon, DefaultColorValue, open_file_dialog};


const default_width: f32 = 1280.0;
const default_height: f32 = 720.0;
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

fn open_file_browser() -> Option<String> {
    let ayylmao = tinyfiledialogs::select_folder_dialog("Select_folder", "");
    ayylmao
} 

impl eframe::App for DaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Image viewer").default_width(default_width*(4.0/5.0)).show(ctx, |image_panel|{
            // let load_folder = image_panel.button("Load Folder");   
            // if load_folder.clicked() {
                
            //    let file_str = open_file_browser();
            //    let fukc = match &file_str {
            //         Some(yea) => println!("there's a file"),
            //         None => println!("no file selected!")
            //    };
            //    println!("{:?}", &file_str);
            // }    
            // // egui_plot::Plot::new("Image in question")
            // //     .allow_drag(true)
            // //     .allow_zoom(true)
            // //     .show(image_panel, |plot_ui| {
            // //         let mut image_viewer = image_panel.image(egui::include_image!("/home/hbpopos/Downloads/1.jpg"));
            // //     });
            // let load_file = image_panel.button("Load File");   
            // if load_file.clicked() {
                
            //    let file_str = open_file_dialog("selectImage", "image.png", None);
            // //    let fukc = match &file_str {
            // //         Some(yea) => println!("there's a file"),
            // //         None => println!("no file selected!")
            // //    };
            // //    println!("{:?}", file_str.unwrap());
            //     // let hehe = String::from(file_str.unwrap());
            //     image_panel.image(egui::include_image!("/home/hbpopos/Downloads/1.jpg"));
            // }    
            image_panel.image(egui::include_image!("/home/hbpopos/Downloads/1.jpg"));
        });
        egui::Window::new("Da Toolbar").show(ctx, |toolbar| {
            let draw_rect = toolbar.button("Draw Rect");
            let draw_pts = toolbar.button("Draw Point");
            let delete_label = toolbar.button("Edit Poly");
            let next_img = toolbar.button("Next Image");
            let prev_img = toolbar.button("Previous Image");
            
        });

        egui::Window::new("Da Info Panel").show(ctx, |ui| {
            ui.label("there is nothig here!");
        });


    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(default_width, default_height)),
        ..Default::default()
    };
    eframe::run_native("microlabel", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::<DaApp>::default()

    }))
}