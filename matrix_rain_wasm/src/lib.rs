use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use image::{ ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage };
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_filled_circle_mut, draw_text_mut, draw_filled_rect_mut };
use imageproc::rect::Rect;
use rusttype::{ Font, Scale };
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use image::io::Reader as ImageReader;
use image::Pixel;
use web_sys::console;
use std::cmp;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

const REF_IMG_DATA: [[u8; 50]; 53] = [
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,69 ,143 ,148 ,73 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,77 ,152 ,168 ,163 ,166 ,120 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,76 ,153 ,146 ,142 ,138 ,151 ,162 ,138 ,51 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,64 ,148 ,151 ,153 ,159 ,159 ,155 ,151 ,151 ,134 ,48 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,46 ,137 ,152 ,151 ,152 ,140 ,117 ,118 ,125 ,140 ,143 ,112 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,105 ,138 ,136 ,129 ,125 ,124 ,137 ,137 ,132 ,116 ,123 ,134 ,63 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,54 ,131 ,121 ,119 ,136 ,121 ,103 ,128 ,135 ,139 ,122 ,107 ,119 ,101 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,84 ,126 ,132 ,107 ,76 ,48 ,44 ,44 ,46 ,69 ,109 ,119 ,108 ,107 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,98 ,121 ,65 ,44 ,44 ,44 ,44 ,44 ,45 ,45 ,52 ,110 ,116 ,99 ,52 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,44 ,103 ,56 ,44 ,44 ,45 ,45 ,45 ,45 ,45 ,45 ,45 ,46 ,91 ,100 ,54 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,70 ,55 ,43 ,44 ,44 ,45 ,45 ,46 ,46 ,46 ,46 ,46 ,45 ,45 ,72 ,56 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,44 ,54 ,43 ,44 ,44 ,44 ,45 ,46 ,46 ,47 ,47 ,47 ,46 ,46 ,45 ,45 ,56 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,46 ,43 ,43 ,44 ,44 ,45 ,45 ,46 ,46 ,47 ,47 ,47 ,47 ,46 ,45 ,45 ,50 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,44 ,44 ,45 ,45 ,46 ,47 ,47 ,48 ,47 ,47 ,47 ,45 ,45 ,48 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,44 ,44 ,45 ,45 ,46 ,47 ,48 ,48 ,48 ,47 ,47 ,46 ,45 ,47 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,44 ,44 ,45 ,45 ,46 ,46 ,47 ,47 ,47 ,47 ,46 ,45 ,45 ,45 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,44 ,44 ,45 ,45 ,46 ,47 ,47 ,47 ,47 ,46 ,45 ,45 ,45 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,43 ,44 ,45 ,45 ,45 ,46 ,46 ,46 ,46 ,45 ,45 ,44 ,45 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,43 ,43 ,44 ,45 ,45 ,45 ,45 ,45 ,45 ,45 ,44 ,44 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,43 ,44 ,44 ,44 ,45 ,45 ,45 ,45 ,44 ,44 ,44 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,43 ,43 ,44 ,44 ,44 ,44 ,44 ,44 ,44 ,43 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,57 ,89 ,43 ,43 ,43 ,43 ,43 ,44 ,44 ,44 ,43 ,43 ,43 ,43 ,43 ,60 ,51 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,58 ,107 ,132 ,92 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,55 ,119 ,98 ,52 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,91 ,110 ,129 ,135 ,132 ,92 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,48 ,98 ,118 ,125 ,102 ,56 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,96 ,127 ,131 ,135 ,132 ,125 ,119 ,85 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,43 ,52 ,82 ,110 ,124 ,127 ,128 ,103 ,56 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,62 ,120 ,130 ,128 ,124 ,120 ,121 ,119 ,109 ,73 ,46 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,43 ,43 ,43 ,42 ,43 ,53 ,75 ,98 ,112 ,116 ,119 ,125 ,124 ,79 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,86 ,123 ,121 ,120 ,119 ,119 ,118 ,110 ,95 ,65 ,47 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,51 ,66 ,88 ,103 ,108 ,112 ,115 ,116 ,120 ,51 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,54 ,78 ,98 ,100 ,94 ,95 ,93 ,82 ,85 ,83 ,60 ,47 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,49 ,60 ,79 ,87 ,92 ,95 ,96 ,96 ,97 ,64 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,61 ,59 ,74 ,83 ,74 ,80 ,81 ,77 ,86 ,78 ,58 ,46 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,44 ,48 ,56 ,69 ,76 ,80 ,87 ,90 ,82 ,68 ,59 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,0 ,56 ,59 ,62 ,68 ,71 ,80 ,70 ,82 ,87 ,71 ,57 ,46 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,44 ,47 ,52 ,64 ,79 ,81 ,90 ,79 ,65 ,50 ,50 ,45 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,47 ,47 ,48 ,55 ,56 ,71 ,67 ,65 ,89 ,72 ,65 ,55 ,46 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,46 ,51 ,59 ,72 ,82 ,86 ,66 ,54 ,47 ,44 ,44 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,45 ,43 ,42 ,46 ,44 ,43 ,47 ,85 ,77 ,62 ,63 ,54 ,46 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,46 ,51 ,57 ,61 ,80 ,81 ,60 ,48 ,43 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,44 ,42 ,42 ,43 ,43 ,53 ,80 ,72 ,52 ,65 ,58 ,50 ,45 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,46 ,49 ,58 ,65 ,65 ,57 ,48 ,44 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,44 ,42 ,42 ,42 ,43 ,47 ,43 ,45 ,56 ,59 ,54 ,49 ,45 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,45 ,48 ,51 ,49 ,47 ,42 ,42 ,42 ,43 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,44 ,43 ,42 ,42 ,42 ,42 ,43 ,42 ,47 ,54 ,55 ,50 ,47 ,44 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,44 ,46 ,48 ,46 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,44 ,45 ,42 ,42 ,42 ,42 ,42 ,43 ,45 ,47 ,51 ,46 ,46 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,44 ,44 ,44 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,43 ,44 ,43 ,42 ,42 ,42 ,42 ,42 ,45 ,57 ,45 ,44 ,44 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,45 ,42 ,42 ,42 ,42 ,42 ,42 ,44 ,60 ,49 ,44 ,45 ,44 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,44 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,44 ,44 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,45 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,45 ,44 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,44 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,44 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,43 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0],
[0 ,0 ,0 ,0 ,0 ,0 ,43 ,43 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,42 ,43 ,43 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0]
];

#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas_matrix_bg_1 = document.get_element_by_id("matrix-bg-1").unwrap();
    let canvas_matrix_bg_1: web_sys::HtmlCanvasElement = canvas_matrix_bg_1
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx_mx_bg_1 = canvas_matrix_bg_1
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let canvas_hoodie = document.get_element_by_id("hoodie-canvas").unwrap();
    let canvas_hoodie : web_sys::HtmlCanvasElement = canvas_hoodie
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx_hoodie = canvas_hoodie
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas_matrix_bg_1.width();
    let height = canvas_matrix_bg_1.height();

    let mut mat_set_1 = setup(width, height, 15);
    let mut mat_set_2 = setup(width, height, 20);
    let green_1 = mat_set_1.medium_green;
    let green_2 = mat_set_1.bright_green;

    let trail_data = TrailData {
        kor_data: [
            [0xAC00,0xBE1D,0xC73C,0xB0D4,0xB2C8,0xB969,0xC9F0,0xB530,0xB69C,0xB871],
            [0xC906,0xD2C8,0xBE60,0xBE0D,0xCE59,0xC2B7,0xC545,0xBD40,0xB4F1,0xCCB5],
            [0xCE20,0xCF00,0xD000,0xBF08,0xBC14,0xB728,0xBF55,0xAD70,0xC448,0xB560]
        ],
        eng_data: [
            [0x003E,0x002B,0x003D,0x0074,0x0078,0x0025,0x0030,0x002E,0x0026,0x003F], //>==tx
            [0x0032,0x0035,0x0035,0x002E,0x0032,0x0035,0x0035,0x002E,0x0030,0x0030],
            [0x0069,0x0070,0x0061,0x0064,0x0064,0x0031,0x0072,0x0068,0x006F,0x0073]
        ],
        num_data: [
            [0x0030,0x0030,0x0031,0x0030,0x0031,0x0031,0x0031,0x0030,0x0031,0x0030], //0010111010
            [0x0031,0x0030,0x0030,0x0031,0x0030,0x0030,0x0031,0x0031,0x0030,0x0030], //1001001100
            [0x0031,0x0031,0x0030,0x0030,0x0030,0x0031,0x0030,0x0031,0x0030,0x0030]
        ]
    };

    let ref_trail_rands_1 = mat_set_1.trail_offsets.clone();
    let ref_trail_rands_2 = mat_set_2.trail_offsets.clone();

    let low_res_width = 50;
    let low_res_height = 52;


    animate_limited(move || {

        {
            let mut img_1 = RgbaImage::new(width, height);
            ctx_mx_bg_1.clear_rect(0.0, 0.0, width as f64, height as f64);
            draw_matrix(&mut mat_set_1, &mut img_1, 1, &trail_data, &ref_trail_rands_1, green_1);
            draw_matrix(&mut mat_set_2, &mut img_1, 3, &trail_data, &ref_trail_rands_2, green_2);
            let img_data: web_sys::ImageData = web_sys::ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(img_1.as_raw()), width).unwrap();
            ctx_mx_bg_1.put_image_data(&img_data, 0.0, 0.0).unwrap();
        }

        {
            let portrait_block_size = 15;
            let mut img_3 = RgbaImage::new(low_res_width * portrait_block_size, low_res_height * portrait_block_size);
            ctx_hoodie.clear_rect(0.0, 0.0, (low_res_width * portrait_block_size) as f64, (low_res_height * portrait_block_size) as f64);
            create_portrait(&mut mat_set_1, &mut img_3, low_res_width, low_res_height, &trail_data, &ref_trail_rands_1, green_2, portrait_block_size);
            let img_data: web_sys::ImageData = web_sys::ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(img_3.as_raw()), low_res_width * portrait_block_size).unwrap();
            ctx_hoodie.put_image_data(&img_data, 0.0, 0.0).unwrap();
        }

    }, 1, )
}

fn create_portrait(
        mat_set: &Setup,
        img: &mut RgbaImage,
        low_res_width: u32,
        low_res_height: u32,
        trail_data: &TrailData,
        trail_rands: &Vec<i32>,
        green: [u8; 3],
        block_size: u32,
        ) {

    console::log_1(&"Draw portrait called".into());
    draw_filled_rect_mut(img, Rect::at(0, 0).of_size(block_size * low_res_width, block_size * low_res_height), Rgba::from([0, 0, 0, 255]));

    for i in 0..low_res_width {
        let font: &Font<'static>;
        let font_scale: Scale;
        let font_data: &[u32; 10];
        let mut font_offset: i32 = 0;
        match (trail_rands[i as usize % trail_rands.len()]) % 4 {
            0 => { 
                font = &mat_set.kor_font;
                font_scale = Scale { x: mat_set.block_size as f32, y: mat_set.block_size as f32 };
                font_data = &trail_data.kor_data[0];
            },
            1 => { 
                font = &mat_set.eng_font;
                font_scale = Scale { x: mat_set.block_size as f32 * 1.3, y: mat_set.block_size as f32 * 1.3 };
                font_data = &trail_data.eng_data[0];
                font_offset = (block_size / 4) as i32;
            },
            _ => { 
                font = &mat_set.num_font;
                font_scale = Scale { x: mat_set.block_size as f32 * 1.3, y: mat_set.block_size as f32 * 1.3 };
                font_data = &trail_data.num_data[0];
                font_offset = (block_size / 4) as i32;
            }
        };

        for j in 0..low_res_height {
            let font_num = ((mat_set.trail_offsets[i as usize % trail_rands.len()] - (j as i32)).abs() % 10) as usize;
            let mut black = Rgba::from([0, 0, 0, 255]);
            let p_val = cmp::min((REF_IMG_DATA[j as usize][i as usize] as f64 * 1.5) as u8, 255);
            black.blend(&Rgba::from([green[0], green[1], green[2], p_val]));
            draw_text_mut(img,
                          black,
                          (i * block_size) as i32 + font_offset,
                          (j * block_size) as i32,
                          font_scale,
                          font, &char::from_u32(font_data[font_num]).unwrap().to_string()
                          );
        }
    }
}

fn animate_limited(mut draw_frame: impl FnMut() + 'static, max_fps: i32) {
    let animate_cb = Rc::new(RefCell::new(None));
    let animate_cb2 = animate_cb.clone();

    let timeout_cb = Rc::new(RefCell::new(None));
    let timeout_cb2 = timeout_cb.clone();

    let w = window();
    *timeout_cb2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(&w, animate_cb.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let w2 = window();
    *animate_cb2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_frame();

        set_timeout(&w2, timeout_cb.borrow().as_ref().unwrap(), 1000 / max_fps);
    }) as Box<dyn FnMut()>));

    request_animation_frame(&window(), animate_cb2.borrow().as_ref().unwrap());
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}

#[derive(Clone)]
struct Setup {
    width: u32,
    height: u32,
    width_unit: u32,
    height_unit: u32,
    block_size: u32,
    kor_font: Font<'static>,
    eng_font: Font<'static>,
    num_font: Font<'static>,
    trail_len: u32,
    trail_offsets: Vec<i32>,
    trail_langs: Vec<u8>, // 0 for korean, 1 for english, 2 for numbers (binary)
    bright_green: [u8; 3],
    medium_green: [u8; 3],
    dark_green: [u8; 3]
}

struct TrailData {
    kor_data: [[u32; 10]; 3],
    eng_data: [[u32; 10]; 3],
    num_data: [[u32; 10]; 3]
}

fn setup(width: u32, height: u32, block_size: u32) -> Setup {
    // Loading font
    let mut font_data: &[u8] = include_bytes!("./SongMyung-Regular.ttf");
    let kor_font: Font<'static>;
    if let Some(font_option) =  Font::try_from_bytes(font_data) {
        kor_font = font_option;
    } else {
        panic!("None font!");
    }

    font_data = include_bytes!("./RobotoMono-Regular.ttf");
    let eng_font: Font<'static>;
    if let Some(font_option) =  Font::try_from_bytes(font_data) {
        eng_font = font_option;
    } else {
        panic!("None font!");
    }

    let num_font = eng_font.clone();

    let mut trail_offsets: Vec<i32> = Vec::new();
    let mut trail_langs: Vec<u8> = Vec::new();
    for _ in 0..(width / block_size) {
        trail_offsets.push((rand::thread_rng().gen_range(0..2*(height / block_size))) as i32);
        let lang: u8 = rand::thread_rng().gen_range(0..3) as u8;
        trail_langs.push(lang);
    }

    Setup {
        width,
        height,
        width_unit: (width / block_size),
        height_unit: (height / block_size),
        block_size,
        trail_len: (height / ( block_size)),
        kor_font,
        eng_font,
        num_font,
        trail_offsets,
        trail_langs,
        bright_green: [0, 255, 65],
        medium_green: [0, 143, 17],
        dark_green: [0, 59, 0]
    }
}

fn draw_matrix(mat_set: &mut Setup,
               img: &mut RgbaImage,
               inc: i32,
               trail_data: &TrailData,
               ref_trail_rands: &Vec<i32>,
               green: [u8; 3]
               ) {
    for (pos, trail_offset) in mat_set.trail_offsets.iter_mut().enumerate() {
        let mut trail_font_index: usize = 9;
        let rand_num: u8 = (ref_trail_rands[pos] % 256) as u8;
        let font: &Font<'static>;
        let mut font_lang_num = (rand_num & 0x0F) % 6;
        let font_data_num = (rand_num >> 4) % 3;
        let font_data: &[u32; 10];
        let font_scale: Scale;
        match font_lang_num {
            0 => {
                font = &mat_set.kor_font; font_lang_num = 0;
                font_data = &trail_data.kor_data[font_data_num as usize];
                font_scale = Scale { x: mat_set.block_size as f32, y: mat_set.block_size as f32 };
                font_lang_num = 0;

            },
            1 => { 
                font = &mat_set.eng_font; font_lang_num = 1;
                font_data = &trail_data.eng_data[font_data_num as usize];
                font_scale = Scale { x: mat_set.block_size as f32 * 1.3, y: mat_set.block_size as f32 * 1.3};
                font_lang_num = 1;
            },
            _ => { 
                font = &mat_set.num_font; font_lang_num = 2;
                font_data = &trail_data.num_data[font_data_num as usize];
                font_scale = Scale { x: mat_set.block_size as f32 * 1.3, y: mat_set.block_size as f32 * 1.3};
                font_lang_num = 2;
            }
        };

        let mut font_pos_adjust: i32 = 0;
        if font_lang_num != 0 {
            font_pos_adjust = mat_set.block_size as i32 / 4;
        }

        for i in 0..mat_set.trail_len {
            let coord: (i32, i32) = (
                (pos * mat_set.block_size as usize) as i32,
                (*trail_offset - i as i32 + inc) * mat_set.block_size as i32
            );
            if coord.0 < 0 || coord.1 < 0 { continue; }
            let alpha: u8 = ((255 * (mat_set.trail_len - i)) / mat_set.trail_len) as u8;

            draw_text_mut(img,
                          Rgba::from([green[0], green[1], green[2], alpha]),
                          // (coord.0 + (mat_set.block_size as i32/4)),
                          coord.0 + font_pos_adjust,
                          coord.1,
                          font_scale,
                          font, &char::from_u32(font_data[trail_font_index]).unwrap().to_string()
                          );
            trail_font_index = (trail_font_index + 1) % 10;
        }
        *trail_offset += inc;
        if (*trail_offset - mat_set.trail_len as i32) >= mat_set.height_unit as i32 {
            *trail_offset = 0;
        }
    }
}

fn draw_canvas(context: &web_sys::CanvasRenderingContext2d) {
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
