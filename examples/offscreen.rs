use miniquad_wasm_bindgen::*;

use glam::{vec3, Mat4};

struct Stage {
	display_pipeline: Pipeline,
	display_bind: Bindings,
	offscreen_pipeline: Pipeline,
	offscreen_bind: Bindings,
	offscreen_pass: RenderPass,
	rx: f32,
	ry: f32,
	ctx: Box<dyn RenderingBackend>,
}

impl Stage {
	pub fn new() -> Stage {
		let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

		let color_img = ctx.new_render_texture(TextureParams {
			width: 256,
			height: 256,
			format: TextureFormat::RGB8,
			..Default::default()
		});

		let depth_img = ctx.new_render_texture(TextureParams {
			width: 256,
			height: 256,
			format: TextureFormat::Depth,
			..Default::default()
		});

		let offscreen_pass = ctx.new_render_pass(color_img, Some(depth_img));

		#[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos               color                   uvs */
            -1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 0.0,
             1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 1.0,
            -1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 1.0,

            -1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 0.0,
             1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 1.0,
            -1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 1.0,

            -1.0, -1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 0.0,
            -1.0,  1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 0.0,
            -1.0,  1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 1.0,
            -1.0, -1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 1.0,

             1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 0.0,
             1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 1.0,
             1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 1.0,

            -1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 0.0,
            -1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 0.0,
             1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 1.0,
             1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 1.0,

            -1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 0.0,
            -1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 1.0,
             1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 1.0
        ];

		let vertex_buffer = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(vertices));

		#[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];

		let index_buffer = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(indices));

		let offscreen_bind = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer: index_buffer,
			images: vec![],
		};

		let display_bind = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer,
			images: vec![color_img],
		};

		let default_shader = ctx.new_shader(ShaderSource::new(display_shader::VERTEX, display_shader::FRAGMENT), display_shader::meta()).unwrap();
		let display_pipeline = ctx.new_pipeline(
			&[BufferLayout::default()],
			&[
				VertexAttribute::new("in_pos", VertexFormat::Float3),
				VertexAttribute::new("in_color", VertexFormat::Float4),
				VertexAttribute::new("in_uv", VertexFormat::Float2),
			],
			default_shader,
			PipelineParams {
				depth_test: Comparison::LessOrEqual,
				depth_write: true,
				..Default::default()
			},
		);

		let offscreen_shader = ctx.new_shader(ShaderSource::new(offscreen_shader::VERTEX, offscreen_shader::FRAGMENT), offscreen_shader::meta()).unwrap();
		let offscreen_pipeline = ctx.new_pipeline(
			&[BufferLayout { stride: 36, ..Default::default() }],
			&[VertexAttribute::new("in_pos", VertexFormat::Float3), VertexAttribute::new("in_color", VertexFormat::Float4)],
			offscreen_shader,
			PipelineParams {
				depth_test: Comparison::LessOrEqual,
				depth_write: true,
				..Default::default()
			},
		);

		Stage {
			display_pipeline,
			display_bind,
			offscreen_pipeline,
			offscreen_bind,
			offscreen_pass,
			rx: 0.,
			ry: 0.,
			ctx,
		}
	}
}

impl EventHandler for Stage {
	fn update(&mut self) {}

	fn draw(&mut self) {
		let cxt = self.ctx.as_mut();

		let (width, height) = {
			let (w, h) = window::screen_size();
			(w as f32, h as f32)
		};
		let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
		let view = Mat4::look_at_rh(vec3(0.0, 1.5, 3.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
		let view_proj = proj * view;

		self.rx += 0.01;
		self.ry += 0.03;
		let model = Mat4::from_rotation_y(self.ry) * Mat4::from_rotation_x(self.rx);

		let vs_params = display_shader::Uniforms { mvp: view_proj * model };

		// the offscreen pass, rendering a rotating, untextured cube into a render target image
		cxt.begin_pass(Some(self.offscreen_pass), PassAction::clear_color(1.0, 1.0, 1.0, 1.0));
		cxt.apply_pipeline(&self.offscreen_pipeline);
		cxt.apply_bindings(&self.offscreen_bind);
		cxt.apply_uniforms(UniformsSource::table(&vs_params));
		cxt.draw(0, 36, 1);
		cxt.end_render_pass();

		// and the display-pass, rendering a rotating, textured cube, using the
		// previously rendered offscreen render-target as texture
		cxt.begin_default_pass(PassAction::clear_color(0.0, 0., 0.45, 1.));
		cxt.apply_pipeline(&self.display_pipeline);
		cxt.apply_bindings(&self.display_bind);
		cxt.apply_uniforms(UniformsSource::table(&vs_params));
		cxt.draw(0, 36, 1);
		cxt.end_render_pass();

		cxt.commit_frame();
	}
}

fn main() {
	start(conf::Conf::default(), move || Box::new(Stage::new()));
}

mod display_shader {
	use miniquad_wasm_bindgen::*;

	pub const VERTEX: &str = r#"#version 100
    attribute vec4 in_pos;
    attribute vec4 in_color;
    attribute vec2 in_uv;

    varying lowp vec4 color;
    varying lowp vec2 uv;

    uniform mat4 mvp;

    void main() {
        gl_Position = mvp * in_pos;
        color = in_color;
        uv = in_uv;
    }
    "#;

	pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 uv;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = color * texture2D(tex, uv);
    }
    "#;

	pub fn meta() -> ShaderMeta {
		ShaderMeta {
			images: vec!["tex".to_string()],
			uniforms: UniformBlockLayout {
				uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
			},
		}
	}

	#[repr(C)]
	pub struct Uniforms {
		pub mvp: glam::Mat4,
	}
}

mod offscreen_shader {
	use miniquad_wasm_bindgen::*;

	pub const VERTEX: &str = r#"#version 100
    attribute vec3 in_pos;
    attribute vec4 in_color;

    varying lowp vec4 color;

    uniform mat4 mvp;

    void main() {
        gl_Position = mvp * vec4(in_pos, 1.0);
        color = in_color;
    }
    "#;

	pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }
    "#;

	pub fn meta() -> ShaderMeta {
		ShaderMeta {
			images: vec![],
			uniforms: UniformBlockLayout {
				uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
			},
		}
	}
}
