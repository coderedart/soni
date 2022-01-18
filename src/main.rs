use std::time::{Duration, Instant};

use egui_macroquad::egui;
use macroquad::prelude::*;
#[macroquad::main("egui with macroquad")]
async fn main() {
    let state = State::new(screen_width(), screen_height());
    let end = 10.0_f32; // seconds for flight trip duration
    let plane = Texture2D::from_file_with_format(PLANE_BYTES, Some(ImageFormat::Png));
    loop {
        clear_background(WHITE);
        let gradient = end / state.start.elapsed().as_secs_f32();
        let current_a1_pos = state.a1_pos.lerp(state.a1_direction, gradient);
        
        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Aeroplane").show(egui_ctx, |ui| {
                ui.label(format!(
                    "aeroplane 1 position: ({}, {})",
                    &state.a1_pos[0], &state.a1_pos[1]
                ));
                ui.label(format!("current airplane 1 position: {} {}", current_a1_pos.x, current_a1_pos.y));
            });
        });
        let width = screen_width() / 2.0;
        let height = screen_height() / 2.0;
        let x = screen_width() / 4.0;
        let y = screen_height() / 4.0;
        macroquad::shapes::draw_rectangle_lines(
            x,
            y,
            width,
            height,
            5.0,
            macroquad::color_u8!(255, 0, 0, 255),
        );
        // Draw things before egui
        draw_texture_ex(
            plane,
            current_a1_pos.x,
            current_a1_pos.y,
            macroquad::color::WHITE,
            DrawTextureParams {
                dest_size: Some([32.0, 32.0].into()),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub a1_pos: Vec2,
    pub a2_pos: Vec2,
    pub a1_direction: Vec2,
    pub a2_direction: Vec2,
    pub start: Instant,
}

impl State {
    pub fn new(width: f32, height: f32) -> Self {
        let a1_pos = [0.0, 0.0].into();
        let a2_pos = [width, 0.0].into();
        let a1_direction = [width, height].into();
        let a2_direction = [0.0, height].into();
        Self {
            a1_pos,
            a2_pos,
            a1_direction,
            a2_direction,
            start: Instant::now(),
        }
    }
}
pub const PLANE_BYTES: &[u8] = include_bytes!("../plane.png");
