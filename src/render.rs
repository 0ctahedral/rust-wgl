use web_sys::WebGlRenderingContext as GL;
pub trait Renderer {
  // add object to render queue
  // fn add_to_queue(&self, item: ());
  // render all objects in queue
  fn render(&self);
}

// WebGl implementation of renderer
pub struct Wgl {
  ctx: GL,
  // will be a vector of models to draw
  queue: Vec<()>
}

impl Wgl {
  pub fn new(ctx: GL) -> Self {
    ctx.enable(GL::BLEND);
    ctx.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    ctx.clear_color(0.0, 0.0, 0.0, 1.0);
    ctx.clear_depth(1.0);
    Self {
      ctx: ctx,
      queue: vec!(),
    }
  }
}

impl Renderer for Wgl {
  fn render(&self) {
    self.ctx.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT)
  }
}
