use wasm_bindgen::prelude::*;
use web_sys::*;
use std::boxed::Box;
use crate::render::*;

// #[wasm_bindgen]
pub struct Engine {
  // miliseconds elapsed before the next frame can be drawn
  frame_thresh: f64,
  // render pipeline
  pub pipeline: Box<dyn Renderer>,
}

// #[wasm_bindgen]
impl Engine {
  // #[wasm_bindgen(constructor)]
  pub fn new(gl: WebGlRenderingContext) -> Self {
    Self {
      // default to 30 fps
      frame_thresh: 1000.0 / 30.0,
      pipeline: Box::new(Wgl::new(gl))
    }
  }

  pub fn update(&mut self,
                _time: f32,
                _height: f32,
                _width: f32
  ) -> Result<(), JsValue> {
    Ok(())
  }

  pub fn set_frame_rate(&mut self, fps: f64) {
    self.frame_thresh = 1000.0 / fps;
  }

  pub fn get_frame_thresh(&self) -> f64{
    self.frame_thresh
  }
}
