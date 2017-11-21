
use super::math;
use super::svg::{NB_POINTS};

#[derive(Debug, Clone, Copy)]
pub struct Arc {
    pub start: u32,
    pub len: u32,
    pub level: u32
}

pub fn describe_arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle:f64) -> String {
    let start: math::CartesianCoord= math::polar_to_cartesian(x, y, radius, end_angle);
    let end: math::CartesianCoord = math::polar_to_cartesian(x, y, radius, start_angle);

    let large_arc_flag: &str = if end_angle - start_angle <= 180_f64 { "0" } else { "1" };

    format!("M {} {} A {} {} 0 {} 0 {} {}", start.x, start.y, radius, radius, large_arc_flag, end.x, end.y)
}


pub fn calculate_arcs(code: &[u32]) -> Vec<Arc> {
    let mut arcs: Vec<Arc> = Vec::new();

    let mut start: u32 = 0;
    let mut len:u32 = 0;
    let mut level:u32 = 0;

    for c in code {
        if start + len >= NB_POINTS {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
            }
            start = 0;
            len = 0;
            level += 1;
        }
        if level == 2 && (start + len - 1) % 9 == (NB_POINTS / 4) - 1 {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
            }
            start += len + 2;
            len = 0;
        }
        if *c == 0 {
            start += 1;
        } else if *c == 1 {
            len += 1;
        }
    }

    arcs
}
