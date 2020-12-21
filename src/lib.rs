#![macro_use]
mod webmacros;
mod webutils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use std::cell::RefCell;
use std::rc::Rc;
// #[wasm_bindgen]
// extern {
//     // import the alert() js function
//     pub fn alert(s: &str);
// }

// helper function for requesting animation frames
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  webutils::window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  setup_page(&webutils::document(), &webutils::body())?;

  let canvas = webutils::document().get_element_by_id("my-canvas")
    .expect("Canvas not found");

  let c = Client::new(canvas);

  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

    c.render();
    
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());
  Ok(())
}

// create public functions that we can then hook up
// to the events

#[wasm_bindgen]
pub struct Client {
  gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl Client {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas: Element) -> Self {
    let gl = init_webgl_ctx(canvas).unwrap();
    Self {
      gl: gl,
    }
  }

  pub fn update(&mut self, _time: f32, _height: f32, _width: f32)
                -> Result<(), JsValue> {
    Ok(())
  }

  pub fn render(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT)
  }
}

pub fn init_webgl_ctx(canvas: Element) -> Result<WebGlRenderingContext, JsValue> {
  let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()?;
  let gl: WebGlRenderingContext = canvas.get_context("webgl")?
    .unwrap().dyn_into()?;

  gl.enable(GL::BLEND);
  gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.clear_depth(1.0);

  Ok(gl)
}

// Add a title and a canvas that is green
fn setup_page(doc: &Document, body: &HtmlElement)
              -> Result<(), JsValue> {
  append_text_element_attrs!(doc, body, "h1",
                             "This will have a silly canvas below",);
  append_element_attrs!(doc, body, "canvas",
                        ("id", "my-canvas"),
                        ("width", "500"),
                        ("height", "500"),
                        ("style", "border:1px solid")
  );

  Ok(())
}

