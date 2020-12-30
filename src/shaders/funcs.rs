use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};

pub fn link_program(
  ctx: &WebGlRenderingContext,
  v: &WebGlShader,
  f: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = ctx.create_program()
    .ok_or_else(|| String::from("could not create new program"))?;

  ctx.attach_shader(&program, v);
  ctx.attach_shader(&program, f);
  ctx.link_program(&program);

  if ctx
    .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(ctx
        .get_program_info_log(&program)
        .unwrap_or_else(
          || String::from("Unknown error creating program object")
        )
    )
  }
}

pub fn compile_shader(
  ctx: &WebGlRenderingContext,
  shader_type: u32,
  src: &str
) -> Result<WebGlShader, String> {
  let shader = ctx.create_shader(shader_type)
    .ok_or_else(|| String::from("could not create new shader"))?;

  ctx.shader_source(&shader, src);
  ctx.compile_shader(&shader);

  if ctx
    .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(ctx
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")))
  }
}
