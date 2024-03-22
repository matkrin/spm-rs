use anyhow::Result;
use clap::Parser;
use spm_rs::igor_ibw::read_ibw;
use spm_rs::mulfile::read_mul;
use spm_rs::omicron_matrix::read_omicron_matrix;
// use spm_rs::rhk_sm4::read_rhk_sm4;

// use eframe::egui;
// use egui_extras::RetainedImage;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg()]
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.filename.ends_with(".mul") || args.filename.ends_with(".flm") {
        let mulfile = read_mul(&args.filename)?;
        // let b = load_from_memory(mulfile[0].img_data.img_data.as_slice());
        println!("{:?}", mulfile[0].img_data.to_png_bytes());

        // for mut i in mulfile {
        //     i.img_data.correct_plane();
        //     i.img_data.correct_lines();
        //     i.img_data.save_png();
        // }
    } else if args.filename.ends_with(".Z_mtrx") {
        let mut omicron_matrix = read_omicron_matrix(&args.filename)?;
        omicron_matrix.img_data_fw.correct_plane();
        omicron_matrix.img_data_bw.correct_lines();
        // omicron_matrix.img_data_fw.save_png();
        // omicron_matrix.img_data_bw.save_png();
    } else if args.filename.ends_with(".ibw") {
        let wave = read_ibw(&args.filename)?;
        dbg!(wave);
        // println!("{}", wave.data[0]);
        // println!("{:?}", wave.bname)
    // } else if args.filename.to_lowercase().ends_with(".sm4") {
        // let rhk_sm4 = read_rhk_sm4(&args.filename)?;
        // dbg!(rhk_sm4);
    }

    Ok(())
}

// fn main() {
//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(1350.0, 800.0)),
//         ..Default::default()
//     };
//
//     eframe::run_native(
//         "Show an image with eframe/egui",
//         options,
//         Box::new(|_cc| Box::new(MyApp::default())),
//     );
// }
//
// struct MyApp {
//     images: Vec<RetainedImage>,
// }
//
// impl Default for MyApp {
//     fn default() -> Self {
//         let args = Args::parse();
//         if args.filename.ends_with(".mul") || args.filename.ends_with(".flm") {
//             let mut mulfile = read_mul(&args.filename).unwrap();
//             Self {
//                 images: mulfile.iter_mut().map(|m| {
//                     m.img_data.correct_plane();
//                     m.img_data.correct_lines();
//
//                     RetainedImage::from_image_bytes(m.img_id.clone(), &m.img_data.to_png_bytes())
//                 }
//                 .unwrap()).collect(),
//             }
//         } else if args.filename.ends_with(".Z_mtrx") {
//             unimplemented!();
//         } else {
//             unimplemented!();
//         }
//     }
// }
//
// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             egui::ScrollArea::vertical().show(ui, |ui| {
//                 egui::Grid::new("grod").spacing(egui::vec2(5.0, 5.0)).show(ui, |ui| {
//                     for (i, img) in self.images.iter().enumerate() {
//                         ui.add(
//                             egui::Image::new(img.texture_id(ctx), egui::vec2(256.0, 256.0))
//                         );
//                         if (i + 1) % 5 == 0 {
//                             ui.end_row();
//                         }
//                     }
//                 });
//             });
//
//
//             // ui.heading("This is an image you can click:");
//             // ui.add(egui::ImageButton::new(
//             //     self.image.texture_id(ctx),
//             //     // self.image.size_vec2(),
//             //     egui::vec2(200.0, 200.0)
//             // ));
//         });
//     }
// }
