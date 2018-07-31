#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

//TODO: arraybufferview as enum and impl method to map to different functions
//TODO: all pub types in function to references
//TODO: possible solution of different getParameter methods is to add accessors on different types
//TODO: safety with methods that can throw
//TODO: JsString?
//TODO: js-sys objects?
pub mod glenum;
pub mod rendering_context;
pub mod vertex_array_object;

pub use glenum::*;
pub use rendering_context::*;
pub use vertex_array_object::*;
