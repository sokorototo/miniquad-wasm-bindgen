use miniquad_wasm_bindgen::*;

#[repr(C)]
struct Vertex {
	pos: [f32; 2],
	color: [u8; 4],
}

struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
	ctx: Box<dyn RenderingBackend>,
}

impl Stage {
	pub fn new() -> Stage {
		let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

		#[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : [ -0.5, -0.5 ], color: [0xFF, 0, 0, 0xFF] },
            Vertex { pos : [  0.5, -0.5 ], color: [0, 0xFF, 0, 0xFF] },
            Vertex { pos : [  0.0,  0.5 ], color: [0, 0, 0xFF, 0xFF] },
        ];
		let vertex_buffer = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices));

		let indices: [u16; 3] = [0, 1, 2];
		let index_buffer = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices));

		let bindings = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer: index_buffer,
			images: vec![],
		};

		let shader = ctx.new_shader(ShaderSource::new(shader::VERTEX, shader::FRAGMENT), shader::meta()).unwrap();

		let pipeline = ctx.new_pipeline(
			&[BufferLayout::default()],
			&[VertexAttribute::new("in_pos", VertexFormat::Float2), VertexAttribute::new("in_color", VertexFormat::Byte4)],
			shader,
			PipelineParams::default(),
		);

		Stage { pipeline, bindings, ctx }
	}
}

impl EventHandler for Stage {
	fn update(&mut self) {}

	fn draw(&mut self) {
		self.ctx.begin_default_pass(Default::default());

		self.ctx.apply_pipeline(&self.pipeline);
		self.ctx.apply_bindings(&self.bindings);
		self.ctx.draw(0, 3, 1);
		self.ctx.end_render_pass();

		self.ctx.commit_frame();
	}
}

fn main() {
	miniquad_wasm_bindgen::start(conf::Conf::default(), move || Box::new(Stage::new()));
}

mod shader {
	use miniquad_wasm_bindgen::*;

	pub const VERTEX: &str = r#"#version 300 es
    in vec2 in_pos;
    in lowp uvec4 in_color;
    out lowp vec4 color;
    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        color = vec4(in_color) / 255.0;
    }"#;

	pub const FRAGMENT: &str = r#"#version 300 es
    in lowp vec4 color;
    void main() {
        gl_FragColor = color;
    }"#;

	pub fn meta() -> ShaderMeta {
		ShaderMeta {
			images: vec![],
			uniforms: UniformBlockLayout { uniforms: vec![] },
		}
	}
}
