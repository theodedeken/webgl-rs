//! WebGLFramebuffer and methods
use glenum::FramebufferKind;
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes a new `WebGLRSFramebuffer` object
    pub fn create_framebuffer(&self) -> WebGLRSFramebuffer {
        WebGLRSFramebuffer {
            context: self,
            inner: self._create_framebuffer(),
        }
    }
}

/// Collection of buffers to be used as a rendering destination
///
/// The `WebGLFramebuffer` interface is part of the WebGL API and represents a collection of buffers that
/// serve as a rendering destination.
pub struct WebGLRSFramebuffer<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLFramebuffer,
}

impl<'ctx> WebGLRSFramebuffer<'ctx> {
    /// Binds this buffer to a given target
    ///
    /// # Arguments
    /// * `target` - specifying the binding point.
    pub fn bind(&self, target: FramebufferKind) {
        self.context._bind_framebuffer(target, &self.inner);
    }

    /// Returns `true` if this is a valid `WebGLRSFramebuffer` object
    pub fn is_valid(&self) -> bool {
        self.context._is_framebuffer(&self.inner)
    }

    /// Deletes this `WebGLRSFramebuffer` object
    pub fn delete(self) {
        self.context._delete_framebuffer(self.inner);
    }
}

/// WebGLFramebuffer bindings
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    type WebGLFramebuffer;
    /// Binding for `WebGLRenderingContext.createFramebuffer()`
    #[wasm_bindgen(method, js_name = createFramebuffer)]
    fn _create_framebuffer(this: &WebGL2RenderingContext) -> WebGLFramebuffer;

    /// Binding for `WebGLRenderingContext.bindFramebuffer()`
    #[wasm_bindgen(method, js_name = bindFramebuffer)]
    fn _bind_framebuffer(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        framebuffer: &WebGLFramebuffer,
    );

    /// Binding for `WebGLRenderingContext.deleteFramebuffer()`
    #[wasm_bindgen(method, js_name = deleteFramebuffer)]
    fn _delete_framebuffer(this: &WebGL2RenderingContext, framebuffer: WebGLFramebuffer);

    /// Binding for `WebGLRenderingContext.isFramebuffer()`
    #[wasm_bindgen(method, js_name = isFramebuffer)]
    fn _is_framebuffer(this: &WebGL2RenderingContext, framebuffer: &WebGLFramebuffer) -> bool;
}
