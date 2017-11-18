/*
 * Project: circle-code
 * File: src/svg.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

// static NB_LINES: u32 = 4_u32;
// static PRECISION: f64 = 10_f64;

use super::math;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::string::*;
use std::fmt;

static DATA_FILE		: &'static str = "data.svg";
static NB_POINTS		: u32 = 36;
static IMAGE_SIZE		: f64 = 400.0;
static CIRCLE_RAY		: f64 = 180.0;
static STROKE_WIDTH		: u64 = 6;
static ANCHOR_BORDER	: u64 = 4;
static CIRCLE_PADDING	: f64 = 12.0;

#[derive(Debug, Default)]
struct SvgParam (String);

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

#[derive(Debug, Clone, Copy)]
pub struct Arc {
	pub start: u32,
	pub len: u32,
	pub level: u32
}

fn describe_arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle:f64) -> SvgParam {
	let start: math::CartesianCoord= math::polar_to_cartesian(x, y, radius, end_angle);
	let end: math::CartesianCoord = math::polar_to_cartesian(x, y, radius, start_angle);

	let large_arc_flag: &str = match end_angle - start_angle <= 180_f64 {
		true => "0",
		false => "1"
	};

	SvgParam::new(&format!("M {} {} A {} {} 0 {} 0 {} {}", start.x, start.y, radius, radius, large_arc_flag, end.x, end.y))
}


pub fn calculate_arcs(code: String) -> Vec<Arc> {
	let mut arcs: Vec<Arc> = Vec::new();

	let mut start: u32 = 0;
	let mut len:u32 = 0;
	let mut level:u32 = 0;

	for c in code.chars() {
		if start + len == NB_POINTS - 1 {
			arcs.push(Arc{
				start: start,
				len: len,
				level: level
			});
			start = 0;
			len = 0;
			level +=1;
		}
		if c == '0' {
			if len != 0 {
				arcs.push(Arc{
					start: start,
					len: len,
					level: level
				});
				start += len;
				len = 0;
			}
			start += 1;
		} else if c == '1' {
			len += 1;
		}
	}

	arcs
}

fn save_svf_file(paths: Vec<String>) {
	let path = Path::new(DATA_FILE);
	let display = path.display();

	let mut file = match File::create(&path) {
		Err(why) => {
			println!("couldn't create {}: {}",
				display,
				why.description());
			return
		},
		Ok(file) => file,
	};

	for data in paths {
		match file.write_all(data.as_bytes()) {
			Err(why) => {
				println!("couldn't write to {}: {}",
					display,
					why.description());
				return
			},
			Ok(_) => {},
		}
	}
	
}

pub fn generate_svg(arcs: Vec<Arc>, image_url: &str, _logo_url: &str, color: &str) {

	let mut svg: Vec<String> = Vec::new();
	let color = SvgParam::new(color);
	let stroke_width = SvgParam::from_int(STROKE_WIDTH);

	svg.push(format!("<svg width=\"{0}px\" height=\"{0}px\" viewBox=\"0 0 {0} {0}\" viewport-fill=\"red\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n", IMAGE_SIZE));
	svg.push(format!("\t<g id=\"first_line\" stroke=\"none\" stroke-width=\"1\" fill=\"none\" fill-rule=\"evenodd\">\n"));
	
	svg.push(svg_avatar(image_url).to_string());

	let center = SvgParam::from_float(IMAGE_SIZE / 2.0);
	let offset = IMAGE_SIZE / 2.0 - CIRCLE_RAY + CIRCLE_PADDING * 2.0;
	let left_offset = SvgParam::from_float(offset);
	let rifght_offset = SvgParam::from_float(IMAGE_SIZE - offset);
	
	svg.push(svg_anchor(&color, &center, &left_offset).to_string());
	svg.push(svg_anchor(&color, &center, &rifght_offset).to_string());
	svg.push(svg_anchor(&color, &left_offset, &center).to_string());
	svg.push(svg_anchor(&color, &rifght_offset, &center).to_string());

	svg.push(svg_arcs(&color, &stroke_width, &(arcs.clone().into_iter().filter(|arc| arc.level == 0).collect())).to_string());
	svg.push(svg_arcs(&color, &stroke_width, &(arcs.clone().into_iter().filter(|arc| arc.level == 1).collect())).to_string());
	svg.push(svg_arcs(&color, &stroke_width, &(arcs.clone().into_iter().filter(|arc| arc.level == 2).collect())).to_string());
	svg.push(svg_arcs(&color, &stroke_width, &(arcs.clone().into_iter().filter(|arc| arc.level == 3).collect())).to_string());

	svg.push("\t</g>\n".to_string());
	svg.push("</svg>".to_string());

	save_svf_file(svg);
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

fn svg_arcs(color: &SvgParam, stroke_width: &SvgParam, arcs: &Vec<Arc>) -> SvgGroup {
	let mut svg_arcs = SvgGroup::new();
	
	let angle = 360 / NB_POINTS;
	
	for arc in arcs.clone() {
		let d = describe_arc((IMAGE_SIZE / 2.0), (IMAGE_SIZE / 2.0), CIRCLE_RAY - (CIRCLE_PADDING * arc.level as f64), (arc.start * angle) as f64, ((arc.start + arc.len) * angle) as f64);
		let path = format!("\t<path stroke-linecap=\"round\" fill=\"none\" stroke={color} stroke-width={stroke_width} d={d}></path>", d = d, color = color, stroke_width = stroke_width);
		svg_arcs.push(path.clone());
	}

	svg_arcs
}

fn svg_avatar(image_url: &str) -> SvgGroup {
	let mut svg_avatar = SvgGroup::new();
	
	let url_image = SvgParam::new(image_url);
	let c = SvgParam::from_float((IMAGE_SIZE / 2.0));

	let clip_size = SvgParam::from_float(CIRCLE_RAY - (4.0 * CIRCLE_PADDING));
	let image_size = SvgParam::from_float((CIRCLE_RAY - CIRCLE_PADDING) * 2.0);
	let image_padding = SvgParam::from_float((IMAGE_SIZE - (CIRCLE_RAY - CIRCLE_PADDING) * 2.0) / 2.0);

	svg_avatar.push("\t<clipPath id=\"clipCircle\">".to_string());
	svg_avatar.push(format!("\t\t<circle r={r} cx={cx} cy={cy}/>", r = clip_size, cx= c, cy = c));
	svg_avatar.push("\t</clipPath>".to_string());

	svg_avatar.push(format!("\t<image xlink:href={url_image} x={image_padding} y={image_padding} height={image_size} width={image_size} clip-path=\"url(#clipCircle)\"/>", url_image = url_image, image_size = image_size, image_padding =  image_padding));
	
	svg_avatar
}
