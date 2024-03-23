use anyhow::Result;
use clap::Parser;
use spm_rs::igor_ibw::read_ibw;
use spm_rs::mulfile::read_mul;

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

struct MyApp<'a> {
    images: Vec<egui::Image<'a>>,
}

impl<'a> MyApp<'_> {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let args = Args::parse();

        if args.filename.ends_with(".mul") || args.filename.ends_with(".flm") {
            let mut mulfile = read_mul(&args.filename).unwrap();

            Self {
                images: mulfile
                    .iter_mut()
                    .map(|m| {
                        m.img_data.correct_plane();
                        m.img_data.correct_lines();
                        let uri = m.img_id.clone();
                        let bytes = m.img_data.to_png_bytes();

                        egui::Image::from_bytes(uri, bytes)
                    })
                    .collect(),
            }
        } else {
            Self { images: Vec::new() }
        }
    }
}

impl eframe::App for MyApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("grod")
                    .spacing(egui::vec2(5.0, 5.0))
                    .show(ui, |ui| {
                        for (i, img) in self.images.iter().enumerate() {
                            let sense = egui::Sense { click: true, drag: false, focusable: false };
                            let image_clone = img
                                .clone()
                                .sense(sense)
                                .fit_to_exact_size(egui::Vec2 { x: 200., y: 200. });
                            let response = ui.add(image_clone);
                            if response.clicked() {
                                println!("clicked {:?}", response);
                            }
                            if (i + 1) % 5 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
        });
    }
}
