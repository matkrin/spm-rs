use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
// use spm_rs::igor_ibw::read_ibw;
use spm_rs::{
    mulfile::{read_mul, MulImage},
    spm_image::SpmImage,
};

// use spm_rs::rhk_sm4::read_rhk_sm4;

use eframe::{
    egui,
    emath::RectTransform,
    epaint::{Color32, Rounding},
};

const BROWSER_IMAGE_SIZE: f32 = 200.0;
const IMAGES_PER_ROW: usize = 6;
const BROWSER_WINDOW_WIDTH: f32 = (IMAGES_PER_ROW as f32) * (BROWSER_IMAGE_SIZE + 5.0) + 10.0;
const BROWSER_WINDOW_HEIGHT: f32 = 700.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg()]
    filename: String,
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([BROWSER_WINDOW_WIDTH, BROWSER_WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "spm-rs",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(cc))
        }),
    )
}

#[derive(Debug)]
struct GuiImage {
    img: MulImage,
    png: Vec<u8>,
}

impl GuiImage {
    pub fn new(img: MulImage) -> Self {
        let png = img.img_data.to_png_bytes();
        Self { img, png }
    }

    pub fn img_id(&self) -> String {
        self.img.img_id.clone()
    }

    pub fn img_data(&self) -> &SpmImage {
        &self.img.img_data
    }

    pub fn xres(&self) -> usize {
        self.img.xres
    }

    pub fn yres(&self) -> usize {
        self.img.yres
    }

    pub fn set_png(&mut self, png: Vec<u8>) {
        self.png = png
    }
}

struct MyApp {
    images: Vec<GuiImage>,
    active_images: HashMap<String, bool>,
    start_rect: egui::Pos2,
    end_rect: egui::Pos2,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
                    GuiImage::new(img)
                })
                .collect();

            Self {
                images: gui_images,
                active_images,
                start_rect: egui::Pos2::default(),
                end_rect: egui::Pos2::default(),
            }
        } else {
            Self {
                images: Vec::new(),
                active_images: HashMap::new(),
                start_rect: egui::Pos2::default(),
                end_rect: egui::Pos2::default(),
            }
        }
    }

    fn main_window(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| self.grid_view(ctx, ui));
        });
    }

    fn grid_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) -> Result<()> {
        egui::Grid::new("grid")
            .spacing(egui::vec2(5.0, 5.0))
            .show(ui, |ui| {
                for (i, img) in self.images.iter().enumerate() {
                    let sense = egui::Sense {
                        click: true,
                        drag: false,
                        focusable: false,
                    };
                    let image_clone =
                        egui::Image::from_bytes(img.img.img_id.clone(), img.png.clone())
                            .sense(sense)
                            .fit_to_exact_size(egui::Vec2 {
                                x: BROWSER_IMAGE_SIZE,
                                y: BROWSER_IMAGE_SIZE,
                            });
                    let response = ui.add(image_clone);
                    if response.double_clicked() {
                        if let Some(entry) = self.active_images.get_mut(&img.img.img_id) {
                            *entry = true;
                        };
                        println!("clicked {:?}", response);
                    }
                    if (i + 1) % IMAGES_PER_ROW == 0 {
                        ui.end_row();
                    }
                }
            });
        Ok(())
    }

    fn analysis_windows(&mut self, ctx: &egui::Context) -> Result<()> {
        for img in self.images.iter_mut() {
            if self.active_images.get(&img.img_id()).is_some_and(|&x| x) {
                let new_viewport_id = egui::ViewportId::from_hash_of(&img.img_id());
                let new_viewport = egui::ViewportBuilder::default()
                    .with_title(&img.img_id())
                    .with_inner_size(egui::Vec2 {
                        x: img.xres() as f32,
                        y: img.yres() as f32,
                    });
                ctx.show_viewport_immediate(new_viewport_id, new_viewport, |ctx, _class| {
                    egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
                        // ui.add(image_clone);
                        let analysis_image = egui::Image::from_bytes(img.img_id(), img.png.clone());
                        let image_rect = egui::Rect::from_two_pos(
                            egui::Pos2::ZERO,
                            egui::pos2(img.xres() as f32, img.yres() as f32),
                        );
                        analysis_image.paint_at(ui, image_rect);

                        // Create a "canvas" for drawing on
                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(img.xres() as f32, img.yres() as f32),
                            egui::Sense::click_and_drag(),
                        );


                        // Get the relative position of our "canvas"
                        // let to_screen = RectTransform::from_to(
                        //     egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
                        //     response.rect,
                        // );

                        if response.drag_started() {
                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                let xres = img.xres() as f32;
                                let yres = img.yres() as f32;
                                let x = if pointer_pos.x > xres { xres } else { pointer_pos.x };
                                let y = if pointer_pos.y > yres { yres } else { pointer_pos.y };
                                self.start_rect = egui::pos2(x, y);
                            }
                        }

                        if response.dragged() {
                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                let xres = img.xres() as f32;
                                let yres = img.yres() as f32;
                                let x = if pointer_pos.x > xres { xres } else { pointer_pos.x };
                                let y = if pointer_pos.y > yres { yres } else { pointer_pos.y };
                                self.end_rect = egui::pos2(x, y);
                            }
                        }

                        if response.drag_released() {
                            let mut x_start = self.start_rect.x.round() as usize;
                            let mut y_start = self.start_rect.y.round() as usize;
                            let mut x_end = self.end_rect.x.round() as usize;
                            let mut y_end = self.end_rect.y.round() as usize;
                            if x_start > x_end {
                                std::mem::swap(&mut x_start, &mut x_end);
                            }
                            if y_start > y_end {
                                std::mem::swap(&mut y_start, &mut y_end);
                            }
                            println!("x_start : {x_start}, x_end : {x_end}, y_start : {y_start}, y_end: {y_end}" );
                            if let Ok(new_png) = img.img_data().to_png_bytes_selection(y_start, y_end, x_start, x_end) {
                                img.set_png(new_png);
                                let analysis_image_uri = analysis_image
                                    .source()
                                    .uri()
                                    .unwrap_or_default()
                                    .to_string();
                                ui.ctx().forget_image(&analysis_image_uri);
                            };
                        }

                        painter.add(egui::Shape::Rect(eframe::epaint::RectShape {
                            rect: egui::Rect::from_two_pos(self.start_rect, self.end_rect),
                            rounding: Rounding::ZERO,
                            fill: Color32::TRANSPARENT,
                            stroke: egui::Stroke {
                                width: 1.0,
                                color: Color32::YELLOW,
                            },
                            fill_texture_id: egui::TextureId::Managed(0),
                            uv: egui::Rect::ZERO,
                        }));

                        if ctx.input(|i| i.viewport().close_requested()) {
                            // Tell parent viewport that we should not show next frame:
                            if let Some(entry) = self.active_images.get_mut(&img.img.img_id) {
                                *entry = false;
                            };
                        }
                    });
                });
            }
        }
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_window(ctx);
        self.analysis_windows(ctx);
    }
}
