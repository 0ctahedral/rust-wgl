use web_sys::*;
use crate::shaders::funcs::*;

// holds the shader program for the renderer
pub struct Program {
  pub program: WebGlProgram,
  pub u_color: WebGlUniformLocation,
  pub u_matrix: WebGlUniformLocation,
}

impl Program {
  pub fn new(
    ctx: &WebGlRenderingContext,
    vert: &str,
    frag: &str,
  )  -> Result<Self, String> {
    let vert_shader = compile_shader(
      &ctx,
      WebGlRenderingContext::VERTEX_SHADER,
      vert,
    ).unwrap();

    let frag_shader = compile_shader(
      &ctx,
      WebGlRenderingContext::FRAGMENT_SHADER,
      frag,
    ).unwrap();

    let p = link_program(&ctx, &vert_shader, &frag_shader).unwrap();

    Ok(Self {
      u_color: ctx.get_uniform_location(&p, "u_color").unwrap(),
      u_matrix: ctx.get_uniform_location(&p, "u_matrix").unwrap(),
      program: p,
    })
  }
}
