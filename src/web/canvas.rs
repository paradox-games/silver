//!  Silver3d web render engine:
//! canvas module with WebGLCanvas struct
//! and implementations

use web_sys::*;

pub struct WebGLCanvas {
    id: String,
    canvas: web_sys::HtmlCanvasElement,
    ctx: WebGlRenderingContext,
}

impl WebGLCanvas {
    pub fn init(id: String) {
        let cv = web_sys::window().unwrap().document().unwrap().get_element_by_id(id).unwrap().dyn_into::<web_sys::HtmlCanvasElement>()?;

        return WebGLCanvas {
            id: id,
            canvas: cv,
            ctx: cv.get_context("webgl")?.unwrap().dyn_into::<WebGlRenderingContext>()?,
        };
    }
}