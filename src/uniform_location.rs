//! WebGLUniformLocation and methods
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct WebGLRSUniformLocation<'ctx> {
    pub(crate) context: &'ctx WebGL2RenderingContext,
    pub(crate) inner: WebGLUniformLocation,
}

impl<'ctx> WebGLRSUniformLocation<'ctx> {}

/// WebGLUniformLocation
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    /// FIXME: would prefer this to be pub(crate) but does not seem to work, not sure if it is a limitation of wasm_bindgen
    /// or if it is just not possible in rust.
    #[derive(Clone)]
    pub type WebGLUniformLocation;

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1f)]
    pub fn uniform_1f(this: &WebGL2RenderingContext, location: &WebGLUniformLocation, v0: f32);

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2f)]
    pub fn uniform_2f(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: f32,
        v1: f32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3f)]
    pub fn uniform_3f(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: f32,
        v1: f32,
        v2: f32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4f)]
    pub fn uniform_4f(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1i)]
    pub fn uniform_1i(this: &WebGL2RenderingContext, location: &WebGLUniformLocation, v0: i32);

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2i)]
    pub fn uniform_2i(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: i32,
        v1: i32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3i)]
    pub fn uniform_3i(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: i32,
        v1: i32,
        v2: i32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4i)]
    pub fn uniform_4i(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1fv)]
    pub fn uniform_1fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2fv)]
    pub fn uniform_2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3fv)]
    pub fn uniform_3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4fv)]
    pub fn uniform_4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1iv)]
    pub fn uniform_1iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2iv)]
    pub fn uniform_2iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3iv)]
    pub fn uniform_3iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniform[1234][fi][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4iv)]
    pub fn uniform_4iv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<i32>,
    );

    /// The `WebGLRenderingContext.uniformMatrix[234]fv()` methods of the WebGL API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix2fv)]
    pub fn uniform_matrix_2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniformMatrix[234]fv()` methods of the WebGL API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix3fv)]
    pub fn uniform_matrix_3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        value: Vec<f32>,
    );

    /// The `WebGLRenderingContext.uniformMatrix[234]fv()` methods of the WebGL API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix4fv)]
    pub fn uniform_matrix_4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        value: Vec<f32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1ui)]
    pub fn uniform_1ui(this: &WebGL2RenderingContext, location: &WebGLUniformLocation, v0: u32);

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2ui)]
    pub fn uniform_2ui(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: u32,
        v1: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3ui)]
    pub fn uniform_3ui(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: u32,
        v1: u32,
        v2: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4ui)]
    pub fn uniform_4ui(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform1uiv)]
    pub fn uniform_1uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform2uiv)]
    pub fn uniform_2uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform3uiv)]
    pub fn uniform_3uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    /// The `WebGL2RenderingContext.uniform[1234][uif][v]()` methods of the WebGL API specify values of uniform variables.
    #[wasm_bindgen(method, js_name = uniform4uiv)]
    pub fn uniform_4uiv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        value: Vec<u32>,
    );

    //TODO all uniform vector methods with optional srcoffset and srclength

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix2x3fv)]
    pub fn uniform_matrix_2x3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix2x4fv)]
    pub fn uniform_matrix_2x4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix3x2fv)]
    pub fn uniform_matrix_3x2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix3x4fv)]
    pub fn uniform_matrix_3x4fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix4x2fv)]
    pub fn uniform_matrix_4x2fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );

    /// The `WebGL2RenderingContext.uniformMatrix[234]x[234]fv()` methods of the WebGL 2 API specify matrix values for
    /// uniform variables.
    #[wasm_bindgen(method, js_name = uniformMatrix4x3fv)]
    pub fn uniform_matrix_4x3fv(
        this: &WebGL2RenderingContext,
        location: &WebGLUniformLocation,
        transpose: bool,
        data: Vec<f32>,
        srcOffset: u32,
        srcLength: u32,
    );
}
