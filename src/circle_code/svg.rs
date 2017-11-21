/*
* Project: circle-code
* File: src/svg.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

// static NB_LINES: u32 = 4_u32;
// static PRECISION: f64 = 10_f64;


use super::downloader;
use super::constructor;

use std::string::*;
use std::fmt;

use self::constructor::{Arc, describe_arc};

static NB_POINTS		: u32 = 36;
static IMAGE_SIZE		: f64 = 400.0;
static CIRCLE_RAY		: f64 = 180.0;
static STROKE_WIDTH		: u64 = 6;
static ANCHOR_BORDER	: u64 = 4;
static CIRCLE_PADDING	: f64 = 16.0;

#[derive(Debug, Default)]
pub struct SvgParam (String);

impl fmt::Display for SvgParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl SvgParam {
    fn new(x: &str) -> SvgParam {
        SvgParam (x.to_string())
    }

    fn from_float(x: f64) -> SvgParam {
        SvgParam (x.to_string())
    }

    fn from_int(x: u64) -> SvgParam {
        SvgParam (x.to_string())
    }
}

#[derive(Debug, Default)]
struct SvgGroup (Vec<String>);

impl fmt::Display for SvgGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut group: String = String::new();
        
        group.push_str("\t<g>\n");

        for elem in self.0.clone() {
            group.push_str(&format!("\t{}\n", elem));
        }
        group.push_str("\t</g>\n");

        write!(f, "{}", group)
    }
}

impl SvgGroup {
    fn new() -> SvgGroup {
        SvgGroup(Vec::new())
    }

    fn push(&mut self, s: String) {
        self.0.push(s);
    }
}

pub fn generate_canvas() -> Vec<Arc> {
    let mut arcs: Vec<Arc> = Vec::new();

    for level in 0..4 {
        for start in 0..36 {
            if level == 2 && (start == 0 || start % 9 == 0) {
                break
            }
            arcs.push(Arc{ start: start, len: 0, level: level })
        }
    }

    arcs
}

pub fn generate_svg_test(arcs: &[Arc], canvas: &[Arc], image_url: &str, _logo_url: &str, color: &str) -> Vec<String> {

    let mut svg: Vec<String> = Vec::new();

    svg.push(format!("<svg width=\"{0}px\" height=\"{0}px\" viewBox=\"0 0 {0} {0}\" viewport-fill=\"red\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n", IMAGE_SIZE));
    svg.push("\t<g id=\"first_line\" stroke=\"none\" stroke-width=\"1\" fill=\"none\" fill-rule=\"evenodd\">\n".to_owned());

    svg.push(svg_avatar(image_url).to_string());

    let center = SvgParam::from_float(IMAGE_SIZE / 2.0);
    let offset = IMAGE_SIZE / 2.0 - CIRCLE_RAY + CIRCLE_PADDING * 2.0;
    let left_offset = SvgParam::from_float(offset);
    let rifght_offset = SvgParam::from_float(IMAGE_SIZE - offset);

    svg.push(svg_anchor(&SvgParam::new(color), &center, &left_offset).to_string());
    svg.push(svg_anchor(&SvgParam::new(color), &center, &rifght_offset).to_string());
    svg.push(svg_anchor(&SvgParam::new(color), &left_offset, &center).to_string());
    svg.push(svg_anchor(&SvgParam::new(color), &rifght_offset, &center).to_string());

    svg.push(generate_svg_arcs(arcs, color).to_string());
    svg.push(generate_svg_arcs(canvas, "#80FFFFFF").to_string());

    svg.push("\t</g>\n".to_string());
    svg.push("</svg>".to_string());

    svg
}

pub fn generate_svg(arcs: &[Arc], image_url: &str, _logo_url: &str, color: &str) -> Vec<String> {

    let mut svg: Vec<String> = Vec::new();

    svg.push(format!("<svg width=\"{0}px\" height=\"{0}px\" viewBox=\"0 0 {0} {0}\" viewport-fill=\"red\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n", IMAGE_SIZE));
    svg.push("\t<g id=\"first_line\" stroke=\"none\" stroke-width=\"1\" fill=\"none\" fill-rule=\"evenodd\">\n".to_owned());
    
    svg.push(svg_avatar(image_url).to_string());

    let center = SvgParam::from_float(IMAGE_SIZE / 2.0);
    let offset = IMAGE_SIZE / 2.0 - CIRCLE_RAY + CIRCLE_PADDING * 2.0;
    let left_offset = SvgParam::from_float(offset);
    let rifght_offset = SvgParam::from_float(IMAGE_SIZE - offset);

    svg.push(svg_anchor(&SvgParam::new(color), &center, &left_offset).to_string());
    svg.push(svg_anchor(&SvgParam::new(color), &center, &rifght_offset).to_string());
    svg.push(svg_anchor(&SvgParam::new(color), &left_offset, &center).to_string());
    svg.push(svg_anchor(&SvgParam::new(color), &rifght_offset, &center).to_string());
    
    svg.push(generate_svg_arcs(arcs, color).to_string());

    svg.push("\t</g>\n".to_string());
    svg.push("</svg>".to_string());

    svg
}

fn generate_svg_arcs(arcs: &[Arc], color: &str) -> SvgGroup {
    let mut svg = SvgGroup::new();

    let color = SvgParam::new(color);
    let stroke_width = SvgParam::from_int(STROKE_WIDTH);

    let arc_lvl_0: Vec<Arc> = Vec::from(arcs).clone().into_iter().filter(|arc| arc.level == 0).collect();
    let arc_lvl_1: Vec<Arc> = Vec::from(arcs).clone().into_iter().filter(|arc| arc.level == 1).collect();
    let arc_lvl_2: Vec<Arc> = Vec::from(arcs).clone().into_iter().filter(|arc| arc.level == 2).collect();
    let arc_lvl_3: Vec<Arc> = Vec::from(arcs).clone().into_iter().filter(|arc| arc.level == 3).collect();

    svg.push(svg_arcs(&color, &stroke_width, &arc_lvl_0).to_string());
    svg.push(svg_arcs(&color, &stroke_width, &arc_lvl_1).to_string());
    svg.push(svg_arcs(&color, &stroke_width, &arc_lvl_2).to_string());
    svg.push(svg_arcs(&color, &stroke_width, &arc_lvl_3).to_string());

    svg
}

fn svg_anchor(color: &SvgParam, cx: &SvgParam, cy: &SvgParam) -> SvgGroup {

    let external_ray = SvgParam::from_int(8);
    let internal_ray = SvgParam::from_int(1);
    let stroke_width = SvgParam::from_int(ANCHOR_BORDER);

    let mut anchor = SvgGroup::new();

    anchor.push(format!("\t<circle stroke={color} stroke-width={stroke_width} cx={cx} cy={cy} r={external_ray}></circle>", 
        color = color, 
        stroke_width = stroke_width, 
        external_ray = external_ray, 
        cx = cx, 
        cy = cy)
    );
    anchor.push(format!("\t<circle stroke={color} stroke-width={stroke_width} cx={cx} cy={cy} r={internal_ray}></circle>", 
        color = color, 
        stroke_width = stroke_width, 
        internal_ray = internal_ray, 
        cx = cx, 
        cy = cy)
    );

    anchor
}

fn svg_arcs(color: &SvgParam, stroke_width: &SvgParam, arcs: &[Arc]) -> SvgGroup {
    let mut svg_arcs = SvgGroup::new();
    
    let angle = 360 / NB_POINTS;

    for arc in arcs.to_owned() {
        let d = SvgParam::new(&describe_arc((IMAGE_SIZE / 2.0), (IMAGE_SIZE / 2.0), CIRCLE_RAY - (CIRCLE_PADDING * f64::from(arc.level)), f64::from(arc.start * angle), f64::from((arc.start + arc.len) * angle)));
        let path = format!("\t<path stroke-linecap=\"round\" fill=\"none\" stroke={color} stroke-width={stroke_width} d={d}></path>", d = d, color = color, stroke_width = stroke_width);
        svg_arcs.push(path.clone());
    }

    svg_arcs
}

fn svg_avatar(image_url: &str) -> SvgGroup {
    let mut svg_avatar = SvgGroup::new();
    
    let url_image = SvgParam::new(&format!("data:image/svg+xml;base64,{}", downloader::image(image_url)));
    let c = SvgParam::from_float(IMAGE_SIZE / 2.0);

    let clip_size = SvgParam::from_float(CIRCLE_RAY - (4.0 * CIRCLE_PADDING));
    let image_size = SvgParam::from_float((CIRCLE_RAY - CIRCLE_PADDING) * 2.0);
    let image_padding = SvgParam::from_float((IMAGE_SIZE - (CIRCLE_RAY - CIRCLE_PADDING) * 2.0) / 2.0);

    svg_avatar.push("\t<clipPath id=\"clipCircle\">".to_string());
    svg_avatar.push(format!("\t\t<circle r={r} cx={cx} cy={cy}/>", r = clip_size, cx= c, cy = c));
    svg_avatar.push("\t</clipPath>".to_string());

    svg_avatar.push(format!("\t<image xlink:href={url_image} x={image_padding} y={image_padding} height={image_size} width={image_size} clip-path=\"url(#clipCircle)\"/>", url_image = url_image, image_size = image_size, image_padding =  image_padding));
    
    svg_avatar
}
