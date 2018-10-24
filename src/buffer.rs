//! WebGLBuffer and methods
use glenum::{BufferBase, BufferKind};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates a new `WebGLRSBuffer` object which is used for storing data such as vertices or colors.
    pub fn create_buffer(&self) -> WebGLRSBuffer {
        WebGLRSBuffer {
            context: self,
            inner: self._create_buffer(),
        }
    }
}

/// Buffer which is used for storing data
///
/// The `WebGLBuffer` interface is part of the WebGL API and represents an opaque buffer object
/// storing data such as vertices or colors.
#[derive(Clone, Copy, Debug)]
pub struct WebGLRSBuffer<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLBuffer,
}

impl<'ctx> WebGLRSBuffer<'ctx> {
    /// Deletes this `WebGLRSBuffer`
    pub fn delete(self) {
        self.context._delete_buffer(self.inner);
    }

    /// Returns true is the `WebGLRSBuffer` object is valid.
    pub fn is_valid(&self) -> bool {
        self.context._is_buffer(&self.inner)
    }

    /// Binds the buffer to a given target
    ///
    /// # Arguments
    /// * `target` - an enum specifying the binding point.
    pub fn bind(&self, target: BufferKind) {
        self.context._bind_buffer(target, &self.inner);
    }

    /// Binds the `WebGLRSBuffer` to a given binding point (target) at a given index.
    ///
    /// # Arguments
    /// * `target` - an enum specifying the target for the bind operation.
    /// * `index` - the index of the target.
    pub fn bind_base(&self, target: BufferBase, index: u32) {
        self.context._bind_buffer_base(target, index, &self.inner);
    }

    /// Binds a range of the `WebGLRSBuffer` to a given binding point (target) at a given index.
    ///
    /// # Arguments
    /// * `target` - an enum specifying the target for the bind operation.
    /// * `index` - the index of the target.
    /// * `offset` - the starting offset.
    /// * `size` - the amount of data that can be read from the buffer.
    pub fn bind_range(&self, target: BufferBase, index: u32, offset: u32, size: u32) {
        self.context
            ._bind_buffer_range(target, index, &self.inner, offset, size);
    }
}

/// WebGLBuffer bindings
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    type WebGLBuffer;
    /// Binding for `WebGLRenderingContext.createBuffer()`
    #[wasm_bindgen(method, js_name = createBuffer)]
    fn _create_buffer(this: &WebGL2RenderingContext) -> WebGLBuffer;

    /// Binding for `WebGLRenderingContext.deleteBuffer()`
    #[wasm_bindgen(method, js_name = deleteBuffer)]
    fn _delete_buffer(this: &WebGL2RenderingContext, buffer: WebGLBuffer);

    /// Binding for  `WebGLRenderingContext.isBuffer()`
    #[wasm_bindgen(method, js_name = isBuffer)]
    fn _is_buffer(this: &WebGL2RenderingContext, buffer: &WebGLBuffer) -> bool;

    /// Binding for  `WebGLRenderingContext.bindBuffer()`
    #[wasm_bindgen(method, js_name = bindBuffer)]
    fn _bind_buffer(this: &WebGL2RenderingContext, target: BufferKind, buffer: &WebGLBuffer);

    /// Binding for `WebGL2RenderingContext.bindBufferBase()`
    #[wasm_bindgen(method, js_name = bindBufferBase)]
    fn _bind_buffer_base(
        this: &WebGL2RenderingContext,
        target: BufferBase,
        index: u32,
        buffer: &WebGLBuffer,
    );

    /// Binding `WebGL2RenderingContext.bindBufferRange()`
    #[wasm_bindgen(method, js_name = bindBufferRange)]
    fn _bind_buffer_range(
        this: &WebGL2RenderingContext,
        target: BufferBase,
        index: u32,
        buffer: &WebGLBuffer,
        offset: u32,
        size: u32,
    );
}
