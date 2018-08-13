//! WebGLSync and methods
use glenum::{GPUState, SyncParameter, SyncStatus, WaitStatus};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::prelude::*;

impl WebGL2RenderingContext {
    /// Creates a new `WebGLRSSync` object and inserts it into the GL command stream.
    ///
    /// # Arguments
    /// * `conditions` - specifying the condition that must be met to set the sync object's state to.
    /// * `flags` - specifying a bitwise combination of flags controlling the behavior of the sync object.
    ///         Must be 0 (exists for extensions only).
    pub fn fence_sync(&self, conditions: GPUState, flags: u32) -> WebGLRSSync {
        WebGLRSSync {
            context: self,
            inner: self._fence_sync(conditions, flags),
        }
    }
}

pub struct WebGLRSSync<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLSync,
}

impl<'ctx> WebGLRSSync<'ctx> {
    /// Deletes this `WebGLRSSync` object
    pub fn delete(self) {
        self.context._delete_sync(self.inner);
    }

    /// Returns true if this is a valid `WebGLRSSync` object.  
    pub fn is_valid(&self) -> bool {
        self.context._is_sync(&self.inner)
    }

    /// Blocks and waits for this `WebGLRSSync` object to become signaled or a given timeout to be passed.
    ///
    /// # Arguments
    /// * `flags` - specifying a bitwise combination of flags controlling the flushing behavior. May be gl.SYNC_FLUSH_COMMANDS_BIT.
    /// * `timeout` - specifying a timeout (in nanoseconds) for which to wait for the sync object to become signaled. Must not be larger than gl.MAX_CLIENT_WAIT_TIMEOUT_WEBGL.
    pub fn client_wait(&self, flags: u32, timeout: i64) -> WaitStatus {
        self.context._client_wait_sync(&self.inner, flags, timeout)
    }

    /// Returns immediately, but waits on the GL server until the `WebGLRSSync` object is signaled.
    ///
    /// The method is a no-op in the absence of the possibility of synchronizing between multiple GL contexts.
    /// # Arguments
    /// * `flags` - specifying a bitwise combination of flags controlling the flushing behavior. May be gl.SYNC_FLUSH_COMMANDS_BIT.
    /// * `timeout` - specifying a timeout (in nanoseconds) for which to wait for the sync object to become signaled. Must not be larger than gl.MAX_CLIENT_WAIT_TIMEOUT_WEBGL.
    // FIXME: timeout must be gl.TIMEOUT_IGNORED.
    pub fn wait(&self, flags: u32, timeout: i64) {
        self.context._wait_sync(&self.inner, flags, timeout);
    }

    /// Returns the status of this `WebGLRSSync` object.
    pub fn status(&self) -> SyncStatus {
        self.context
            ._get_sync_parameter_status(&self.inner, SyncParameter::Status)
    }
}

/// Bindings for WebGLSync
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLSync;

    /// Binding for `WebGL2RenderingContext.fenceSync()`
    #[wasm_bindgen(method, js_name = fenceSync)]
    fn _fence_sync(this: &WebGL2RenderingContext, conditions: GPUState, flags: u32) -> WebGLSync;

    /// Binding for `WebGL2RenderingContext.isSync()` method of the WebGL 2 API
    #[wasm_bindgen(method, js_name = isSync)]
    fn _is_sync(this: &WebGL2RenderingContext, sync: &WebGLSync) -> bool;

    /// Binding for `WebGL2RenderingContext.deleteSync()`
    #[wasm_bindgen(method, js_name = deleteSync)]
    fn _delete_sync(this: &WebGL2RenderingContext, sync: WebGLSync);

    /// Binding for `WebGL2RenderingContext.clientWaitSync()`
    #[wasm_bindgen(method, js_name = clientWaitSync)]
    fn _client_wait_sync(
        this: &WebGL2RenderingContext,
        sync: &WebGLSync,
        flags: u32,
        timeout: i64,
    ) -> WaitStatus;

    /// Binding for `WebGL2RenderingContext.waitSync()`
    #[wasm_bindgen(method, js_name = waitSync)]
    fn _wait_sync(this: &WebGL2RenderingContext, sync: &WebGLSync, flags: u32, timeout: i64);

    /// Binding for `WebGL2RenderingContext.getSyncParameter()` when asking for status
    #[wasm_bindgen(method, js_name = getSyncParameter)]
    fn _get_sync_parameter_status(
        this: &WebGL2RenderingContext,
        sync: &WebGLSync,
        pname: SyncParameter,
    ) -> SyncStatus;
}
