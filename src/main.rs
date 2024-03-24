use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::RwLock;

use anyhow::Result;
use clap::Parser;
use eframe::egui::accesskit::Vec2;
use spm_rs::igor_ibw::read_ibw;
use spm_rs::mulfile::{read_mul, MulImage};

// use spm_rs::rhk_sm4::read_rhk_sm4;

use eframe::egui;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg()]
    filename: String,
}

// fn main() -> Result<()> {
//     let args = Args::parse();
//     if args.filename.ends_with(".mul") || args.filename.ends_with(".flm") {
//         let mulfile = read_mul(&args.filename)?;
//         // let b = load_from_memory(mulfile[0].img_data.img_data.as_slice());
//         println!("{:?}", mulfile[0].img_data.to_png_bytes());

//         // for mut i in mulfile {
//         //     i.img_data.correct_plane();
//         //     i.img_data.correct_lines();
//         //     i.img_data.save_png();
//         // }
//     } else if args.filename.ends_with(".Z_mtrx") {
//         let mut omicron_matrix = read_omicron_matrix(&args.filename)?;
//         omicron_matrix.img_data_fw.correct_plane();
//         omicron_matrix.img_data_bw.correct_lines();
//         // omicron_matrix.img_data_fw.save_png();
//         // omicron_matrix.img_data_bw.save_png();
//     } else if args.filename.ends_with(".ibw") {
//         let wave = read_ibw(&args.filename)?;
//         dbg!(wave);
//         // println!("{}", wave.data[0]);
//         // println!("{:?}", wave.bname)
//     // } else if args.filename.to_lowercase().ends_with(".sm4") {
//         // let rhk_sm4 = read_rhk_sm4(&args.filename)?;
//         // dbg!(rhk_sm4);
//     }

//     Ok(())
// }

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(cc))
        }),
    )
}

struct GuiImage {
    img: MulImage,
    png: Vec<u8>,
}

struct MyApp {
    images: Vec<GuiImage>,
    active_images: HashMap<String, bool>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let args = Args::parse();

        if args.filename.ends_with(".mul") || args.filename.ends_with(".flm") {
            let mulfile = read_mul(&args.filename).unwrap();

            let active_images = mulfile
                .iter()
                .map(|img| (img.img_id.clone(), false))
                .collect();

            let gui_images = mulfile
                .into_iter()
                .map(|mut img| {
                    img.img_data.correct_plane();
                    img.img_data.correct_lines();
                    let png = img.img_data.to_png_bytes();
                    GuiImage { img, png }
                })
                .collect();

            Self {
                images: gui_images,
                active_images,
            }
        } else {
            Self {
                images: Vec::new(),
                active_images: HashMap::new(),
            }
        }
    }

    fn main_window(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| self.grid_view(ctx, ui));
        });
    }

    fn grid_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Grid::new("grid")
            .spacing(egui::vec2(5.0, 5.0))
            .show(ui, |ui| {
                for (i, img) in self.images.iter().enumerate() {
                    if (i + 1) % 5 == 0 {
                        ui.end_row();
                    }

                    let sense = egui::Sense {
                        click: true,
                        drag: false,
                        focusable: false,
                    };
                    let image_clone =
                        egui::Image::from_bytes(img.img.img_id.clone(), img.png.clone())
                            .sense(sense)
                            .fit_to_exact_size(egui::Vec2 { x: 200., y: 200. });
                    let response = ui.add(image_clone);
                    if response.double_clicked() {
                        if let Some(entry) = self.active_images.get_mut(&img.img.img_id) {
                            *entry = true;
                        };
                        println!("clicked {:?}", response);
                    }
                    if self.active_images.get(&img.img.img_id).is_some_and(|&x| x) {
                        let new_viewport_id = egui::ViewportId::from_hash_of(&img.img.img_id);
                        let new_viewport = egui::ViewportBuilder::default()
                            .with_title(&img.img.img_id)
                            .with_inner_size(egui::Vec2 {
                                x: img.img.xres as f32,
                                y: img.img.yres as f32,
                            });
                        ctx.show_viewport_immediate(new_viewport_id, new_viewport, |ctx, class| {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                let image_clone = egui::Image::from_bytes(
                                    img.img.img_id.clone(),
                                    img.png.clone(),
                                );
                                ui.add(image_clone);
                            });

                            if ctx.input(|i| i.viewport().close_requested()) {
                                // Tell parent viewport that we should not show next frame:
                                if let Some(entry) = self.active_images.get_mut(&img.img.img_id) {
                                    *entry = false;
                                };
                            }
                        });
                    };
                }
            });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_window(ctx);
    }
}
