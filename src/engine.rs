use wasm_bindgen::prelude::*;
use web_sys::*;
use std::boxed::Box;
use crate::render::*;

// #[wasm_bindgen]
pub struct Engine {
  // miliseconds elapsed before the next frame can be drawn
  frame_thresh: f32,
  // render pipeline
  // pub pipeline: Box<dyn Renderer>,

  // state
  width: f32,
  height: f32,
  bottom: f32,
  top: f32,
  left: f32,
  right: f32,
}

// engine should contain all shaders and objects to draw

// #[wasm_bindgen]
impl Engine {
  // #[wasm_bindgen(constructor)]
  pub fn new(gl: WebGlRenderingContext) -> Self {
    Self {
      // default to 30 fps
      frame_thresh: 1000.0 / 30.0,
      // pipeline: Box::new(Wgl::new(gl)),
      width: 0.,
      height: 0.,
      bottom: 0.,
      top: 0.,
      left: 0.,
      right: 0.,
    }
  }

  pub fn update(&mut self,
                _time: f32,
                cheight: f32,
                cwidth: f32
  ) {
    self.width = cwidth;
    self.height = cheight;
    self.bottom = 0.;
    self.top = cheight;
    self.left = 0.;
    self.right = cwidth;

    // self.pipeline.render(
    //   self.bottom, self.top,
    //   self.left, self.right,
    //   self.height, self.width
    // )
  }

  pub fn set_frame_rate(&mut self, fps: f32) {
    self.frame_thresh = 1000.0 / fps;
  }

  pub fn get_frame_thresh(&self) -> f32{
    self.frame_thresh
  }
}
