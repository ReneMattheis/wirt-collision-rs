use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait DebugRenderable {
    fn debug_render(&self, ctx: Context, gl: &mut GlGraphics);
}