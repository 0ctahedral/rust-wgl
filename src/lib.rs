#![macro_use]
mod webmacros;
mod webutils;
mod engine;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use js_sys::Date;
use std::cell::RefCell;
use std::rc::Rc;

// helper function for requesting animation frames
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  webutils::window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  setup_page(&webutils::document(), &webutils::body())?;
  let canvas = webutils::document().get_element_by_id("my-canvas")
    .expect("Canvas not found");

  // give our client the gl rendering context
  let gl = get_webgl_ctx(canvas).unwrap();
  let mut c = engine::Engine::new(gl);

  // set the frame rate to 60 cuz why not
  c.set_frame_rate(6 as f64);

  let f = Rc::new(RefCell::new(None));
  // setup g as the render loop
  let g = f.clone();
  let mut lastupdate = Date::now();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    let d = Date::now() - lastupdate;
    if d > c.get_frame_thresh() {
      lastupdate = Date::now();
      c.render();
    }
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());

  webutils::log("loggin");
  Ok(())
}

pub fn get_webgl_ctx(canvas: Element)
                     -> Result<WebGlRenderingContext, JsValue> {

  let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()?;

  let ctx = canvas.get_context("webgl")?
    .unwrap().dyn_into::<WebGlRenderingContext>()?;
  Ok(ctx)
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

