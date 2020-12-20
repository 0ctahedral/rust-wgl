#[macro_use]
mod webmacros;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};
// #[wasm_bindgen]
// extern {
//     // import the alert() js function
//     pub fn alert(s: &str);
// }
// Called when the wasm module is instantiated

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  let document = get_document()?;
  let body = document.body().unwrap();

  setup_page(&document, &body)?;

  Ok(())
}

// Get the document of the current page
fn get_document() -> Result<Document, JsValue> {
  let window = web_sys::window().expect("could not find window");
  let document = window.document().expect("could not find document");
  Ok(document)
}

// Add a title and a canvas that is green
fn setup_page(doc: &Document, body: &HtmlElement)
         -> Result<(), JsValue> {
  append_text_element_attrs!(doc, body, "h1", "This will have a silly canvas below",);
  append_element_attrs!(doc, body, "canvas",
                        ("id", "da-canvas"),
                        ("width", "500"),
                        ("height", "500"),
                        ("style", "border:1px solid")
  );
  Ok(())
}

