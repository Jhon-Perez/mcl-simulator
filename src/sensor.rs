use ggez::{
    Context, GameResult,
    graphics::{Canvas, Color, DrawParam, Mesh},
};

use crate::{FIELD_SIZE, Robot};

pub const MAX_DIST: f32 = 78.74015748; // 2000mm

pub struct Sensor {
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_h: f32,
    pub max_dist: f32,
}

impl Sensor {
    fn world_pose(&self, robot: &Robot) -> (f32, f32, f32) {
        let (sin, cos) = robot.h.sin_cos();

        // Rotate offset into world frame
        let x = robot.x + self.offset_x * cos - self.offset_y * sin;
        let y = robot.y + self.offset_x * sin + self.offset_y * cos;

        let h = robot.h + self.offset_h;

        (x, y, h)
    }
}

fn raycast(x: f32, y: f32, angle: f32, max_dist: f32) -> f32 {
    let dx = angle.cos();
    let dy = angle.sin();

    let mut min_t = max_dist;

    // Avoid divide by zero issues
    if dx.abs() > 1e-6 {
        // Left wall (x = 0)
        let t = (0.0 - x) / dx;
        if t > 0.0 {
            let y_hit = y + t * dy;
            if y_hit >= 0.0 && y_hit <= FIELD_SIZE {
                min_t = min_t.min(t);
            }
        }

        // Right wall (x = FIELD_SIZE)
        let t = (FIELD_SIZE - x) / dx;
        if t > 0.0 {
            let y_hit = y + t * dy;
            if y_hit >= 0.0 && y_hit <= FIELD_SIZE {
                min_t = min_t.min(t);
            }
        }
    }

    if dy.abs() > 1e-6 {
        // Bottom wall (y = 0)
        let t = (0.0 - y) / dy;
        if t > 0.0 {
            let x_hit = x + t * dx;
            if x_hit >= 0.0 && x_hit <= FIELD_SIZE {
                min_t = min_t.min(t);
            }
        }

        // Top wall (y = FIELD_SIZE)
        let t = (FIELD_SIZE - y) / dy;
        if t > 0.0 {
            let x_hit = x + t * dx;
            if x_hit >= 0.0 && x_hit <= FIELD_SIZE {
                min_t = min_t.min(t);
            }
        }
    }

    min_t.min(max_dist)
}

pub fn draw_sensor(
    ctx: &mut Context,
    canvas: &mut Canvas,
    sensor: &Sensor,
    robot: &Robot,
    scale: f32,
) -> GameResult {
    let (sx, sy, sh) = sensor.world_pose(robot);

    let dist = raycast(sx, sy, sh, sensor.max_dist);

    println!("dist: {}", dist);

    let end_x = sx + dist * sh.cos();
    let end_y = sy + dist * sh.sin();

    // Convert to screen space
    let start_screen = [sx * scale, (FIELD_SIZE - sy) * scale];
    let end_screen = [end_x * scale, (FIELD_SIZE - end_y) * scale];

    let mesh = Mesh::new_line(ctx, &[start_screen, end_screen], 2.0, Color::RED)?;

    canvas.draw(&mesh, DrawParam::default());

    Ok(())
}
