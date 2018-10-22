//! WebGLTransformFeedback and methods
use glenum::TransformFeedback;
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes a `WebGLRSTransformFeedback` object.
    pub fn create_transform_feedback(&self) -> WebGLRSTransformFeedback {
        WebGLRSTransformFeedback {
            context: self,
            inner: self._create_transform_feedback(),
        }
    }
}

pub struct WebGLRSTransformFeedback<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLTransformFeedback,
}

impl<'ctx> WebGLRSTransformFeedback<'ctx> {
    /// Deletes this `WebGLRSTransformFeedback` object.
    pub fn delete(self) {
        self.context._delete_transform_feedback(self.inner);
    }

    /// Returns true if this is a valid `WebGLRSTransformFeedback` object.
    pub fn is_valid(&self) -> bool {
        self.context._is_transform_feedback(&self.inner)
    }

    /// Binds this `WebGLRSTransformFeedback` object to the current GL state.
    ///
    /// # Arguments
    /// * `target` -  specifying the target (binding point).
    pub fn bind(&self, target: TransformFeedback) {
        self.context._bind_transform_feedback(target, &self.inner);
    }
}

/// Bindings for WebGLTransformFeedback
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    pub type WebGLTransformFeedback;

    /// Binding for `WebGL2RenderingContext.createTransformFeedback()`
    #[wasm_bindgen(method, js_name = createTransformFeedback)]
    fn _create_transform_feedback(this: &WebGL2RenderingContext) -> WebGLTransformFeedback;

    /// Binding for `WebGL2RenderingContext.deleteTransformFeedback()`
    #[wasm_bindgen(method, js_name = deleteTransformFeedback)]
    fn _delete_transform_feedback(
        this: &WebGL2RenderingContext,
        transform_feedback: WebGLTransformFeedback,
    );

    /// Binding for `WebGL2RenderingContext.isTransformFeedback()`
    #[wasm_bindgen(method, js_name = isTransformFeedback)]
    fn _is_transform_feedback(
        this: &WebGL2RenderingContext,
        transform_feedback: &WebGLTransformFeedback,
    ) -> bool;

    /// Binding for `WebGL2RenderingContext.bindTransformFeedback()`
    #[wasm_bindgen(method, js_name = bindTransformFeedback)]
    fn _bind_transform_feedback(
        this: &WebGL2RenderingContext,
        target: TransformFeedback,
        transform_feedback: &WebGLTransformFeedback,
    );

}
