//use wasm_bindgen::prelude::*;
use web_sys::*;
use crate::render::*;
use crate::model::*;

// commands
pub enum Command {
  Clear(u32), // clear the screen to this color
  FillColor(u32), // change the fill color
  FillColorA(u32, f32), // change the fill color
  Draw(Model), // draw Model to the screen
  // DrawToBuff(), // draw Model to specific buffer
}

// #[wasm_bindgen]
pub struct Engine {
  renderer: Renderer,
  // miliseconds elapsed before the next frame can be drawn
  frame_thresh: f32,
  // state
  width: f32,
  height: f32,
  bottom: f32,
  top: f32,
  left: f32,
  right: f32,

  commands: Vec<Command>,
}

// #[wasm_bindgen]
impl Engine {
  // #[wasm_bindgen(constructor)]
  pub fn new(gl: WebGlRenderingContext) -> Self {
    Self {
      // default to 30 fps
      frame_thresh: 1000.0 / 30.0,
      renderer: Renderer::new(gl),
      width: 0.,
      height: 0.,
      bottom: 0.,
      top: 0.,
      left: 0.,
      right: 0.,
      commands: vec![],
    }
  }

  // update the state of the engine and its current models
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

    // update all models transforms
    // create list of commands to feed to renderer
  }

  // render all models and renderer state changes
  pub fn render(&mut self) {
    // makes writing stuff a lot shorter
    let ren = &mut self.renderer;
    // draw each model and update renderer state
    for c in &self.commands {
      match c {
        Command::Draw(m) => {
          let bw = ren.create_buffer_wrap(&m);
          ren.draw_buffers(bw, &m);
        },
        Command::Clear(col) => {
          let (r, g, b) = rgb_from_u32(*col);
          ren.clear(r, g, b);
        },
        Command::FillColor(col) => {
          let (r, g, b) = rgb_from_u32(*col);
          ren.set_fill_color(r, g, b, 1.0);
        },
        Command::FillColorA(col, a) => {
          let (r, g, b) = rgb_from_u32(*col);
          ren.set_fill_color(r, g, b, *a);
        },
      }
    }
  }

  pub fn add_model(&mut self, m: Model) {
    self.commands.push(Command::Draw(m));
  }

  pub fn clear(&mut self, c: u32) {
    self.commands.push(Command::Clear(c))
  }

  pub fn change_color(&mut self, c: u32) {
    self.commands.push(Command::FillColor(c));
  }

  pub fn set_frame_rate(&mut self, fps: f32) {
    self.frame_thresh = 1000.0 / fps;
  }

  pub fn get_frame_thresh(&self) -> f32{
    self.frame_thresh
  }
}

fn rgb_from_u32(c: u32) -> (f32, f32, f32) {
  let r = c & 0xff;
  let g = c >> 8 & 0xff;
  let b = c >> 16 & 0xff;

  (r as f32 / 256.,
   g as f32 / 256.,
   b as f32 / 256.)
}
