/*
* Project: circle-code
* File: circle_code/consructor.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

use super::math;
use super::svg::{ANCHOR_EXT, ANCHOR_BORDER, SPointNumber};

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

#[allow(dead_code)]
pub fn test_arc(nb_points: &SPointNumber) -> Vec<Arc> {
    let mut arcs: Vec<Arc> = Vec::new();

    for i in 0..4 {
        for j in 0..nb_points.num {
            if i == 2 && nb_points.avoid.contains(&j) {

            } else {
                let arc = Arc{ start: j, len: 1, level: i };
                arcs.push(arc);
            }
        }
    }

    arcs
}


pub fn calculate_arcs(code: &[u32], nb_points: &SPointNumber) -> Vec<Arc> {
    let mut arcs: Vec<Arc> = Vec::new();

    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut delta = 0;

    for i_line in 0..4 {
        let mut row: Vec<u32> = Vec::new();

        for i_case in 0..nb_points.num {
            let index = ((i_line * nb_points.num ) + i_case) as usize;

            if i_line == 2 && nb_points.avoid.contains(&(i_case as u32)) {
                row.push(0);
                delta += 1;
                continue;
            }

            if index > code.len() - 1 { break }

            row.push(code[index - delta]);
        }

        map.push(row);
    }

    println!("{:?}", map[2]);

    let mut start: u32 = 0;
    let mut len:u32 = 0;
    let mut level:u32 = 0;

    // let arc = Arc{ start: start, len: len, level: level };
    // arcs.push(arc);

    let pt_in_anchor = (ANCHOR_EXT + ANCHOR_BORDER) / (360 / nb_points.num);

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
