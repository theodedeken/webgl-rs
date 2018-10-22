//! WebGLTexture and methods
use glenum::{Attachment, FramebufferKind, TextureBindPoint, TextureKind};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes a WebGLRSTexture
    pub fn create_texture(&self) -> WebGLRSTexture {
        WebGLRSTexture {
            context: self,
            inner: self._create_texture(),
        }
    }
}

/// The WebGLTexture interface is part of the WebGL API and represents an opaque texture object providing
/// storage and state for texturing operations.
pub struct WebGLRSTexture<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLTexture,
}

impl<'ctx> WebGLRSTexture<'ctx> {
    /// Deletes the `WebGLRSTexture` object.
    pub fn delete(self) {
        self.context._delete_texture(self.inner);
    }

    /// Binds the `WebGLRSTexture` to a target
    ///
    /// # Arguments
    /// * `target` - specifying the binding point.
    pub fn bind(&self, target: TextureKind) {
        self.context._bind_texture(target, &self.inner);
    }

    /// Returns true if the `WebGLRSTexture` is valid and false otherwise.
    pub fn is_valid(&self) -> bool {
        self.context._is_texture(&self.inner)
    }

    /// Attaches this `WebGLRSTexture` object to a framebuffer.
    ///
    /// # Arguments
    /// * `target` - specifying the binding point.
    /// * `attachment` - specifying the attachment point for the texture.
    /// * `tex_target` - specifying the texture target.
    /// * `level` - specifying the mipmap level of the texture image to attach.
    pub fn attach_framebuffer(
        &self,
        target: FramebufferKind,
        attachment: Attachment,
        tex_target: TextureBindPoint,
        level: i32,
    ) {
        self.context
            ._framebuffer_texture_2d(target, attachment, tex_target, &self.inner, level);
    }

    /// Attaches a single layer of this `WebGLRSTexture` object to a framebuffer.TextureBindPoint
    ///
    /// # Arguments
    /// * `target` - specifying the binding point.
    /// * `attachment` - specifying the attachment point for the texture.
    /// * `level` - specifying the mipmap level of the texture image to attach.
    /// * `layer` - specifying the layer of the texture image to attach.
    pub fn attach_layer_framebuffer(
        &self,
        target: FramebufferKind,
        attachment: Attachment,
        level: i32,
        layer: i32,
    ) {
        self.context
            ._framebuffer_texture_layer(target, attachment, &self.inner, level, layer);
    }
}

/// Bindings for WebGLTexture
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    type WebGLTexture;
    /// Binding for `WebGLRenderingContext.createTexture()`.
    #[wasm_bindgen(method, js_name = createTexture)]
    fn _create_texture(this: &WebGL2RenderingContext) -> WebGLTexture;

    /// Binding for `WebGLRenderingContext.bindTexture()`
    #[wasm_bindgen(method, js_name = bindTexture)]
    fn _bind_texture(this: &WebGL2RenderingContext, target: TextureKind, texture: &WebGLTexture);

    /// Binding for `WebGLRenderingContext.deleteTexture()`
    #[wasm_bindgen(method, js_name = deleteTexture)]
    fn _delete_texture(this: &WebGL2RenderingContext, texture: WebGLTexture);

    /// Binding for `WebGLRenderingContext.isTexture()`
    #[wasm_bindgen(method, js_name = isTexture)]
    fn _is_texture(this: &WebGL2RenderingContext, texture: &WebGLTexture) -> bool;

    /// Binding for `WebGLRenderingContext.framebufferTexture2D()`
    #[wasm_bindgen(method, js_name = framebufferTexture2D)]
    fn _framebuffer_texture_2d(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        attachment: Attachment,
        textarget: TextureBindPoint,
        texture: &WebGLTexture,
        level: i32,
    );

    /// Binding for `WebGL2RenderingContext.framebufferTextureLayer()`
    #[wasm_bindgen(method, js_name = framebufferTextureLayer)]
    fn _framebuffer_texture_layer(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        attachment: Attachment,
        texture: &WebGLTexture,
        level: i32,
        layer: i32,
    );
}
