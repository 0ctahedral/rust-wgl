#![macro_use]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use js_sys::Date;
use std::cell::RefCell;
use std::rc::Rc;

pub mod webmacros;
pub mod webutils;
pub mod model;
pub mod engine;
pub mod render;
pub mod shaders;

// helper function for requesting animation frames
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  webutils::window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  setup_page(&webutils::document(), &webutils::body())?;

  // give our client the gl rendering context
  let mut e = engine::Engine::new(webutils::get_webgl_ctx());

  // set the frame rate to 60 cuz why not
  e.set_frame_rate(60.);

  let square = model::square(&webutils::get_webgl_ctx());
  e.pipeline.add_to_queue(square);

  let f = Rc::new(RefCell::new(None));
  // setup g as the render loop
  let g = f.clone();
  let mut lastupdate = Date::now();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    let d = Date::now() - lastupdate;
    if d > e.get_frame_thresh() as f64 {
      lastupdate = Date::now();
      e.update(d as f32,
               webutils::canvas().width() as f32,
               webutils::canvas().height() as f32,
               );
    }
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());
  Ok(())
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
