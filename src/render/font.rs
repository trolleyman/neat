use prelude::*;
use std::io::Write;
use std::rc::Rc;
use std::borrow::Cow;
use std::process::exit;

use glium::{Blend, BlendingFunction, LinearBlendingFactor, Texture2d, Program, Surface, VertexBuffer, IndexBuffer, DrawParameters, BackfaceCullingMode};
use glium::Rect as GlRect;
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, ClientFormat, MipmapsOption};
use rusttype::*;
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use unicode_normalization::UnicodeNormalization;

use render::Color;
use util;
use vfs::{load_shader, load_font};

const SIZE: u32 = 8192;
static EMPTY_TEXTURE_DATA: [u8; SIZE as usize * SIZE as usize] = [0; SIZE as usize * SIZE as usize];

/// The state of the formatter
struct FormatState {
	init_x: f32,
	_init_y: f32,
	
	pub x: f32,
	pub y: f32,
	scale: Scale,
	
	v_metrics: VMetrics,
}
impl FormatState {
	pub fn new(x: f32, y: f32, scale: f32, font: &Font) -> FormatState {
		let scale = Scale::uniform(scale);
		let v_metrics = font.v_metrics(scale);
		let y = y + v_metrics.ascent;
		FormatState{
			init_x: x,
			_init_y: y,
			
			x: x,
			y: y,
			scale: scale,
			
			v_metrics: v_metrics,
		}
	}
	
	/// Processes a newline
	pub fn newline(&mut self) {
		self.x = self.init_x;
		self.y = self.y + self.v_metrics.descent + self.v_metrics.ascent + self.v_metrics.line_gap;
	}
	
	/// Lays out a char at the current posiion, and updated the current position to be after it.
	/// 
	/// Handles newlines properly.
	/// 
	/// Returns None if the char does not have a glyph.
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

/// Font rendering handler.
pub struct FontRender {
	ctx: Rc<Context>,
	cache: Cache,
	
	font_collection: FontCollection<'static>,
	font_index: usize,
	
	font_tex: Texture2d,
	shader: Program,
}
impl FontRender {
	/// Constructs a new font renderer with an OpenGL context.
	/// 
	/// Loads the default font from the filesystem.
	pub fn new(ctx: Rc<Context>) -> FontRender {
		let shader = load_shader(&ctx, "font");
		
		const FONT_PATH: &'static str = "consolas.ttf";
		const FONT_INDEX: usize = 0;
		
		let font_collection = load_font(FONT_PATH, FONT_INDEX);
		
		let img = RawImage2d {
			data  : Cow::Borrowed(&EMPTY_TEXTURE_DATA as &[u8]),
			width : SIZE,
			height: SIZE,
			format: ClientFormat::U8,
		};
		
		let font_tex = match Texture2d::with_mipmaps(&ctx, img, MipmapsOption::NoMipmap) {
			Ok(t) => t,
			Err(e) => {
				error!("Could not create texture: {:?}", e);
				exit(1);
			},
		};
		
		FontRender {
			ctx: ctx,
			cache: Cache::new(SIZE, SIZE, 0.1, 1.0),
			
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
		
		let mut state = FormatState::new(x, y, scale, &font);
		
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

/// Render and cache the specified glyphs.
/// 
/// # Returns
/// Err if the cache is too small to cache all of the glyphs and render them at once.
/// Retry with a smaller slice.
fn cache_glyphs(font_tex: &mut Texture2d, cache: &mut Cache, glyphs: &[(char, PositionedGlyph)]) -> Result<(), CacheWriteErr> {
	cache.clear_queue();
	for &(_, ref glyph) in glyphs.iter() {
		cache.queue_glyph(0, glyph.clone());
	}
	let mut n = 0;
	let ret = cache.cache_queued(|rect: Rect<u32>, data| {
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
		
		n += 1;
		
		font_tex.write(rect, data);
	});
	if n > 0 {
		let s: String = glyphs.iter().map(|&(c, _)| c).collect();
		if n == 1 {
			warn!("{} cache miss while rendering '{}'", n, s)
		} else {
			warn!("{} cache misses while rendering '{}'", n, s)
		}
	}
	ret
}

/// Adds the vertices necessary to `vs` and `is` to draw the glyph to the screen, if it is in `cache`.
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

/// Draws the glyphs at a specified point on `surface`.
/// 
/// Properly calculates matrix.
fn draw_glyphs<S: Surface>(ctx: &Rc<Context>, surface: &mut S, shader: &Program, font_tex: &mut Texture2d, cache: &mut Cache, size: (f32, f32), glyphs: &[(char, PositionedGlyph)], color: Color) {
	// Calculate matrix
	let (w, h) = size;
	let mut mat = Mat4::one();
	mat = mat * util::mat4_scale(Vec3::new(1.0, -1.0, 1.0));
	mat = mat * util::mat4_translation(Vec3::new(-1.0, -1.0, 0.0));
	mat = mat * util::mat4_scale(Vec3::new(2.0 / w, 2.0 / h, 1.0));
	draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, glyphs, color)
}

/// Transforms the glyphs by `mat` and then draws the glyphs on `surface`.
fn draw_glyphs_mat<S: Surface>(ctx: &Rc<Context>, surface: &mut S, shader: &Program, font_tex: &mut Texture2d, cache: &mut Cache, mat: Mat4<f32>, glyphs: &[(char, PositionedGlyph)], color: Color) {
	match cache_glyphs(font_tex, cache, glyphs) {
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
					tex  : &*font_tex,
					color: color.into_array(),
					mat  : *mat.as_ref(),
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
			} else if glyphs.len() == 1 {
				// TODO: Maybe render default glyph?
				let c = glyphs[0].0;
				error!("Cannot render character '{}' ({:#04X}): {:?}", c, c as u32, e);
			} else {
				warn!("Cannot render all glyphs in array (len {}): {:?}, splitting at {}", glyphs.len(), e, glyphs.len() / 2);
				// Split glyphs up into two halves, and draw them seperately.
				let (a, b) = glyphs.split_at(glyphs.len() / 2);
				draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, a, color);
				draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, b, color);
			}
		}
	}
}
