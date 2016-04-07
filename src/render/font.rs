use std::io::Write;
use std::rc::Rc;
use std::borrow::Cow;
use std::process::exit;

use glium::{Blend, BlendingFunction, LinearBlendingFactor, Texture2d, Program, Surface, VertexBuffer, IndexBuffer, DrawParameters, BackfaceCullingMode};
use glium::Rect as GlRect;
use glium::index::PrimitiveType;
use glium::backend::Context;
use glium::texture::{RawImage2d, ClientFormat};
use rusttype::*;
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use na::{Vec3, Mat4, Eye};
use unicode_normalization::UnicodeNormalization;

use render::Color;
use util;
use vfs::{load_shader, load_font};

const SIZE: u32 = 8192;
static EMPTY_TEXTURE_DATA: [u8; SIZE as usize * SIZE as usize] = [0; SIZE as usize * SIZE as usize];

struct State {
	init_x: f32,
	_init_y: f32,
	
	pub x: f32,
	pub y: f32,
	scale: Scale,
	
	v_metrics: VMetrics,
}
impl State {
	pub fn new(x: f32, y: f32, scale: f32, font: &Font) -> State {
		let scale = Scale::uniform(scale);
		let v_metrics = font.v_metrics(scale);
		let y = y + v_metrics.ascent;
		State{
			init_x: x,
			_init_y: y,
			
			x: x,
			y: y,
			scale: scale,
			
			v_metrics: v_metrics,
		}
	}
	
	pub fn newline(&mut self) {
		self.x = self.init_x;
		self.y = self.y + self.v_metrics.descent + self.v_metrics.ascent + self.v_metrics.line_gap;
	}
	
	pub fn layout_char<'a, 'b>(&'b mut self, font: &'a Font, cprev: Option<char>, c: char) -> Option<PositionedGlyph<'a>> {
		let c = match (cprev, c) {
			(Some('\r'), '\n') => {
				self.newline();
				None
			},
			(Some('\n'), _) | (Some('\r'), _) => {
				self.newline();
				Some(c)
			},
			(_, _) => {
				Some(c)
			},
		};
		if let Some(glyph) = c.and_then(|c| font.glyph(c)) {
			let scaled = glyph.scaled(self.scale);
			let advance = scaled.h_metrics().advance_width;
			let positioned = scaled.positioned(point(self.x, self.y));
			self.x += advance;
			Some(positioned)
		} else {
			None
		}
	}
}

#[derive(Copy, Clone, Debug)]
struct FontVertex {
	pub pos: [f32; 2],
	pub uv : [f32; 2],
}
impl FontVertex {
	pub fn new<U, V>(pos: U, uv: V) -> FontVertex where U: Into<[f32; 2]>, V: Into<[f32; 2]> {
		FontVertex {
			pos: pos.into(),
			uv : uv .into(),
		}
	}
}

implement_vertex!(FontVertex, pos, uv);

pub struct FontRender {
	ctx: Rc<Context>,
	cache: Cache,
	
	font_collection: FontCollection<'static>,
	font_index: usize,
	
	font_tex: Texture2d,
	shader: Program,
}
impl FontRender {
	pub fn new(ctx: Rc<Context>) -> FontRender {
		let shader = match load_shader(&ctx, "font") {
			Ok(i)  => i,
			Err(e) => {
				error!("{}", e);
				exit(1);
			}
		};
		
		const FONT_PATH: &'static str = "fonts/consolas.ttf";
		const FONT_INDEX: usize = 0;
		
		let font_collection = match load_font(FONT_PATH, FONT_INDEX) {
			Ok(f) => f,
			Err(e) => {
				error!("{}", e);
				exit(1);
			}
		};
		
		let img = RawImage2d {
			data  : Cow::Borrowed(&EMPTY_TEXTURE_DATA as &[u8]),
			width : SIZE,
			height: SIZE,
			format: ClientFormat::U8,
		};
		
		let font_tex = match Texture2d::new(&ctx, img) {
			Ok(t) => t,
			Err(e) => {
				error!("Could not create texture: {:?}", e);
				exit(1);
			},
		};
		
		FontRender {
			ctx: ctx,
			cache: Cache::new(SIZE, SIZE, 0.1, 0.1),
			
			font_collection: font_collection,
			font_index: FONT_INDEX,
			
			font_tex: font_tex,
			shader: shader,
		}
	}
	
	/// Draw a string at x, y on the screen scaled by scale.
	pub fn draw_str<S: Surface>(&mut self, surface: &mut S, s: &str, x: f32, y: f32, screen_w: f32, screen_h: f32, scale: f32, color: Color) {
		let font = match self.font_collection.font_at(self.font_index) {
			Some(f) => f,
			None => {
				warn!("The font at {} could not be loaded from the collection.", self.font_index);
				return;
			},
		};
		
		//println!("Rendering string: {}", s);
		let mut glyphs = Vec::new();
		
		let mut state = State::new(x, y, scale, &font);
		
		let mut cprev = None;
		for c in s.chars().nfc() {
			if let Some(glyph) = state.layout_char(&font, cprev, c) {
				glyphs.push((c, glyph));
			}
			cprev = Some(c);
		}
		
		let size = (screen_w, screen_h);
		draw_glyphs(&self.ctx, surface, &self.shader, &mut self.font_tex, &mut self.cache, size, &glyphs, color);
	}
}

fn cache_queued(font_tex: &mut Texture2d, cache: &mut Cache) -> Result<(), CacheWriteErr> {
	cache.cache_queued(|rect: Rect<u32>, data| {
		let rect = GlRect {
			left:   rect.min.x,
			bottom: rect.min.y,
			width:  rect.width(),
			height: rect.height(),
		};
		let data = RawImage2d {
			data:   Cow::from(data),
			width:  rect.width,
			height: rect.height,
			format: ClientFormat::U8,
		};
		
		font_tex.write(rect, data);
	})
}

fn draw_glyph(cache: &mut Cache, glyph: &PositionedGlyph, vs: &mut Vec<FontVertex>, is: &mut Vec<u32>) {
	if let Ok(Some((uv, pos))) = cache.rect_for(0, glyph) {
		// 0--1
		// |  |
		// 2--3
		let i = vs.len() as u32;
		is.push(i);
		is.push(i+2);
		is.push(i+1);
		
		is.push(i+1);
		is.push(i+2);
		is.push(i+3);
		
		vs.push(FontVertex::new([pos.min.x as f32, pos.min.y as f32], [uv.min.x, uv.min.y]));
		vs.push(FontVertex::new([pos.max.x as f32, pos.min.y as f32], [uv.max.x, uv.min.y]));
		vs.push(FontVertex::new([pos.min.x as f32, pos.max.y as f32], [uv.min.x, uv.max.y]));
		vs.push(FontVertex::new([pos.max.x as f32, pos.max.y as f32], [uv.max.x, uv.max.y]));
	}
}

fn draw_glyphs<S: Surface>(ctx: &Rc<Context>, surface: &mut S, shader: &Program, font_tex: &mut Texture2d, cache: &mut Cache, size: (f32, f32), glyphs: &[(char, PositionedGlyph)], color: Color) {
	// Calculate matrix
	let (w, h) = size;
	let mut mat = Mat4::new_identity(4);
	mat = mat * util::mat4_scale(Vec3::new(1.0, -1.0, 1.0));
	mat = mat * util::mat4_translation(Vec3::new(-1.0, -1.0, 0.0));
	mat = mat * util::mat4_scale(Vec3::new(2.0 / w, 2.0 / h, 1.0));
	draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, glyphs, color)
}

fn draw_glyphs_mat<S: Surface>(ctx: &Rc<Context>, surface: &mut S, shader: &Program, font_tex: &mut Texture2d, cache: &mut Cache, mat: Mat4<f32>, glyphs: &[(char, PositionedGlyph)], color: Color) {
	cache.clear_queue();
	for &(_, ref glyph) in glyphs.iter() {
		cache.queue_glyph(0, glyph.clone());
	}
	match cache_queued(font_tex, cache) {
		Ok(()) => {
			let mut vs = Vec::new();
			let mut is = Vec::new();
			
			for &(_, ref glyph) in glyphs {
				draw_glyph(cache, glyph, &mut vs, &mut is);
			}
			
			// Upload buffer
			let vs = match VertexBuffer::immutable(ctx, &vs) {
				Ok(vs) => vs,
				Err(e) => {
					error!("Could not create vertex buffer: {:?}", e);
					return;
				},
			};
			let is = match IndexBuffer ::immutable(ctx, PrimitiveType::TrianglesList, &is) {
				Ok(is) => is,
				Err(e) => {
					error!("Could not create index buffer: {:?}", e);
					return;
				},
			};
			// Draw buffer
			surface.draw(
				&vs,
				&is,
				&shader,
				&uniform!{
					tex:   &*font_tex,
					color: <Color as Into<[f32; 3]>>::into(color),
					mat:   *mat.as_ref(),
				},
				&DrawParameters {
					blend: Blend {
						color: BlendingFunction::Addition{
							source:      LinearBlendingFactor::SourceAlpha,
							destination: LinearBlendingFactor::OneMinusSourceAlpha,
						},
						alpha: BlendingFunction::Max,
						..Default::default()
					},
					backface_culling: BackfaceCullingMode::CullClockwise,
					..Default::default()
				}
			).ok();
		},
		Err(e) => {
			if glyphs.len() == 0 {
				// Do nothing, it's fine to not render any glyphs.
				error!("Error: This should never happen, but it's fine if it does.");
			} else if glyphs.len() == 1 {
				// TODO: Maybe render default glyph?
				let c = glyphs[0].0;
				error!("Error: Cannot render character '{}' ({:#04X}): {:?}", c, c as u32, e);
			} else {
				warn!("Error: Cannot render all glyphs in array (len {}): {:?}, splitting at {}", glyphs.len(), e, glyphs.len() / 2);
				// Split glyphs up into two halves, and draw them seperately.
				let (a, b) = glyphs.split_at(glyphs.len() / 2);
				draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, a, color);
				draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, b, color);
			}
		}
	}
}
