//! WebGLRenderbuffer and methods
use glenum::{Attachment, FramebufferKind, RenderbufferKind};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes WebGLRSRenderbuffer object.
    pub fn create_renderbuffer(&self) -> WebGLRSRenderbuffer {
        WebGLRSRenderbuffer {
            context: self,
            inner: self._create_renderbuffer(),
        }
    }
}

/// The `WebGLRenderbuffer` interface is represents a buffer that can contain an image, or can be source
/// or target of an rendering operation.
pub struct WebGLRSRenderbuffer<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLRenderbuffer,
}

impl<'ctx> WebGLRSRenderbuffer<'ctx> {
    /// Deletes the `WebGLRSRenderbuffer` object
    pub fn delete(self) {
        self.context._delete_renderbuffer(self.inner);
    }

    /// Returns true if the `WebGLRSRenderbuffer` object is valid and false otherwise.
    pub fn is_valid(&self) -> bool {
        self.context._is_renderbuffer(&self.inner)
    }

    /// Binds the `WebGLRSRenderbuffer` object to the given target.
    ///
    /// # Arguments
    /// * `target` - specifying the binding point.
    pub fn bind(&self, target: RenderbufferKind) {
        self.context._bind_renderbuffer(target, &self.inner);
    }

    /// Attaches this `WebGLRSRenderbuffer` object to a framebuffer.
    ///
    /// # Arguments
    /// * `fb_target` - specifying the binding point (target) for the framebuffer.
    /// * `attachment` - specifying the attachment point for the render buffer.
    /// * `rb_target` - specifying the binding point (target) for the render buffer.
    pub fn attach_framebuffer(
        &self,
        fb_target: FramebufferKind,
        attachment: Attachment,
        rb_target: RenderbufferKind,
    ) {
        self.context
            ._framebuffer_renderbuffer(fb_target, attachment, rb_target, &self.inner);
    }
}

/// Bindings for WebGLRenderbuffer
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    type WebGLRenderbuffer;
    /// Binding for `WebGLRenderingContext.createRenderbuffer()`
    #[wasm_bindgen(method, js_name = createRenderbuffer)]
    fn _create_renderbuffer(this: &WebGL2RenderingContext) -> WebGLRenderbuffer;

    /// The `WebGLRenderingContext.framebufferRenderbuffer()` method of the WebGL API attaches a WebGLRenderbuffer object
    /// to a WebGLFramebuffer object.
    #[wasm_bindgen(method, js_name = framebufferRenderbuffer)]
    fn _framebuffer_renderbuffer(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        attachment: Attachment,
        renderbuffertarget: RenderbufferKind,
        renderbuffer: &WebGLRenderbuffer,
    );

    /// The `WebGLRenderingContext.bindRenderbuffer()` method of the WebGL API binds a given WebGLRenderbuffer
    /// to a target, which must be `gl.RENDERBUFFER`.
    #[wasm_bindgen(method, js_name = bindRenderbuffer)]
    fn _bind_renderbuffer(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        renderbuffer: &WebGLRenderbuffer,
    );

    /// The `WebGLRenderingContext.deleteRenderbuffer()` method of the WebGL API deletes a given WebGLRenderbuffer
    /// object. This method has no effect if the render buffer has already been deleted.
    #[wasm_bindgen(method, js_name = deleteRenderbuffer)]
    fn _delete_renderbuffer(this: &WebGL2RenderingContext, renderbuffer: WebGLRenderbuffer);

    /// The `WebGLRenderingContext.isRenderbuffer()` method of the WebGL API returns true if the passed
    /// WebGLRenderbuffer is valid and false otherwise.
    #[wasm_bindgen(method, js_name = isRenderbuffer)]
    fn _is_renderbuffer(this: &WebGL2RenderingContext, renderbuffer: &WebGLRenderbuffer) -> bool;

}
