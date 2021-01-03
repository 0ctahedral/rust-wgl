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

  let mut r = render::Renderer::new(webutils::get_webgl_ctx());

  let f = Rc::new(RefCell::new(None));
  // setup g as the render loop
  let g = f.clone();
  let mut lastupdate = Date::now();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    let d = Date::now() - lastupdate;
    if d > e.get_frame_thresh() as f64 {
      lastupdate = Date::now();
      // e.update(d as f32,
      //          webutils::canvas().width() as f32,
      //          webutils::canvas().height() as f32,
      //          );
      draw(&mut r);
    }
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());
  Ok(())
}

fn draw(r: &mut render::Renderer) {
  r.pre_draw_frame();
  r.set_fill_color(0., 1., 0., 1.);
  rect(r, 10, 10, 100, 100);
}

fn rect(r: &render::Renderer,_x: i32, _y: i32, _w: u32, _h:u32) { 
  // TODO: ask renderer if it has this shape cached
  // or to draw the given vertices and indices

  // this is with the center
  // let vertices: Vec<f32> = vec![
  //   -1.,  1.,  0.,
  //    1.,  1.,  0.,
  //    1., -1., 0.,
  //   -1., -1., 0.,
  // ];

  // with the left corner
  let vertices: Vec<f32> = vec![
    0.,  0.,  0.,
     2.,  0.,  0.,
     2., -2., 0.,
    0., -2., 0.,
  ];

  let indices: Vec<u16> = vec![
    0, 1, 3,
    1, 3, 2
  ];

  let x = _x as f32;
  let y = _y as f32;

  let sx = _w as f32;
  let sy = _h as f32;

  let t: [f32; 16] = [
    sx, 0., 0., 0.,
    0., sy, 0., 0.,
    0., 0., 1., 0.,
    x,  y,  1.,  1.,
  ];

  let bi = r.create_buffer_info(vertices, indices, t);
  r.draw_buffers(bi);
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
