use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;

use crate::model::*;

pub trait Renderer {
  // add object to render queue
  fn add_to_queue(&mut self, item: Model);
  // render all objects in queue
  fn render(&self,
            bottom: f32,
            top: f32,
            left: f32,
            right: f32,
            height: f32,
            width: f32);
}

// WebGl implementation of renderer
pub struct Wgl {
  ctx: GL,
  vert_buff: WebGlBuffer,
  ind_buff: WebGlBuffer,
  // will be a vector of models to draw
  queue: Vec<Model>,
}

impl Wgl {
  pub fn new(ctx: GL) -> Self {
    ctx.enable(GL::BLEND);
    ctx.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    ctx.clear_color(0.0, 0.0, 0.0, 1.0);
    ctx.clear_depth(1.0);
    // create gl buffer
    let v_buff = ctx.create_buffer()
      .ok_or("failed to create vert buffer").unwrap();
    let i_buff = ctx.create_buffer()
      .ok_or("failed to create index buffer").unwrap();
    Self {
      ctx: ctx,
      vert_buff: v_buff,
      ind_buff: i_buff,
      queue: vec!(),
    }
  }
}

impl Renderer for Wgl {
  fn render(&self,
            bottom: f32,
            top: f32,
            left: f32,
            right: f32,
            height: f32,
            width: f32,
  ) {
    self.ctx.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    for m in &self.queue {
      // use our shaders
      self.ctx.use_program(Some(&m.program));

      // add uniforms
      // bright green color
      self.ctx.uniform4f(
        Some(&m.u_color),
        1.,
        1.,
        0.,
        1.,
      );

      // full opacity
      self.ctx.uniform1f(Some(&m.u_opacity), 1.);

      // setup position
      let tm = translation_matrix(
        2. * left / width - 1.,
        2. * bottom / height - 1.,
        0.
      );

      let sm = scaling_matrix(
        2. * (right - left) / width,
        2. * (top - bottom) / height,
        0.,
      );

      let transform = mult_matrix_4(sm, tm);

      self.ctx.uniform_matrix4fv_with_f32_array(
        Some(&m.u_transform),
        false,
        &transform,
      );


      // bind buffer for drawing
      self.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vert_buff));

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

      // self.ctx.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.ind_buff));
      self.ctx.draw_elements_with_i32(
        GL::TRIANGLES, m.indices.len() as i32, GL::UNSIGNED_SHORT, 0);
      // self.ctx.draw_arrays(GL::LINES, 0,
      // (m.vertices.len() / 3) as i32)
    }
  }

  fn add_to_queue(&mut self, item: Model) {
    // convert our buffer to a js buffer
    // TODO: consider moving to utils
    let vert_arr = create_wasm_f32_array(&item.vertices);

    // bind our buffer for adding vertices
    self.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vert_buff));
    self.ctx.buffer_data_with_array_buffer_view(
      GL::ARRAY_BUFFER,
      &vert_arr,
      GL::STATIC_DRAW);

    let ind_arr = create_wasm_u16_array(&item.indices);
    self.ctx.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.ind_buff));
    self.ctx.buffer_data_with_array_buffer_view(
      GL::ELEMENT_ARRAY_BUFFER,
      &ind_arr,
      GL::STATIC_DRAW);

    self.queue.push(item);
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

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
  [
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    tx, ty, tz, 1.,
  ]
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
  [
    sx, 0., 0., 0.,
    0., sy, 0., 0.,
    0., 0., sz, 0.,
    0., 0., 0., 1.,
  ]
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return_var
}
