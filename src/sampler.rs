//! WebGLSampler and methods
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes a `WebGLRSSampler` object.
    pub fn create_sampler(&self) -> WebGLRSSampler {
        WebGLRSSampler {
            context: self,
            inner: self._create_sampler(),
        }
    }
}

pub struct WebGLRSSampler<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLSampler,
}

impl<'ctx> WebGLRSSampler<'ctx> {
    /// Deletes this `WebGLRSSampler` object.
    pub fn delete(self) {
        self.context._delete_sampler(self.inner);
    }

    /// Binds this `WebGLRSSampler` object to the texture unit at the passed index.
    ///
    /// # Arguments
    /// * `unit` - specifying the index of the texture unit to which to bind the sampler to.
    pub fn bind(&self, unit: u32) {
        self.context._bind_sampler(unit, &self.inner);
    }

    /// Returns true if this is a valid `WebGLRSSampler` object.
    pub fn is_valid(&self) -> bool {
        self.context._is_sampler(&self.inner)
    }
}

/// Bindings for WebGLSampler
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    type WebGLSampler;

    /// Binding for `WebGL2RenderingContext.createSampler()`
    #[wasm_bindgen(method, js_name = createSampler)]
    fn _create_sampler(this: &WebGL2RenderingContext) -> WebGLSampler;

    /// Binding for `WebGL2RenderingContext.deleteSampler()`.
    #[wasm_bindgen(method, js_name = deleteSampler)]
    fn _delete_sampler(this: &WebGL2RenderingContext, sampler: WebGLSampler);

    /// Binding for `WebGL2RenderingContext.bindSampler()`
    #[wasm_bindgen(method, js_name = bindSampler)]
    fn _bind_sampler(this: &WebGL2RenderingContext, unit: u32, sampler: &WebGLSampler);

    /// Binding for `WebGL2RenderingContext.isSampler()`
    #[wasm_bindgen(method, js_name = isSampler)]
    fn _is_sampler(this: &WebGL2RenderingContext, sampler: &WebGLSampler) -> bool;

//TODO samplerParameter[if] and getSamplerParameter
}
