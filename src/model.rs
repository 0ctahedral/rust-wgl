use web_sys::*;
use crate::webutils::*;
use crate::shaders::*;

pub struct Model {
  // TODO: Should these just be slices?
  pub vertices: Vec<f32>,
  pub indices: Vec<u16>,
  pub color: [f32; 3],
  pub location: [i32; 3],
  pub scale: [u32; 3],

  // TODO: Should the program be its own type?
}

impl Model {
  pub fn new(
    ctx: &WebGlRenderingContext,
    location: [i32; 3],
    scale: [u32; 3],
    color: [f32; 3],
    vert: &str,
    frag: &str,
    vertices: Vec<f32>,
    indices: Vec<u16>,
  ) -> Result<Self, String>{

    Ok(Self {
      vertices: vertices,
      indices: indices,
      color: color,
      location: location,
      scale: scale,
    })
  }
}

/// create a rectangle with a size given in pixels
pub fn rect(
  ctx: &WebGlRenderingContext,
  x: i32,
  y: i32,
  w: u32,
  h: u32,
  color: [f32; 3]
) -> Model {
  let model = Model::new(ctx,
                         [x, y, 0],
                         [w, h, 0],
                         color,
                         vertex::SIMPLE,
                         fragment::SIMPLE,
                         vec![
                           -1., 1., 0.0,
                           1., 1., 0.0,
                           1., -1., 0.0,
                           -1., -1., 0.0,
                         ],
                         vec![
                           0, 1, 3,
                           3, 1, 2,
                         ],
  );

  match model {
    Ok(m) => m,
    Err(s) => ferr(&s),
  }
}
