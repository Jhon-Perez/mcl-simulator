use std::f32::consts::TAU;

use ggez::{
    Context, ContextBuilder, GameResult,
    event::{self, EventHandler},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    input::keyboard::KeyCode,
};

const FIELD_SIZE: f32 = 140.42;

struct Robot {
    x: f32,
    y: f32,
    h: f32,
    size: f32,
}

impl Robot {
    fn new() -> GameResult<Self> {
        Ok(Self {
            x: 100.0,
            y: 100.0,
            h: 0.0,
            size: 14.5, // 14.9
        })
    }
}

impl EventHandler for Robot {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();

        let linear_velocity = 82.0;
        let angular_velocity = 900.0f32.to_radians() * 0.4;

        let mut new_x = self.x;
        let mut new_y = self.y;
        let mut new_h = self.h;

        let mut distance = 0.0;
        let mut delta_theta = 0.0;

        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            distance += linear_velocity;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            distance -= linear_velocity;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            delta_theta += angular_velocity;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            delta_theta -= angular_velocity;
        }

        distance *= dt;
        delta_theta *= dt;
        new_h = (new_h + delta_theta).rem_euclid(TAU);

        new_x += distance * new_h.cos();
        new_y += distance * new_h.sin();

        let half = self.size / 2.0;

        new_x = new_x.clamp(half, FIELD_SIZE - half);
        new_y = new_y.clamp(half, FIELD_SIZE - half);

        self.x = new_x;
        self.y = new_y;
        self.h = new_h;

        println!("{}, {}, {}", self.x, self.y, self.h.to_degrees());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(20, 20, 20));

        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let scale = screen_w.min(screen_h) / FIELD_SIZE;

        let screen_x = self.x * scale;
        let screen_y = (FIELD_SIZE - self.y) * scale;
        let screen_size = self.size * scale;

        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                -screen_size / 2.0,
                -screen_size / 2.0,
                screen_size,
                screen_size,
            ),
            Color::WHITE,
        )?;

        canvas.draw(
            &rect,
            DrawParam::default()
                .dest([screen_x, screen_y])
                .rotation(-self.h),
        );

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("mcl_sim", "Jhon")
        .window_setup(ggez::conf::WindowSetup::default().title("MCL Sim"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 1000.0))
        .build()?;

    let state = Robot::new()?;
    event::run(ctx, event_loop, state)
}
