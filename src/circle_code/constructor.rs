/*
* Project: circle-code
* File: circle_code/consructor.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

use super::math;
use super::svg::{ANCHOR_EXT, SPointNumber};

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


pub fn calculate_arcs(code: &[u32], nb_points: &SPointNumber) -> Vec<Arc> {
    let mut arcs: Vec<Arc> = Vec::new();

    let mut start: u32 = 0;
    let mut len:u32 = 0;
    let mut level:u32 = 0;

    let nb_points_for_anchor = (ANCHOR_EXT + 1_u32) / (360_u32 / nb_points.num);
    let anchor_pos = nb_points.num / 4;

    for c in code {
        let index = start + if len == 0 { 0 } else { len - 1 };
        if level == 2 && index % anchor_pos >= anchor_pos - nb_points_for_anchor {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
            }
            start += len + nb_points_for_anchor * 2;
            len = 0;
        } else if index >= nb_points.num {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
            }
            len = 0;
            level += 1;
            start = if level == 2 {nb_points_for_anchor} else {0};
        } else if *c == 0 {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
                start += len;
                len = 0;
            }
            start += 1;
        } else if *c == 1 {
            len += 1;
        }
    }

    arcs
}


#[test]
fn test_describe_arc_1() {
    assert!(describe_arc(150_f64, 150_f64, 120_f64, 0_f64, 10_f64) == "M 268.176930361465 170.83778132003164 A 120 120 0 0 0 270 150");
}

#[test]
fn test_describe_arc_2() {
    assert!(describe_arc(150_f64, 150_f64, 120_f64, 90_f64, 90_f64) == "M 150 270 A 120 120 0 0 0 150 270");
}
