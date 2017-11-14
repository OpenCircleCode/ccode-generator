
use std::f64;

struct CartesianCoord {
    x: f64,
    y: f64
}

pub fn polar_to_cartesian(centerX: f64, centerY: f64, radius: f64, angle: f64) -> CartesianCoord {
    let angle_radians = (angle - 90_f64) * f64::consts::PI / 180_f64;

    CartesianCoord {
        x: centerX + (radius * angle_radians.cos()),
        y: centerY + (radius * angle_radians.sin())
    }
}