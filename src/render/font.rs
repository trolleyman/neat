use prelude::*;
use std::rc::Rc;
use std::borrow::Cow;
use std::process::exit;

use glium::{Blend, BlendingFunction, LinearBlendingFactor, Texture2d, Program, Surface, VertexBuffer, IndexBuffer, DrawParameters, BackfaceCullingMode};
use glium::Rect as GlRect;
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, ClientFormat, MipmapsOption};
use rusttype::{Font, PositionedGlyph, GlyphId, IntoGlyphId, Rect, Scale, VMetrics, point};
use rusttype::gpu_cache::{Cache, CacheWriteErr};
use unicode_normalization::UnicodeNormalization;

use render::Color;
use util;
use vfs::{load_shader, load_font};

const SIZE: u32 = 8192;
static EMPTY_TEXTURE_DATA: [u8; SIZE as usize * SIZE as usize] = [0; SIZE as usize * SIZE as usize];

mod line_endings {
	use std::iter::Peekable;
	
	struct EndingIterator<I: Iterator<Item=char>> {
		it: Peekable<I>
	}
	impl<I: Iterator<Item=char>> Iterator for EndingIterator<I> {
		type Item = char;
		fn next(&mut self) -> Option<char> {
			if let Some(c) = self.it.next() {
				let cnext = self.it.peek().map(|&c| c);
				match (c, cnext) {
					('\r', Some('\n')) => self.it.next(), // Skip '\r'
					('\r', _) => Some('\n'), // Replace with '\n'
					_ => Some(c), // Keep as-is
				}
			} else {
				None
			}
		}
	}
	
	pub fn normalize_line_endings<I: Iterator<Item=char>>(it: I) -> impl Iterator<Item=char> {
		EndingIterator {
			it: it.peekable()
		}
	}
}
use self::line_endings::normalize_line_endings;

/// The state of the formatter
struct FormatState {
	init_x: f32,
	
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
	
	/// Lays out a string and returns the positioned glyphs that the text represents.
	/// 
	/// Handles newlines properly. Doesn't perform wrapping
	pub fn layout_text<'a, 'b>(&'b mut self, font: &Font<'a>, text: &str, glyphs: &mut Vec<(char, PositionedGlyph<'a>)>) {
		let mut cprev = None;
		for c in normalize_line_endings(text.chars().nfc()) {
			let glyph = self.layout_char(&font, cprev, c);
			glyphs.push((c, glyph));
			cprev = Some(c);
		}
	}
	
	/// Lays out a char at the current posiion, and updates the current position to be after it.
	/// 
	/// Handles newlines properly.
	fn layout_char<'a, 'b>(&'b mut self, font: &Font<'a>, cprev: Option<char>, c: char) -> PositionedGlyph<'a> {
		// Unwrap is safe here as force_print is true
		self.layout_char_imp(font, true, cprev, c).unwrap()
	}
	
	/// Lays out a char at the current posiion, and updates the current position to be after it.
	/// 
	/// Handles newlines.
	/// 
	/// Returns `None` if the char does not have a glyph for the character supplied.
	#[allow(dead_code)]
	fn try_layout_char<'a, 'b>(&'b mut self, font: &Font<'a>, cprev: Option<char>, c: char) -> Option<PositionedGlyph<'a>> {
		self.layout_char_imp(font, false, cprev, c)
	}
	
	/// Lays out a char at the current posiion, and updated the current position to be after it.
	/// 
	/// Handles newlines.
	/// 
	/// If `force_print` is true, then always prints a glyph, even if the character could not be found.
	/// If not, returns `None` if the char does not have a glyph for the character supplied.
	fn layout_char_imp<'a, 'b>(&'b mut self, font: &Font<'a>, force_print: bool, cprev: Option<char>, c: char) -> Option<PositionedGlyph<'a>> {
		if c == '\n' {
			self.newline();
		}
		
		let glyph_id = c.into_glyph_id(font);
		if force_print && glyph_id == GlyphId(0) {
			return None;
		}
		let glyph = font.glyph(glyph_id);
		
		let scaled = glyph.scaled(self.scale);
		let advance = scaled.h_metrics().advance_width;
		
		// Apply kerning
		match (cprev, c) {
			(Some(cprev), c) => {
				self.x += font.pair_kerning(self.scale, cprev, c);
			},
			_ => {}
		};
		
		let positioned = scaled.positioned(point(self.x, self.y));
		self.x += advance;
		Some(positioned)
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
	cache: Cache<'static>,
	
	font: Font<'static>,
	
	font_tex: Texture2d,
	shader: Program,
}
impl FontRender {
	/// Constructs a new font renderer with an OpenGL context.
	/// 
	/// Loads the default font from the filesystem.
	pub fn new(ctx: Rc<Context>) -> FontRender {
		let shader = load_shader(&ctx, "font");
		
		let font = load_font("consolas.ttf", 0);
		
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
		
		let cache = Cache::builder()
			.dimensions(SIZE, SIZE)
			.pad_glyphs(true)
			.multithread(false)
			.build();
		
		FontRender {
			ctx,
			cache,
			
			font,
			
			font_tex,
			shader,
		}
	}
	
	/// Draw a string at x, y on the screen scaled by scale.
	pub fn draw_str<S: Surface>(&mut self, surface: &mut S, s: &str, x: f32, y: f32, screen_w: f32, screen_h: f32, scale: f32, color: Color) {
		//println!("Rendering string: {}", s);
		let mut state = FormatState::new(x, y, scale, &self.font);
		let mut glyphs = Vec::new();
		state.layout_text(&self.font, s, &mut glyphs);
		
		let size = (screen_w, screen_h);
		draw_glyphs(&self.ctx, surface, &self.shader, &mut self.font_tex, &mut self.cache, size, &glyphs, color);
	}
}

/// Render and cache the specified glyphs.
/// 
/// # Returns
/// Err if the cache is too small to cache all of the glyphs and render them at once.
/// Retry with a smaller slice.
fn cache_glyphs<'a>(font_tex: &mut Texture2d, cache: &mut Cache<'a>, glyphs: &[(char, PositionedGlyph<'a>)]) -> Result<(), CacheWriteErr> {
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
fn draw_glyph<'a>(cache: &mut Cache<'a>, glyph: &PositionedGlyph<'a>, vs: &mut Vec<FontVertex>, is: &mut Vec<u32>) {
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
fn draw_glyphs<'a, S: Surface>(ctx: &Rc<Context>, surface: &mut S, shader: &Program, font_tex: &mut Texture2d, cache: &mut Cache<'a>, size: (f32, f32), glyphs: &[(char, PositionedGlyph<'a>)], color: Color) {
	// Calculate matrix
	let (w, h) = size;
	let mut mat = Matrix4::one();
	mat = mat * util::mat4_scale(Vector3::new(1.0, -1.0, 1.0));
	mat = mat * util::mat4_translation(Vector3::new(-1.0, -1.0, 0.0));
	mat = mat * util::mat4_scale(Vector3::new(2.0 / w, 2.0 / h, 1.0));
	draw_glyphs_mat(ctx, surface, shader, font_tex, cache, mat, glyphs, color)
}

/// Transforms the glyphs by `mat` and then draws the glyphs on `surface`.
fn draw_glyphs_mat<'a, S: Surface>(ctx: &Rc<Context>, surface: &mut S, shader: &Program, font_tex: &mut Texture2d, cache: &mut Cache<'a>, mat: Matrix4<f32>, glyphs: &[(char, PositionedGlyph<'a>)], color: Color) {
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
