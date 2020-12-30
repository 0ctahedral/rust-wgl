use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/// Alert on a fatal error then end
pub fn ferr(s: &str) -> ! {
  alert(s);
  std::process::exit(0)
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn body() -> HtmlElement {
    document().body().expect("document should have a body")
}

// TODO: maybe ge canvas by specific id
// find a way to do this only once
pub fn canvas() -> HtmlCanvasElement {
  let cid = "my-canvas";
  document().get_element_by_id(cid)
    .expect(&format!("could not find canvas with id: {}", cid))
    .dyn_into::<HtmlCanvasElement>()
    .unwrap()
}

pub fn get_webgl_ctx() -> WebGlRenderingContext {
  let canvas = canvas();
  canvas.get_context("webgl")
    .unwrap()
    .expect("context not found")
    .dyn_into::<WebGlRenderingContext>()
    .unwrap()
}
