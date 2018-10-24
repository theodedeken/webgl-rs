//! VertextArrayObject and methods
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes a WebGLRSVertexArrayObject object that represents a vertex array object
    /// (VAO) pointing to vertex array data and which provides names for different sets of vertex data.
    pub fn create_vertex_array(&self) -> WebGLRSVertexArrayObject {
        WebGLRSVertexArrayObject {
            context: self,
            inner: self._create_vertex_array(),
        }
    }
}

/// VertexArrayObject
///
/// The WebGLVertexArrayObject interface is part of the WebGL 2 API, represents vertex array objects (VAOs)
/// pointing to vertex array data, and provides names for different sets of vertex data.
#[derive(Clone, Copy, Debug)]
pub struct WebGLRSVertexArrayObject<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLVertexArrayObject,
}

impl<'ctx> WebGLRSVertexArrayObject<'ctx> {
    /// Deletes the `WebGLRSVertexArrayObject` on the gpu and consumes itself.
    pub fn delete(self) {
        self.context._delete_vertex_array(self.inner);
    }

    /// Return true if this is a valid `WebGLRSVertexArrayObject` object.
    pub fn is_valid(&self) -> bool {
        self.context._is_vertex_array(&self.inner)
    }

    /// Binds this `WebGLRSVertexArrayObject` to the buffer.
    pub fn bind(&self) {
        self.context._bind_vertex_array(&self.inner);
    }
}

/// WebGLVerterArrayObject bindings
#[wasm_bindgen]
extern "C" {
    type WebGLVertexArrayObject;
    /// Binding for `WebGL2RenderingContext.createVertexArray()`
    #[wasm_bindgen(method, js_name = createVertexArray)]
    fn _create_vertex_array(this: &WebGL2RenderingContext) -> WebGLVertexArrayObject;

    /// Binding for `WebGL2RenderingContext.deleteVertexArray()`
    #[wasm_bindgen(method, js_name = deleteVertexArray)]
    fn _delete_vertex_array(this: &WebGL2RenderingContext, vertex_array: WebGLVertexArrayObject);

    /// Binding for `WebGL2RenderingContext.isVertexArray()`
    #[wasm_bindgen(method, js_name = isVertexArray)]
    fn _is_vertex_array(
        this: &WebGL2RenderingContext,
        vertex_array: &WebGLVertexArrayObject,
    ) -> bool;

    /// Binding for `WebGL2RenderingContext.bindVertexArray()`
    #[wasm_bindgen(method, js_name = bindVertexArray)]
    fn _bind_vertex_array(this: &WebGL2RenderingContext, vertex_array: &WebGLVertexArrayObject);
}
