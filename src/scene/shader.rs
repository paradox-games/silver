//!  Silver3d web render engine:
//! shader module with VertexShader
//! and FragmentShader struct
//! and implementations

use web_sys::*;


pub struct VertexShader {
    source: &'static str,
}

impl VertexShader {
    pub fn init(source: &'static str) -> VertexShader {
        return VertexShader {
            source
        };
    }

    pub fn compile(&self,
                   ctx: &WebGlRenderingContext
    ) -> Result<WebGlShader, String> {
        let shader = ctx
                        .create_shader(WebGlRenderingContext::VERTEX_SHADER)
                        .ok_or_else(|| String::from("Unable to create vertex shader object"))?;
        ctx.shader_source(&shader, self.source);
        ctx.compile_shader(&shader);

        if ctx
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false) {
            return Ok(shader);
        } else {
            return Err(ctx
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating vertex shader")));
        }
    }
}

pub struct FragmentShader {
    source: &'static str,
}

impl FragmentShader {
    pub fn init(source: &'static str) -> FragmentShader {
        return FragmentShader {
            source
        };
    }

    pub fn compile(&self,
                   ctx: &WebGlRenderingContext
    ) -> Result<WebGlShader, String> {
        let shader = ctx
                        .create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
                        .ok_or_else(|| String::from("Unable to create fragment shader object"))?;
        ctx.shader_source(&shader, self.source);
        ctx.compile_shader(&shader);

        if ctx
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false) {
            return Ok(shader);
        } else {
            return Err(ctx
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating fragment shader")));
        }
    }
}


pub fn link(
    context: &WebGlRenderingContext,
    vert_shader: &VertexShader,
    frag_shader: &FragmentShader
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create program object"))?;

    context.attach_shader(&program, &vert_shader.compile(context).unwrap());
    context.attach_shader(&program, &frag_shader.compile(context).unwrap());
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(program);
    } else {
        return Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")));
    }
}