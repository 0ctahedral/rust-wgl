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
  // setup the page to have our canvas
  setup_page(&webutils::document(), &webutils::body())?;

  // give our client the gl rendering context
  let mut e = engine::Engine::new(webutils::get_webgl_ctx());

  // set the frame rate to 60 cuz why not
  e.set_frame_rate(60.);

  // draw two squares in magenta and green as a example
  e.clear(0x000000);
  e.change_color(0xff00ff);
  e.add_model(model::rect(10, 10, 100, 100));
  e.change_color(0x00ff00);
  e.add_model(model::rect(20, 20, 100, 100));

  // TODO: find a cleaner way of setting up the render loop

  // we need two references to the render function so that
  // one can call the other recursively to make the loop perpetual
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();
  let mut lastupdate = Date::now();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    let d = Date::now() - lastupdate;
    // only update and render if 
    if d > e.get_frame_thresh() as f64 {
      lastupdate = Date::now();
      e.update(d as f32,
               webutils::canvas().width() as f32,
               webutils::canvas().height() as f32,
               );
      e.render();
    }
    // recursive call to f
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  // set g as the animation frame function
  request_animation_frame(g.borrow().as_ref().unwrap());
  Ok(())
}

// Setup the canvas and title
fn setup_page(doc: &Document, body: &HtmlElement)
              -> Result<(), JsValue> {
  append_text_element_attrs!(doc, body, "h1",
                             "0ctalDraw",);
  append_element_attrs!(doc, body, "canvas",
                        ("id", "my-canvas"),
                        ("width", "500"),
                        ("height", "500")
  );

  // add a slider, later will control values of objects during update
  // append_element_attrs!(doc, body, "input",
  //                       ("type", "range"),
  //                       ("min", "0"),
  //                       ("max", "500"),
  //                       ("value", "0"),
  //                       ("class", "slider")
  // );

  Ok(())
}
