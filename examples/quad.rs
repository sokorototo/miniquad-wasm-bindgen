use miniquad_wasm_bindgen::*;

#[repr(C)]
struct Vec2 {
	x: f32,
	y: f32,
}
#[repr(C)]
struct Vertex {
	pos: Vec2,
	uv: Vec2,
}

struct Stage {
	ctx: Box<dyn RenderingBackend>,

	pipeline: Pipeline,
	bindings: Bindings,
}

impl Stage {
	pub fn new() -> Stage {
		let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

		#[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
        ];
		let vertex_buffer = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices));

		let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
		let index_buffer = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices));

		let pixels: [u8; 4 * 4 * 4] = [
			0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
			0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
		];
		let texture = ctx.new_texture_from_rgba8(4, 4, &pixels);

		let bindings = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer,
			images: vec![texture],
		};

		let shader = ctx.new_shader(ShaderSource::new(shader::VERTEX, shader::FRAGMENT), shader::meta()).unwrap();

		let pipeline = ctx.new_pipeline(
			&[BufferLayout::default()],
			&[VertexAttribute::new("in_pos", VertexFormat::Float2), VertexAttribute::new("in_uv", VertexFormat::Float2)],
			shader,
			PipelineParams::default(),
		);

		Stage { pipeline, bindings, ctx }
	}
}

impl EventHandler for Stage {
	fn update(&mut self) {}

	fn draw(&mut self) {
		let t = date::now();

		self.ctx.begin_default_pass(Default::default());

		self.ctx.apply_pipeline(&self.pipeline);
		self.ctx.apply_bindings(&self.bindings);
		for i in 0..10 {
			let t = t + i as f64 * 0.3;

			self.ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms {
				offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
			}));
			self.ctx.draw(0, 6, 1);
		}
		self.ctx.end_render_pass();

		self.ctx.commit_frame();
	}
}

fn main() {
	miniquad_wasm_bindgen::start(conf::Conf::default(), move || Box::new(Stage::new()));
}

mod shader {
	use miniquad_wasm_bindgen::*;

	pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos + offset, 0, 1);
        texcoord = in_uv;
    }"#;

	pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

	pub fn meta() -> ShaderMeta {
		ShaderMeta {
			images: vec!["tex".to_string()],
			uniforms: UniformBlockLayout {
				uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
			},
		}
	}

	#[repr(C)]
	pub struct Uniforms {
		pub offset: (f32, f32),
	}
}
