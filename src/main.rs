// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};

use anyhow::Result;
use clap::Parser;
use notify::Watcher;
// use spm_rs::igor_ibw::read_ibw;
use spm_rs::{
    mulfile::{read_mul, MulImage},
    spm_image::SpmImage,
};

// use spm_rs::rhk_sm4::read_rhk_sm4;

use eframe::{
    egui::{self},
    // emath::RectTransform,
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
    filename: Option<String>,
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

struct GuiFile {
    filename: String,
    gui_images: Vec<GuiImage>,
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
    files: BTreeMap<String, GuiFile>,
    active_images: HashMap<String, bool>,
    start_rect: egui::Pos2,
    end_rect: egui::Pos2,
    file_watcher: Option<notify::RecommendedWatcher>,
    tx: Sender<PathBuf>,
    rx: Receiver<PathBuf>,
    scale_factor: f32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let args = Args::parse();
        let mut images = BTreeMap::new();

        let (tx, rx) = std::sync::mpsc::channel::<PathBuf>();

        match args.filename {
            Some(filename) if filename.ends_with(".mul") || filename.ends_with(".flm") => {
                let mulfile = read_mul(&filename).unwrap();

                let active_images = mulfile
                    .iter()
                    .map(|img| (img.img_id.clone(), false))
                    .collect();

                let gui_images: Vec<GuiImage> = mulfile
                    .into_iter()
                    .map(|mut img| {
                        img.img_data.correct_plane();
                        img.img_data.correct_lines();
                        GuiImage::new(img)
                    })
                    .collect();

                images.insert(
                    filename.clone(),
                    GuiFile {
                        filename,
                        gui_images,
                    },
                );

                Self {
                    files: images,
                    active_images,
                    start_rect: egui::Pos2::default(),
                    end_rect: egui::Pos2::default(),
                    file_watcher: None,
                    tx,
                    rx,
                    scale_factor: 1.0,
                }
            }
            _ => Self {
                files: images,
                active_images: HashMap::new(),
                start_rect: egui::Pos2::default(),
                end_rect: egui::Pos2::default(),
                file_watcher: None,
                tx,
                rx,
                scale_factor: 1.0,
            },
        }
    }

    fn main_window(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            self.menu(ctx, ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.grid_view(ctx, ui);
            });
        });
    }

    fn menu(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            let open_shortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O);
            if ui.input_mut(|i| i.consume_shortcut(&open_shortcut)) {
                let _ = self.open_file(ctx);
            }

            ui.menu_button("File", |ui| {
                if ui
                    .add(
                        egui::Button::new("Open...")
                            .shortcut_text(ui.ctx().format_shortcut(&open_shortcut)),
                    )
                    .clicked()
                {
                    let _ = self.open_file(ctx);
                }
            });
        });
    }

    fn open_file(&mut self, ctx: &egui::Context) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Some(parent) = path.parent() {
                for f in std::fs::read_dir(parent)? {
                    self.load_mulfile(f?.path());
                }

                let tx_clone = self.tx.clone();
                let ctx_clone = ctx.clone();

                self.file_watcher = Some(notify::recommended_watcher(
                    move |res: Result<notify::Event, notify::Error>| match res {
                        Ok(event) => match event.kind {
                            notify::EventKind::Modify(notify::event::ModifyKind::Any)
                            | notify::EventKind::Create(notify::event::CreateKind::Any) => {
                                if let Some(path) = event.paths.first() {
                                    if path.extension().is_some_and(|x| x == "mul") {
                                        let _ = tx_clone.send(path.into());
                                        ctx_clone.request_repaint();
                                    }
                                };
                            }
                            _ => (),
                        },
                        Err(_) => todo!(),
                    },
                )?);

                if let Some(watcher) = self.file_watcher.as_mut() {
                    watcher.watch(parent, notify::RecursiveMode::Recursive)?;
                }
            }
        }
        Ok(())
    }

    fn grid_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        for (f_name, gui_file) in self.files.iter() {
            ui.label(f_name);
            egui::Grid::new(f_name)
                .spacing(egui::vec2(5.0, 5.0))
                .show(ui, |ui| {
                    for (i, img) in gui_file.gui_images.iter().enumerate() {
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
            ui.separator();
        }
    }

    fn analysis_windows(&mut self, ctx: &egui::Context) {
        for (_, gui_file) in self.files.iter_mut() {
            for img in gui_file.gui_images.iter_mut() {
                if self.active_images.get(&img.img_id()).is_some_and(|&x| x) {
                    let new_viewport_id = egui::ViewportId::from_hash_of(&img.img_id());
                    let new_viewport = egui::ViewportBuilder::default()
                        .with_title(&img.img_id())
                        .with_inner_size(egui::Vec2 {
                            x: img.xres() as f32 * self.scale_factor,
                            y: img.yres() as f32 * self.scale_factor,
                        });
                    ctx.show_viewport_immediate(new_viewport_id, new_viewport, |ctx, _class| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::none())
                            .show(ctx, |ui| {
                                // ui.add(image_clone);
                                let analysis_image =
                                    egui::Image::from_bytes(img.img_id(), img.png.clone());
                                let image_rect = egui::Rect::from_two_pos(
                                    egui::Pos2::ZERO,
                                    egui::pos2(img.xres() as f32 * self.scale_factor, img.yres() as f32 * self.scale_factor),
                                );
                                analysis_image.paint_at(ui, image_rect);

                                // Create a "canvas" for drawing on
                                let (response, painter) = ui.allocate_painter(
                                    egui::Vec2::new(img.xres() as f32 * self.scale_factor, img.yres() as f32 * self.scale_factor),
                                    egui::Sense::click_and_drag(),
                                );

                                // Get the relative position of our "canvas"
                                let to_screen = egui::emath::RectTransform::from_to(
                                    image_rect,
                                    egui::Rect { min: egui::Pos2::ZERO, max: egui::pos2(img.xres() as f32, img.yres() as f32) },
                                );

                                if response.drag_started() {
                                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                                        let xres = img.xres() as f32 * self.scale_factor;
                                        let yres = img.yres() as f32 * self.scale_factor;
                                        let x = if pointer_pos.x > xres {
                                            xres
                                        } else {
                                            pointer_pos.x
                                        };
                                        let y = if pointer_pos.y > yres {
                                            yres
                                        } else {
                                            pointer_pos.y
                                        };
                                        self.start_rect = egui::pos2(x, y);
                                    }
                                }

                                if response.dragged() {
                                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                                        let xres = img.xres() as f32 * self.scale_factor;
                                        let yres = img.yres() as f32 * self.scale_factor;
                                        let x = if pointer_pos.x > xres {
                                            xres
                                        } else {
                                            pointer_pos.x
                                        };
                                        let y = if pointer_pos.y > yres {
                                            yres
                                        } else {
                                            pointer_pos.y
                                        };
                                        self.end_rect = egui::pos2(x, y);
                                    }
                                }

                                if response.drag_released() {
                                    let start_rect = to_screen.transform_pos(self.start_rect);
                                    let end_rect = to_screen.transform_pos(self.end_rect);
                                    let mut x_start = start_rect.x.round() as usize;
                                    let mut y_start = start_rect.y.round() as usize;
                                    let mut x_end = end_rect.x.round() as usize;
                                    let mut y_end = end_rect.y.round() as usize;
                                    if x_start > x_end {
                                        std::mem::swap(&mut x_start, &mut x_end);
                                    }
                                    if y_start > y_end {
                                        std::mem::swap(&mut y_start, &mut y_end);
                                    }
                                    if let Ok(new_png) = img
                                        .img_data()
                                        .to_png_bytes_selection(y_start, y_end, x_start, x_end)
                                    {
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
                                    if let Some(entry) = self.active_images.get_mut(&img.img.img_id)
                                    {
                                        *entry = false;
                                    };
                                }

                                if ctx.input(|i| i.key_pressed(egui::Key::Plus)) {
                                    self.scale_factor *= 1.2;
                                }
                                if ctx.input(|i| i.key_pressed(egui::Key::Minus)) {
                                    self.scale_factor *= 0.8;
                                }
                                if ctx.input(|i| i.key_pressed(egui::Key::Num0)) {
                                    self.scale_factor = 1.0;
                                }
                            });
                    });
                }
            }
        }
    }

    fn load_mulfile(&mut self, p: PathBuf) {
        if p.extension().is_some_and(|x| x == "mul") {
            let filename = p.file_name().unwrap();

            let mulfile = read_mul(&p.to_string_lossy()).unwrap();

            for mul_img in mulfile.iter() {
                self.active_images
                    .entry(mul_img.img_id.clone())
                    .or_insert(false);
            }

            let gui_images: Vec<GuiImage> = mulfile
                .into_iter()
                .map(|mut img| {
                    img.img_data.correct_plane();
                    img.img_data.correct_lines();
                    GuiImage::new(img)
                })
                .collect();

            self.files.insert(
                filename.to_string_lossy().to_string(),
                GuiFile {
                    filename: filename.to_string_lossy().to_string(),
                    gui_images,
                },
            );
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_window(ctx);
        self.analysis_windows(ctx);
        if let Ok(p) = self.rx.try_recv() {
            self.load_mulfile(p);
        };

    }
}
