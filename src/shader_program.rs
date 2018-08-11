//! WebGLProgram and WebGLShader and methods
use glenum::{ProgramParameter, ShaderKind, ShaderParameter, TransformFeedbackBufferMode};
use rendering_context::{WebGL2RenderingContext, WebGLUniformLocation};
use wasm_bindgen::prelude::*;

// TODO WebGLRenderingContext.getAttachedShaders()

/// WebGLRSProgram

impl WebGL2RenderingContext {
    /// Creates and initializes a `WebGLRSProgram`
    pub fn create_program(&self) -> WebGLRSProgram {
        WebGLRSProgram {
            context: self,
            inner: self._create_program(),
        }
    }
}

/// The `WebGLRSProgram` is part of the WebGL API and is a combination of two compiled WebGLShaders
/// consisting of a vertex shader and a fragment shader (both written in GLSL). These are then linked
/// into a usable program.
pub struct WebGLRSProgram<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLProgram,
}

impl<'ctx> WebGLRSProgram<'ctx> {
    /// Deletes this `WebGLRSProgram` object. This method has no effect if the program has already been deleted.
    pub fn delete(self) {
        self.context._delete_program(self.inner);
    }

    /// Detaches a previously attached `WebGLRSShader` from this `WebGLRSProgram`.
    ///
    /// # Arguments
    /// * `shader` - shader to detach
    pub fn detach_shader(&self, shader: &WebGLRSShader) {
        self.context._detach_shader(&self.inner, &shader.inner);
    }

    /// Attaches a `WebGLRSShader` to this `WebGLRSProgram`.
    ///
    /// # Arguments
    /// * `shader` - shader to attach
    pub fn attach_shader(&self, shader: &WebGLRSShader) {
        self.context._attach_shader(&self.inner, &shader.inner)
    }

    /// Binds a generic vertex index to an attribute variable for this `WebGLRSProgram`
    ///
    /// # Arguments
    /// * `index` - specifying the index of the generic vertex to bind.
    /// * `name` - specifying the name of the variable to bind to the generic vertex index.
    ///         This name cannot start with "webgl_" or "_webgl_", as these are reserved for use by WebGL.
    pub fn bind_attrib_location(&self, index: u32, name: &str) {
        self.context._bind_attrib_location(&self.inner, index, name);
    }

    /// Returns the information log for this `WebGLRSProgram` object. It contains errors that occurred during
    /// failed linking or validation of this object.
    pub fn info_log(&self) -> String {
        self.context._get_program_info_log(&self.inner)
    }

    /// Returns true if this `WebGLRSProgram` is valid, false otherwise.
    pub fn is_valid(&self) -> bool {
        self.context._is_program(&self.inner)
    }

    /// Links this `WebGLRSProgram` to the attached vertex and fragment shaders.
    pub fn link(&self) {
        self.context._link_program(&self.inner);
    }

    /// Sets this `WebGLRSProgram` as part of the current rendering state.
    pub fn enable(&self) {
        self.context._use_program(&self.inner);
    }

    /// Validates this `WebGLRSProgram`. Checks if it is successfully linked and if it can be used in the
    /// current WebGL state.
    pub fn validate(&self) {
        self.context._validate_program(&self.inner);
    }

    /// Returns a `WebGLActiveInfo` object containing size, type, and name of a vertex attribute.
    ///
    /// It is generally used when querying unknown uniforms either for debugging or generic library creation.
    ///
    /// # Arguments
    /// * `index` - specifying the index of the vertex attribute to get.
    pub fn active_attrib(&self, index: u32) -> WebGLActiveInfo {
        self.context._get_active_attrib(&self.inner, index)
    }

    /// Returns a WebGLActiveInfo object containing size, type, and name of a uniform attribute.
    ///
    /// It is generally used when querying unknown uniforms either for debugging or generic library creation.
    ///
    /// # Arguments
    /// * `index` - specifying the index of the vertex attribute to get.
    pub fn active_uniform(&self, index: u32) -> WebGLActiveInfo {
        self.context._get_active_uniform(&self.inner, index)
    }

    /// Returns the location of an attribute variable in this `WebGLRSProgram`.
    ///
    /// # Arguments
    /// * `name` - specifying the name of the attribute variable whose location to get.  
    pub fn attrib_location(&self, name: &str) -> u32 {
        self.context._get_attrib_location(&self.inner, name)
    }

    /// Returns the location of a specific uniform variable which is part this `WebGLRSProgram`.
    ///
    /// The uniform variable is returned as a `WebGLUniformLocation` object, which is an opaque identifier
    /// used to specify where in the GPU's memory that uniform variable is located.
    ///
    /// # Arguments
    /// * `name`- specifying the name of the uniform variable whose location is to be returned. The name can't
    ///         have any whitespace in it, and you can't use this function to get the location of any uniforms
    ///         starting with the reserved string "gl_", since those are internal to the WebGL layer.
    pub fn uniform_location(&self, name: &str) -> WebGLUniformLocation {
        self.context._get_uniform_location(&self.inner, name)
    }

    /// Returns the binding of color numbers to user-defined varying out variables.
    ///
    /// # Arguments
    /// * `name` - specifying the name of the user-defined varying out variable.
    pub fn frag_data_location(&self, name: &str) -> i32 {
        self.context._get_frag_data_location(&self.inner, name)
    }

    /// Returns information about varying variables from WebGLTransformFeedback buffers.
    ///
    /// # Arguments
    /// * `index` -  specifying the index of the varying variable whose information to retrieve.
    pub fn transform_feedback_varying(&self, index: u32) -> WebGLActiveInfo {
        self.context
            ._get_transform_feedback_varying(&self.inner, index)
    }

    /// Retrieves the index of a uniform block in this `WebGLRSProgram`.
    ///
    /// # Arguments
    /// * `uniform_block_name` - specifying the name of the uniform block to whose index to retrieve.
    pub fn uniform_block_index(&self, uniform_block_name: &str) -> u32 {
        self.context
            ._get_uniform_block_index(&self.inner, uniform_block_name)
    }

    /// Retrieves the name of the active uniform block at a given index in this `WebGLRSProgram`.
    ///
    /// # Arguments
    /// * `uniform_block_index` - specifying the index of the uniform block to whose name to retrieve.
    pub fn active_uniform_block_name(&self, uniform_block_index: u32) -> String {
        self.context
            ._get_active_uniform_block_name(&self.inner, uniform_block_index)
    }

    /// Asigns binding points for active uniform blocks.
    ///
    /// # Arguments
    /// * `uniform_block_index` - specifying the index of the active uniform block within the program.
    /// * `uniform_block_binding` - specifying the binding point to which to bind the uniform block.
    pub fn assign_uniform_block_binding(
        &self,
        uniform_block_index: u32,
        uniform_block_binding: u32,
    ) {
        self.context._uniform_block_binding(
            &self.inner,
            uniform_block_index,
            uniform_block_binding,
        );
    }

    /// Indicates whether or not the program is flagged for deletion.
    pub fn delete_status(&self) -> bool {
        self.context
            ._get_program_parameter_bool(&self.inner, ProgramParameter::DeleteStatus)
    }

    /// Indicates whether or not the last link operation was successful.
    pub fn link_status(&self) -> bool {
        self.context
            ._get_program_parameter_bool(&self.inner, ProgramParameter::LinkStatus)
    }

    /// Indicates whether or not the last validation operation was successful.
    pub fn validate_status(&self) -> bool {
        self.context
            ._get_program_parameter_bool(&self.inner, ProgramParameter::ValidateStatus)
    }

    /// Returns the number of attached shaders to a program.
    pub fn attached_shaders(&self) -> i32 {
        self.context
            ._get_program_parameter_i32(&self.inner, ProgramParameter::AttachedShaders)
    }

    /// Returns the number of active attribute variables to a program.
    pub fn active_attributes(&self) -> i32 {
        self.context
            ._get_program_parameter_i32(&self.inner, ProgramParameter::ActiveAttributes)
    }

    /// Returns the number of active uniform variables to a program.
    pub fn active_uniforms(&self) -> i32 {
        self.context
            ._get_program_parameter_i32(&self.inner, ProgramParameter::ActiveUniforms)
    }

    /// Returns the buffer mode when transform feedback is active. May be `SeparateAttribs` or `InterleavedAttribs`.
    pub fn transform_feedback_buffer_mode(&self) -> TransformFeedbackBufferMode {
        self.context
            ._get_program_parameter_enum(&self.inner, ProgramParameter::TransformFeedbackBufferMode)
    }

    /// Returns the number of varying variables to capture in transform feedback mode.
    pub fn transform_feedback_varyings(&self) -> i32 {
        self.context
            ._get_program_parameter_i32(&self.inner, ProgramParameter::TransformFeedbackVaryings)
    }

    // Returns the number of uniform blocks containing active uniforms.
    pub fn active_uniform_blocks(&self) -> i32 {
        self.context
            ._get_program_parameter_i32(&self.inner, ProgramParameter::ActiveUniformBlocks)
    }
}

/// Bindings for WebGLProgram
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    type WebGLProgram;

    /// Binding for `WebGLRenderingContext.createProgram()`
    #[wasm_bindgen(method, js_name = createProgram)]
    fn _create_program(this: &WebGL2RenderingContext) -> WebGLProgram;

    /// Binding for `WebGLRenderingContext.deleteProgram()`.
    #[wasm_bindgen(method, js_name = deleteProgram)]
    fn _delete_program(this: &WebGL2RenderingContext, program: WebGLProgram);

    /// Binding for `WebGLRenderingContext.detachShader()`.
    #[wasm_bindgen(method, js_name = detachShader)]
    fn _detach_shader(this: &WebGL2RenderingContext, program: &WebGLProgram, shader: &WebGLShader);

    /// Binding for `WebGLRenderingContext.attachShader()`.
    #[wasm_bindgen(method, js_name = attachShader)]
    fn _attach_shader(this: &WebGL2RenderingContext, program: &WebGLProgram, shader: &WebGLShader);

    /// Binding for `WebGLRenderingContext.bindAttribLocation()`.
    #[wasm_bindgen(method, js_name = bindAttribLocation)]
    fn _bind_attrib_location(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        index: u32,
        name: &str,
    );

    /// Binding for `WebGLRenderingContext.getProgramInfoLog`.
    #[wasm_bindgen(method, js_name = getProgramInfoLog)]
    fn _get_program_info_log(this: &WebGL2RenderingContext, program: &WebGLProgram) -> String;

    /// Binding for `WebGLRenderingContext.isProgram()`.
    #[wasm_bindgen(method, js_name = isProgram)]
    fn _is_program(this: &WebGL2RenderingContext, program: &WebGLProgram) -> bool;

    /// Binding for `WebGLRenderingContext.linkProgram()`.
    #[wasm_bindgen(method, js_name = linkProgram)]
    fn _link_program(this: &WebGL2RenderingContext, program: &WebGLProgram);

    /// Binding for `WebGLRenderingContext.useProgram()`.
    #[wasm_bindgen(method, js_name = useProgram)]
    fn _use_program(this: &WebGL2RenderingContext, program: &WebGLProgram);

    /// Binding for `WebGLRenderingContext.validateProgram()`.
    #[wasm_bindgen(method, js_name = validateProgram)]
    fn _validate_program(this: &WebGL2RenderingContext, program: &WebGLProgram);

    /// Binding for `WebGLRenderingContext.getActiveAttrib()`.
    #[wasm_bindgen(method, js_name = getActiveAttrib)]
    fn _get_active_attrib(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        index: u32,
    ) -> WebGLActiveInfo;

    /// Binding for `WebGLRenderingContext.getActiveUniform()`.
    #[wasm_bindgen(method, js_name = getActiveUniform)]
    fn _get_active_uniform(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        index: u32,
    ) -> WebGLActiveInfo;

    /// Binding for `WebGLRenderingContext.getAttribLocation()`.
    #[wasm_bindgen(method, js_name = getAttribLocation)]
    fn _get_attrib_location(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        name: &str,
    ) -> u32;

    /// Binding for `WebGLRenderingContext method getUniformLocation()`.
    #[wasm_bindgen(method, js_name = getUniformLocation)]
    fn _get_uniform_location(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        name: &str,
    ) -> WebGLUniformLocation;

    /// Binding for `WebGL2RenderingContext.getFragDataLocation()`.
    #[wasm_bindgen(method, js_name = getFragDataLocation)]
    fn _get_frag_data_location(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        name: &str,
    ) -> i32;

    /// Binding for `WebGL2RenderingContext.getTransformFeedbackVarying()`
    #[wasm_bindgen(method, js_name = getTransfromFeedbackVarying)]
    fn _get_transform_feedback_varying(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        index: u32,
    ) -> WebGLActiveInfo;

    /// Binding for `WebGL2RenderingContext.getUniformBlockIndex()`
    #[wasm_bindgen(method, js_name = getUniformBlockIndex)]
    fn _get_uniform_block_index(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        uniform_block_name: &str,
    ) -> u32;

    /// Binding for `WebGL2RenderingContext.getActiveUniformBlockName()`.
    #[wasm_bindgen(method, js_name = getActiveUniformBlockName)]
    fn _get_active_uniform_block_name(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        uniform_block_index: u32,
    ) -> String;

    /// Binding for `WebGL2RenderingContext.uniformBlockBinding()`.
    #[wasm_bindgen(method, js_name = uniformBlockBinding)]
    fn _uniform_block_binding(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        uniform_block_index: u32,
        uniform_block_binding: u32,
    );

    /// Binding for `WebGL2RenderingContext.getProgramParameter()` when return type is i32
    #[wasm_bindgen(method, js_name = getProgramParameter)]
    fn _get_program_parameter_i32(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        pname: ProgramParameter,
    ) -> i32;
    /// Binding for `WebGL2RenderingContext.getProgramParameter()` when return type is bool
    #[wasm_bindgen(method, js_name = getProgramParameter)]
    fn _get_program_parameter_bool(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        pname: ProgramParameter,
    ) -> bool;
    /// Binding for `WebGL2RenderingContext.getProgramParameter()` when return type is enum
    #[wasm_bindgen(method, js_name = getProgramParameter)]
    fn _get_program_parameter_enum(
        this: &WebGL2RenderingContext,
        program: &WebGLProgram,
        pname: ProgramParameter,
    ) -> TransformFeedbackBufferMode;
}

// WebGLRSShader

impl WebGL2RenderingContext {
    /// Creates a WebGLRSShader that can then be configured further using `shader_source()` and `compile_shader()`.
    ///
    /// # Arguments
    /// * `kind` - Kind of the shader (Vertex or Fragment)
    pub fn create_shader(&self, kind: ShaderKind) -> WebGLRSShader {
        WebGLRSShader {
            context: self,
            inner: self._create_shader(kind),
        }
    }
}

/// The `WebGLRSShader` is part of the WebGL API and can either be a vertex or a fragment shader.
/// A `WebGLRSProgram` requires both types of shaders.
pub struct WebGLRSShader<'ctx> {
    context: &'ctx WebGL2RenderingContext,
    inner: WebGLShader,
}

impl<'ctx> WebGLRSShader<'ctx> {
    /// Compiles a GLSL shader into binary data so that it can be used by a `WebGLRSProgram`.
    pub fn compile(&self) {
        self.context._compile_shader(&self.inner);
    }

    /// Deletes this `WebGLRSShader` object.
    pub fn delete(self) {
        self.context._delete_shader(self.inner);
    }

    /// Returns the information log for the specified `WebGLRSShader` object. It contains warnings, debugging
    /// and compile information.
    pub fn info_log(&self) -> String {
        self.context._get_shader_info_log(&self.inner)
    }

    /// Returns the source code of this `WebGLRSShader` as a `String`.
    pub fn shader_source(&self) -> String {
        self.context._get_shader_info_log(&self.inner)
    }

    /// Returns true if this `WebGLRSShader` is valid, false otherwise.
    pub fn is_valid(&self) -> bool {
        self.context._is_shader(&self.inner)
    }

    /// Sets the source code of the `WebGLRSShader` object.
    pub fn set_shader_source(&self, source: &str) {
        self.context._shader_source(&self.inner, source);
    }

    /// Returns a bool indicating whether or not the `WebGLRSShader` is flagged for deletion.
    pub fn delete_status(&self) -> bool {
        self.context
            ._get_shader_parameter_bool(&self.inner, ShaderParameter::DeleteStatus)
    }

    /// Returns a bool indicating whether or not the last compilation was successful.
    pub fn compile_status(&self) -> bool {
        self.context
            ._get_shader_parameter_bool(&self.inner, ShaderParameter::CompileStatus)
    }

    /// Returns a `ShaderKind` value indicating whether the shader is a vertex shader or fragment shader.
    pub fn kind(&self) -> ShaderKind {
        self.context
            ._get_shader_parameter_enum(&self.inner, ShaderParameter::ShaderType)
    }
}

/// Bindings for WebGLShader
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    type WebGLShader;

    /// Binding for `WebGLRenderingContext.createShader()`
    #[wasm_bindgen(method, js_name = createShader)]
    fn _create_shader(this: &WebGL2RenderingContext, kind: ShaderKind) -> WebGLShader;

    /// Binding for `WebGLRenderingContext.compileShader()`
    #[wasm_bindgen(method, js_name = compileShader)]
    fn _compile_shader(this: &WebGL2RenderingContext, shader: &WebGLShader);

    /// Binding for `WebGLRenderingContext.deleteShader()`
    #[wasm_bindgen(method, js_name = deleteShader)]
    fn _delete_shader(this: &WebGL2RenderingContext, shader: WebGLShader);

    /// Binding for `WebGLRenderingContext.getShaderInfoLog`
    #[wasm_bindgen(method, js_name = getShaderInfoLog)]
    fn _get_shader_info_log(this: &WebGL2RenderingContext, shader: &WebGLShader) -> String;

    /// Binding for `WebGLRenderingContext.getShaderSource()`.
    #[wasm_bindgen(method, js_name = getShaderSource)]
    fn _get_shader_source(this: &WebGL2RenderingContext, shader: &WebGLShader) -> String;

    /// Binding for `WebGLRenderingContext.isShader()`.
    #[wasm_bindgen(method, js_name = isShader)]
    fn _is_shader(this: &WebGL2RenderingContext, shader: &WebGLShader) -> bool;

    /// Binding for `WebGLRenderingContext.shaderSource()`.
    #[wasm_bindgen(method, js_name = shaderSource)]
    fn _shader_source(this: &WebGL2RenderingContext, shader: &WebGLShader, source: &str);

    /// Binding for `WebGLRenderingContext.getShaderParameter()` if return type is bool
    #[wasm_bindgen(method, js_name = getShaderParameter)]
    fn _get_shader_parameter_bool(
        this: &WebGL2RenderingContext,
        shader: &WebGLShader,
        pname: ShaderParameter,
    ) -> bool;
    /// Binding for `WebGLRenderingContext.getShaderParameter()` if return type is enum
    #[wasm_bindgen(method, js_name = getShaderParameter)]
    fn _get_shader_parameter_enum(
        this: &WebGL2RenderingContext,
        shader: &WebGLShader,
        pname: ShaderParameter,
    ) -> ShaderKind;
}

/// WebGLActiveInfo
#[derive(Clone, Copy)]
#[wasm_bindgen]
extern "C" {
    pub type WebGLActiveInfo;

    /// The read-only name of the requested variable.
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &WebGLActiveInfo) -> String;

    /// The read-only size of the requested variable.
    #[wasm_bindgen(method, getter)]
    pub fn size(this: &WebGLActiveInfo) -> u32;

    /// The read-only type of the requested variable.
    #[wasm_bindgen(method, getter = type)]
    pub fn data_type(this: &WebGLActiveInfo) -> u32;
}
