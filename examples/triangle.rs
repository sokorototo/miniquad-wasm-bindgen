use miniquad_wasm_bindgen::*;

#[repr(C)]
struct Vertex {
	pos: [f32; 2],
	color: [f32; 4],
}

struct Stage {
	pipeline: Pipeline,
	bindings: Bindings,
	backend: Box<dyn RenderingBackend>,
}

impl Stage {
	pub fn new() -> Stage {
		let mut backend: Box<dyn RenderingBackend> = window::new_rendering_backend();

		#[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : [ -0.5, -0.5 ], color: [1., 0., 0., 1.] },
            Vertex { pos : [  0.5, -0.5 ], color: [0., 1., 0., 1.] },
            Vertex { pos : [  0.0,  0.5 ], color: [0., 0., 1., 1.] },
        ];
		let vertex_buffer = backend.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices));

		let indices: [u16; 3] = [0, 1, 2];
		let index_buffer = backend.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices));

		let bindings = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer,
			images: vec![],
		};

		let shader = backend.new_shader(ShaderSource::new(shader::VERTEX, shader::FRAGMENT), shader::meta()).unwrap();

		let pipeline = backend.new_pipeline(
			&[BufferLayout::default()],
			&[VertexAttribute::new("in_pos", VertexFormat::Float2), VertexAttribute::new("in_color", VertexFormat::Float4)],
			shader,
			PipelineParams::default(),
		);

		Stage { pipeline, bindings, backend }
	}
}

impl EventHandler for Stage {
	fn update(&mut self) {}

	fn draw(&mut self) {
		self.backend.begin_default_pass(Default::default());

		self.backend.apply_pipeline(&self.pipeline);
		self.backend.apply_bindings(&self.bindings);
		self.backend.draw(0, 3, 1);
		self.backend.end_render_pass();

		self.backend.commit_frame();
	}
}

fn main() {
	let mut conf = conf::Conf::default();
	miniquad_wasm_bindgen::start(conf, move || Box::new(Stage::new()));
}

mod shader {
	use miniquad_wasm_bindgen::*;

	pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec4 in_color;

    varying lowp vec4 color;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        color = in_color;
    }"#;

	pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

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
