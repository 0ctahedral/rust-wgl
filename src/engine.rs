use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct Engine {
  gl: WebGlRenderingContext,
  // miliseconds elapsed before the next frame can be drawn
  frame_thresh: f64,
}

#[wasm_bindgen]
impl Engine {

  #[wasm_bindgen(constructor)]
  pub fn new(gl: WebGlRenderingContext) -> Self {
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);

    Self {
      gl: gl,
      // default to 30 fps
      frame_thresh: 1000.0 / 30.0,
    }
  }

  pub fn update(&mut self,
                _time: f32,
                _height: f32,
                _width: f32
  ) -> Result<(), JsValue> {
    Ok(())
  }

  pub fn render(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT)
  }

  pub fn set_frame_rate(&mut self, fps: f64) {
    self.frame_thresh = 1000.0 / fps;
  }

  pub fn get_frame_thresh(&self) -> f64{
    self.frame_thresh
  }
}
