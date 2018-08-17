# Implementation and structure ideas
## Webgl1 vs Webgl2

from an answer on stackoverflow, the promoted extensions:

// 14 extensions were promoted in WebGL2
[
  'ANGLE_instanced_arrays',
  'EXT_blend_minmax',
  'EXT_frag_depth',
  'EXT_shader_texture_lod',
  'OES_element_index_uint',
  'OES_standard_derivatives',
  'OES_texture_float',
  'OES_texture_half_float',
  'WEBGL_depth_texture',
  'WEBGL_draw_buffers',
  /* with caveats */
  'EXT_sRGB',
  'OES_texture_half_float_linear',
  'EXT_disjoint_timer_query',
]

Please note that last three extensions were promoted with caveats. Extension EXT_sRGB lost a constant SRGB_ALPHA. Extension OES_texture_half_float_linear was promoted while analogous OES_texture_float_linear was not. Extension EXT_disjoint_timer_query promoted partially. Some properties of that extension appeared in WebGL2 context, while other properties were moved to EXT_disjoint_timer_query_webgl2 extension. Also, currently ( 2017.05.16 ) Firefox WebGL2 context still has EXT_disjoint_timer_query extensions and no EXT_disjoint_timer_query_webgl2 extension.

So ideally the api of WebGl1 and Webgl2 should be as equal as possible using these extensions to fill in the gaps in webgl1. This way we can use traits for shared functionality and provide custom implementation where necessary. 
### OES_vertex_array_object
provides the vertexarray interface

## Parameter methods
Prerequisites:
* Splitting of the different objects by making custom structs with reference to context
* methods working on these objects are implemented straight on these custom structs
* Separate into modules but pub use in root
Idea:
* for context state info split the get parameter methods all up in their own separate methods
* for parameters of related opbject split into methods of these objects 

## Image loading
| type of srcData | type                         |
| --------------- | ---------------------------- |
| Int8Array       | BYTE                         |
| Uint8Array      | UNSIGNED_BYTE                |
| Int16Array      | SHORT                        |
| Uint16Array     | UNSIGNED_SHORT               |
| Uint16Array     | UNSIGNED_SHORT_5_6_5         |
| Uint16Array     | UNSIGNED_SHORT_5_5_5_1       |
| Uint16Array     | UNSIGNED_SHORT_4_4_4_4       |
| Int32Array      | INT                          |
| Uint32Array     | UNSIGNED_INT                 |
| Uint32Array     | UNSIGNED_INT_5_9_9_9_REV     |
| Uint32Array     | UNSIGNED_INT_2_10_10_10_REV  |
| Uint32Array     | UNSIGNED_INT_10F_11F_11F_REV |
| Uint32Array     | UNSIGNED_INT_24_8            |
| Uint16Array     | HALF_FLOAT                   |
| Float32Array    | FLOAT                        |