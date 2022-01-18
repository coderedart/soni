use std::time::Instant;

use egui_macroquad::egui;
use macroquad::prelude::*;
#[macroquad::main("egui with macroquad")]
async fn main() {
    let plane_texture = Texture2D::from_file_with_format(PLANE_BYTES, Some(ImageFormat::Png));
    let _plane_sound = macroquad::audio::load_sound_from_bytes(PLANE_SOUND_BYTES)
        .await
        .unwrap();
    let width = screen_width() / 2.0;
    let height = screen_height() / 2.0;
    let x = screen_width() / 4.0;
    let y = screen_height() / 4.0;
    let mut planes = vec![];
    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Airport").show(egui_ctx, |ui| {
                ui.label(format!("total number of planes: {}", planes.len()));
                if ui.button("add plane").clicked() {
                    planes.push(Plane::default());
                }
            });
            planes = planes
                .clone()
                .into_iter()
                .enumerate()
                .filter_map(|(number, mut plane)| {
                    let mut delete = false;

                    egui::Window::new(number).show(egui_ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("source x: ");
                            ui.add(
                                egui::DragValue::new(&mut plane.source.x)
                                    .speed(0.004)
                                    .clamp_range(0.0..=1.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("source y: ");
                            ui.add(
                                egui::DragValue::new(&mut plane.source.y)
                                    .speed(0.004)
                                    .clamp_range(0.0..=1.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("source z: ");
                            ui.add(
                                egui::DragValue::new(&mut plane.source.z)
                                    .speed(0.004)
                                    .clamp_range(0.0..=1.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("destination x: ");
                            ui.add(
                                egui::DragValue::new(&mut plane.destination.x)
                                    .speed(0.004)
                                    .clamp_range(0.0..=1.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("destination y: ");
                            ui.add(
                                egui::DragValue::new(&mut plane.destination.y)
                                    .speed(0.004)
                                    .clamp_range(0.0..=1.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("destination z: ");
                            ui.add(
                                egui::DragValue::new(&mut plane.destination.z)
                                    .speed(0.004)
                                    .clamp_range(0.0..=1.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("trip duration: ");
                            ui.add(egui::DragValue::new(&mut plane.trip_duration));
                        });
                        if let Some(start_time) = plane.start_instant {
                            ui.label("trip completion percentage");
                            let completion =
                                start_time.elapsed().as_secs_f32() / plane.trip_duration;
                            let mut completion = completion * 100.0;
                            if completion > 100.0 {
                                plane.start_instant = None;
                            } else {
                                ui.add(egui::Slider::new(&mut completion, 0.0..=100.0));
                            }
                        } else if ui.button("start plane").clicked() {
                            plane.start_instant = Some(Instant::now());
                        }

                        if ui.button("delete plane").clicked() {
                            delete = true;
                        }
                    });
                    (!delete).then(|| plane)
                })
                .collect::<Vec<Plane>>();
        });

        macroquad::shapes::draw_rectangle_lines(
            x,
            y,
            width,
            height,
            5.0,
            macroquad::color_u8!(255, 0, 0, 255),
        );
        for p in planes.iter() {
            if let Some(start_time) = p.start_instant {
                let completion = start_time.elapsed().as_secs_f32() / p.trip_duration;
                let present_position = p.source.lerp(p.destination, completion);
                draw_texture_ex(
                    plane_texture,
                    present_position.x * screen_width(),
                    present_position.y * screen_height(),
                    macroquad::color::WHITE,
                    DrawTextureParams {
                        dest_size: Some([64.0, 64.0].into()),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );
            }
        }
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}

pub const PLANE_BYTES: &[u8] = include_bytes!("../plane.png");
pub const PLANE_SOUND_BYTES: &[u8] = include_bytes!("../plane.ogg");
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    /// in normalized coordinates of window. between 0.0 - 1.0
    pub source: Vec3,
    /// in normalized coordinates of window. between 0.0 - 1.0
    pub destination: Vec3,
    /// in seconds
    pub trip_duration: f32,
    /// start time
    pub start_instant: Option<Instant>,
}
impl Default for Plane {
    fn default() -> Self {
        Self {
            source: Default::default(),
            destination: Default::default(),
            trip_duration: 10.0,
            start_instant: Default::default(),
        }
    }
}
