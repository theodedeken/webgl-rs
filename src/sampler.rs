//! WebGLSampler and methods
use glenum::{
    CompareMode, DepthTest, TextureMagFilter, TextureMinFilter, TextureParameter, TextureWrap,
};
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

    /// Returns the texture comparison function for this `WebGLRSSampler` object.
    pub fn texture_compare_func(&self) -> DepthTest {
        self.context
            ._get_sampler_parameter_enum1(&self.inner, TextureParameter::CompareFunc)
    }
    /// Sets the texture comparison function for this `WebGLRSSampler` object.
    pub fn set_texture_compare_func(&self, value: DepthTest) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::CompareFunc, value as i32);
    }

    /// Returns the texture comparison mode for this `WebGLRSSampler` object.
    pub fn texture_compare_mode(&self) -> CompareMode {
        self.context
            ._get_sampler_parameter_enum2(&self.inner, TextureParameter::CompareMode)
    }
    /// Sets the texture comparison mode for this `WebGLRSSampler` object.
    pub fn set_texture_compare_mode(&self, value: CompareMode) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::CompareMode, value as i32);
    }

    /// Returns the texture magnification filter for this `WebGLRSSampler` object.
    pub fn texture_mag_filter(&self) -> TextureMagFilter {
        self.context
            ._get_sampler_parameter_enum3(&self.inner, TextureParameter::MagFilter)
    }
    /// Sets the texture magnification filter for this `WebGLRSSampler` object.
    pub fn set_texture_mag_filter(&self, value: TextureMagFilter) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::MagFilter, value as i32);
    }

    /// Returns the maximum level-of-detail value for this `WebGLRSSampler` object.
    pub fn texture_max_lod(&self) -> f32 {
        self.context
            ._get_sampler_parameter_f32(&self.inner, TextureParameter::MaxLod)
    }
    /// Sets the maximum level-of-detail value for this `WebGLRSSampler` object.
    pub fn set_texture_max_lod(&self, value: f32) {
        self.context
            ._sampler_parameter_f(&self.inner, TextureParameter::MaxLod, value);
    }

    /// Returns the texture minification filter for this `WebGLRSSampler` object
    pub fn texture_min_filter(&self) -> TextureMinFilter {
        self.context
            ._get_sampler_parameter_enum4(&self.inner, TextureParameter::MinFilter)
    }
    /// Sets the texture minification filter for this `WebGLRSSampler` object
    pub fn set_texture_min_filter(&self, value: TextureMinFilter) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::MinFilter, value as i32);
    }

    /// Returns the minimum level-of-detail value for this `WebGLRSSampler` object.
    pub fn texture_min_lod(&self) -> f32 {
        self.context
            ._get_sampler_parameter_f32(&self.inner, TextureParameter::MinLod)
    }
    /// Sets the minimum level-of-detail value for this `WebGLRSSampler` object.
    pub fn set_texture_min_lod(&self, value: f32) {
        self.context
            ._sampler_parameter_f(&self.inner, TextureParameter::MinLod, value);
    }

    /// Returns the texture wrapping function for the texture coordinate r for this `WebGLRSSampler` object.
    pub fn texture_wrap_r(&self) -> TextureWrap {
        self.context
            ._get_sampler_parameter_enum5(&self.inner, TextureParameter::WrapR)
    }
    /// Sets the texture wrapping function for the texture coordinate r for this `WebGLRSSampler` object.
    pub fn set_texture_wrap_r(&self, value: TextureWrap) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::WrapR, value as i32);
    }

    /// Return the texture wrapping function for the texture coordinate s for this `WebGLRSSampler` object.
    pub fn texture_wrap_s(&self) -> TextureWrap {
        self.context
            ._get_sampler_parameter_enum5(&self.inner, TextureParameter::WrapS)
    }
    /// Sets the texture wrapping function for the texture coordinate s for this `WebGLRSSampler` object.
    pub fn set_texture_wrap_s(&self, value: TextureWrap) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::WrapS, value as i32);
    }

    /// Returns the texture wrapping function for the texture coordinate t for this `WebGLRSSampler` object.
    pub fn texture_wrap_t(&self) -> TextureWrap {
        self.context
            ._get_sampler_parameter_enum5(&self.inner, TextureParameter::WrapT)
    }
    /// Sets the texture wrapping function for the texture coordinate t for this `WebGLRSSampler` object.
    pub fn set_texture_wrap_t(&self, value: TextureWrap) {
        self.context
            ._sampler_parameter_i(&self.inner, TextureParameter::WrapT, value as i32);
    }
}

/// Bindings for WebGLSampler
#[wasm_bindgen]
#[derive(Clone, Copy)]
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

    /// Binding for `WebGL2RenderingContext.getSamplerParameter()` when return type is `f32`
    #[wasm_bindgen(method, js_name = getSamplerParameter)]
    fn _get_sampler_parameter_f32(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
    ) -> f32;
    /// Binding for `WebGL2RenderingContext.getSamplerParameter()` when return type is `DepthTest`
    #[wasm_bindgen(method, js_name = getSamplerParameter)]
    fn _get_sampler_parameter_enum1(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
    ) -> DepthTest;
    /// Binding for `WebGL2RenderingContext.getSamplerParameter()` when return type is `CompareMode`
    #[wasm_bindgen(method, js_name = getSamplerParameter)]
    fn _get_sampler_parameter_enum2(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
    ) -> CompareMode;
    /// Binding for `WebGL2RenderingContext.getSamplerParameter()` when return type is `TextureMagFilter`
    #[wasm_bindgen(method, js_name = getSamplerParameter)]
    fn _get_sampler_parameter_enum3(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
    ) -> TextureMagFilter;
    /// Binding for `WebGL2RenderingContext.getSamplerParameter()` when return type is `TextureMinFilter`
    #[wasm_bindgen(method, js_name = getSamplerParameter)]
    fn _get_sampler_parameter_enum4(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
    ) -> TextureMinFilter;
    /// Binding for `WebGL2RenderingContext.getSamplerParameter()` when return type is `TextureWrap`
    #[wasm_bindgen(method, js_name = getSamplerParameter)]
    fn _get_sampler_parameter_enum5(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
    ) -> TextureWrap;

    /// Binding for `WebGL2RenderingContext.samplerParameteri()`
    #[wasm_bindgen(method, js_name = samplerParameteri)]
    fn _sampler_parameter_i(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
        param: i32,
    );

    /// Binding for `WebGL2RenderingContext.samplerParameterf()`
    #[wasm_bindgen(method, js_name = samplerParameterf)]
    fn _sampler_parameter_f(
        this: &WebGL2RenderingContext,
        sampler: &WebGLSampler,
        pname: TextureParameter,
        param: f32,
    );
}
