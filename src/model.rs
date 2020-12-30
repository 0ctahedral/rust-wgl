use web_sys::*;
use crate::shaders::funcs::*;
use crate::shaders::*;
use crate::webutils::*;

pub struct Model {
  pub program: WebGlProgram,
  pub vertices: Vec<f32>,
  pub indices: Vec<u16>,
  pub u_color: WebGlUniformLocation,
  pub u_opacity: WebGlUniformLocation,
  pub u_transform: WebGlUniformLocation,
}

impl Model {
  pub fn new(
    ctx: &WebGlRenderingContext,
    vert: &str,
    frag: &str,
    vertices: Vec<f32>,
    indices: Vec<u16>,
  ) -> Result<Self, String>{
    let vert_shader = compile_shader(
      ctx,
      WebGlRenderingContext::VERTEX_SHADER,
      vert
    )?;

    let frag_shader = compile_shader(
      ctx,
      WebGlRenderingContext::FRAGMENT_SHADER,
      frag
    )?;

    let p = link_program(ctx, &vert_shader, &frag_shader)?;

    Ok(Self {
      u_color: ctx.get_uniform_location(&p, "uColor").unwrap(),
      u_opacity: ctx.get_uniform_location(&p, "uOpacity").unwrap(),
      u_transform: ctx.get_uniform_location(&p, "uTransform").unwrap(),
      program: p,
      vertices: vertices,
      indices: indices,
    })
  }
}

pub fn square(ctx: &WebGlRenderingContext) -> Model {
  let model = Model::new(ctx,
                         vertex::SIMPLE,
                         fragment::SIMPLE,
                         vec![
                           -0.5, 0.5, 0.0,
                           0.5, 0.5, 0.0,
                           0.5, -0.5, 0.0,
                           -0.5, -0.5, 0.0,
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
