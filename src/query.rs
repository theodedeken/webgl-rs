//! WebGLQuery and methods
use glenum::{Query, QueryParameter, QueryTarget};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates and initializes a `WebGLRSQuery` object, that provide ways to asynchronously query for information.
    pub fn create_query(&self) -> WebGLRSQuery {
        WebGLRSQuery {
            context: self,
            inner: self._create_query(),
        }
    }

    /// Returns the currently active WebGLQuery for the target, or null.
    ///
    /// # Arguments
    /// * `target` - specifying the target of the query
    /// TODO nullable -> Option
    pub fn query(&self, target: QueryTarget) -> WebGLRSQuery {
        WebGLRSQuery {
            context: self,
            inner: self._get_query(target, Query::Current),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WebGLRSQuery<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLQuery,
}

impl<'ctx> WebGLRSQuery<'ctx> {
    /// Deletes this `WebGLRSQuery` object.
    pub fn delete(self) {
        self.context._delete_query(self.inner);
    }

    /// Returns true if this is a valid `WebGLRSQuery` object.
    pub fn is_valid(&self) -> bool {
        self.context._is_query(&self.inner)
    }

    /// Starts an asynchronous query. The target parameter indicates which kind of query to begin.
    ///
    /// # Arguments
    /// * `target` - specifying the target of the query
    pub fn begin(&self, target: QueryTarget) {
        self.context._begin_query(target, &self.inner);
    }

    /// Indicates whether the `WebGLRSQuery` has a result available.
    pub fn result_available(&self) -> bool {
        self.context
            ._get_query_parameter_bool(&self.inner, QueryParameter::ResultAvailable)
    }

    /// Returns the result of the `WebGLRSQuery`
    pub fn result(&self) -> u32 {
        self.context
            ._get_query_parameter_u32(&self.inner, QueryParameter::Result)
    }
}

/// Binding for WebGLQuery
#[wasm_bindgen]
#[derive(Clone, Copy)]
extern "C" {
    type WebGLQuery;

    /// Binding for `WebGL2RenderingContext.createQuery()`
    #[wasm_bindgen(method, js_name = createQuery)]
    fn _create_query(this: &WebGL2RenderingContext) -> WebGLQuery;

    /// Binding for `WebGL2RenderingContext.deleteQuery()`
    #[wasm_bindgen(method, js_name = deleteQuery)]
    fn _delete_query(this: &WebGL2RenderingContext, query: WebGLQuery);

    /// Binding for `WebGL2RenderingContext.isQuery()`
    #[wasm_bindgen(method, js_name = isQuery)]
    fn _is_query(this: &WebGL2RenderingContext, query: &WebGLQuery) -> bool;

    /// Binding for `WebGL2RenderingContext.beginQuery()`
    #[wasm_bindgen(method, js_name = beginQuery)]
    fn _begin_query(this: &WebGL2RenderingContext, target: QueryTarget, query: &WebGLQuery);

    /// Binding for `WebGL2RenderingContext.getQuery()`
    #[wasm_bindgen(method, js_name = getQuery)]
    fn _get_query(this: &WebGL2RenderingContext, target: QueryTarget, pname: Query) -> WebGLQuery;

    /// Binding for `WebGL2RenderingContext.getQueryParameter()` if return type is bool
    #[wasm_bindgen(method, js_name = getQueryParameter)]
    fn _get_query_parameter_bool(
        this: &WebGL2RenderingContext,
        query: &WebGLQuery,
        pname: QueryParameter,
    ) -> bool;
    /// Binding for `WebGL2RenderingContext.getQueryParameter()` if return type is u32
    #[wasm_bindgen(method, js_name = getQueryParameter)]
    fn _get_query_parameter_u32(
        this: &WebGL2RenderingContext,
        query: &WebGLQuery,
        pname: QueryParameter,
    ) -> u32;
}
