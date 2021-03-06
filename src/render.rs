use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;

use crate::webutils::*;
use crate::shaders::program::*;
use crate::model::*;
use crate::matrix;

// WebGl implementation of renderer
pub struct Renderer {
  ctx: GL,
  // shader program we are using for everything
  program: Program,
  width: f32,
  height: f32,
  fill_color: [f32; 4],
}

// contains draw info for a given object
// will be able to reuse for objects with same vertices
pub struct BufferWrap {
  // buffers for vertices and indices
  pub v_buff: WebGlBuffer,
  pub i_buff: WebGlBuffer,
  pub i_len: usize,
}

impl Renderer {
  pub fn new(ctx: GL) -> Self {
    // setup:
    ctx.enable(GL::BLEND);
    ctx.enable(GL::CULL_FACE);
    ctx.enable(GL::DEPTH_TEST);
    ctx.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    ctx.clear_color(0.0, 0.0, 0.0, 1.0);
    ctx.clear_depth(1.0);

    let w = canvas().width() as f32;
    let h = canvas().height() as f32;
    ctx.viewport(0, 0, w as i32, h as i32);

    // create the shader
    let p = Program::new(
      &ctx,
      include_str!("shaders/simple_vert.glsl"),
      include_str!("shaders/simple_frag.glsl")
    ).unwrap();

    Self {
      ctx: ctx,
      program: p,
      width: w,
      height: h,
      fill_color: [1., 1., 1., 1.],
    }
  }

  // clear the screen and make a background of this color
  pub fn clear(&self, r: f32, g: f32, b: f32) {
    self.ctx.clear_color(r, g, b, 1.0);
    self.ctx.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
  }

  // set the current fill color
  pub fn set_fill_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
    self.fill_color = [r, g, b, a];
  }

  // creates a buffer wrap for drawing a set of vertices
  pub fn create_buffer_wrap(
    &self,
    m: &Model
  ) -> BufferWrap {
    // create arrays for indices and vertices
    let vert_arr = create_wasm_f32_array(&m.vertices);
    let ind_arr = create_wasm_u16_array(&m.indices);

    // create buffers
    let v_buff = self.ctx.create_buffer()
      .ok_or("failed to create vert buffer").unwrap();
    let i_buff = self.ctx.create_buffer()
      .ok_or("failed to create index buffer").unwrap();

    // bind and fill buffers
    self.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&v_buff));
    self.ctx.buffer_data_with_array_buffer_view(
      GL::ARRAY_BUFFER,
      &vert_arr,
      GL::STATIC_DRAW,
    );

    self.ctx.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&i_buff));
    self.ctx.buffer_data_with_array_buffer_view(
      GL::ELEMENT_ARRAY_BUFFER,
      &ind_arr,
      GL::STATIC_DRAW,
    );

    BufferWrap {
      v_buff: v_buff,
      i_buff: i_buff,
      i_len: m.indices.len(),
    }
  }

  // draw a model with its given BufferWrap and transform
  pub fn draw_buffers(&self, bw: BufferWrap, m: &Model) {
    // use pixels instead of clip space
    self.ctx.viewport(0, 0, self.width as i32, self.height as i32);
    
    // use the shader
    self.ctx.use_program(Some(&self.program.program));
    // add uniforms

    // color
    self.ctx.uniform4f(
      Some(&self.program.u_color),
      self.fill_color[0],
      self.fill_color[1],
      self.fill_color[2],
      self.fill_color[3],
    );


    let matrix = matrix::mult(
      matrix::ortho(
        0., self.width,
        self.height, 0.,
        500., -500.),
      m.transform
    );

    self.ctx.uniform_matrix4fv_with_f32_array(
      Some(&self.program.u_matrix),
      false,
      &matrix,
    );


    // bind buffer for drawing
    self.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&bw.v_buff));

    // add position attribute
    self.ctx.vertex_attrib_pointer_with_i32(
      0, // which attribute
      3, // number of values per vertex
      GL::FLOAT, // type
      false, // normalize
      0, // stride
      0 // offset
    );
    self.ctx.enable_vertex_attrib_array(0);
    self.ctx.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&bw.i_buff));

    self.ctx.draw_elements_with_i32(
      GL::TRIANGLES,
      bw.i_len as i32,
      GL::UNSIGNED_SHORT,
      0
    );
  }

}

pub fn create_wasm_u16_array(v: &Vec<u16>) -> js_sys::Uint16Array {
  let mem_buff = wasm_bindgen::memory()
    .dyn_into::<WebAssembly::Memory>()
    .unwrap()
    .buffer();

  let ptr = v.as_ptr() as u32 / 2;

  js_sys::Uint16Array::new(&mem_buff)
    .subarray(ptr,
              ptr + v.len() as u32)
}

pub fn create_wasm_f32_array(v: &Vec<f32>) -> js_sys::Float32Array {
  let mem_buff = wasm_bindgen::memory()
    .dyn_into::<WebAssembly::Memory>()
    .unwrap()
    .buffer();

  let ptr = v.as_ptr() as u32 / 4;
  js_sys::Float32Array::new(&mem_buff)
    .subarray(ptr,
              ptr + v.len() as u32)
}
