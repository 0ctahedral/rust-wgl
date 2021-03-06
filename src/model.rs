use crate::matrix;
use crate::webutils::*;

pub struct Model {
  // TODO: Should these just be slices?
  pub vertices: Vec<f32>,
  pub indices: Vec<u16>,
  pub transform: [f32; 16],
}

impl Model {
  pub fn new(
    vertices: Vec<f32>,
    indices: Vec<u16>,
    transform: [f32; 16],
  ) -> Self {
    Self {
      vertices: vertices,
      indices: indices,
      transform: transform,
    }
  }

}

pub fn rect(_x: i32, _y: i32, _w: u32, _h:u32) -> Model { 
  // TODO: add engine switch for if this should be used
  // with the center
  // let vertices: Vec<f32> = vec![
  //   -0.5,  0.5,  0.,
  //    0.5,  0.5,  0.,
  //    0.5, -0.5, 0.,
  //   -0.5, -0.5, 0.,
  // ];

  // with the top left corner
  let vertices: Vec<f32> = vec![
    0.,  0.,  0.,
    1.,  0.,  0.,
    1., 1., 0.,
    0., 1., 0.,
  ];

  let indices: Vec<u16> = vec![
    0, 3, 1,
    1, 3, 2
  ];

  let x = _x as f32;
  let y = _y as f32;

  let sx = _w as f32;
  let sy = _h as f32;

  let t2 = matrix::mult(
    matrix::tranlate(x, y, 0.),
    matrix::scale(sx, sy, 1.)
  );


  Model::new(vertices, indices, t2)
}
