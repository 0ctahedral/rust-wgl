use web_sys::*;
use crate::shaders::funcs::*;

pub struct Program {
  pub program: WebGlProgram,
  pub u_color: WebGlUniformLocation,
  pub u_opacity: WebGlUniformLocation,
  pub u_transform: WebGlUniformLocation,
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
      u_color: ctx.get_uniform_location(&p, "uColor").unwrap(),
      u_opacity: ctx.get_uniform_location(&p, "uOpacity").unwrap(),
      u_transform: ctx.get_uniform_location(&p, "uTransform").unwrap(),
      program: p,
    })
  }
}
