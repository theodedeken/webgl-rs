use glenum::{BufferKind, DataHint, PixelCopyFormat, PixelReadFormat, PixelType, TextureBindPoint};
use rendering_context::WebGL2RenderingContext;
use wasm_bindgen::JsValue;

pub trait Buffer {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint);
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64);
}

pub trait Image {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue>;
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue>;
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue>;
}

impl Buffer for Vec<u8> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_u8(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_u8(target, offset, self);
    }
}

impl Image for Vec<u8> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_u8(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_u8(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_u8(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<i8> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_i8(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_i8(target, offset, self);
    }
}

impl Image for Vec<i8> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_i8(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_i8(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_i8(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<u16> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_u16(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_u16(target, offset, self);
    }
}

impl Image for Vec<u16> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_u16(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_u16(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_u16(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<i16> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_i16(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_i16(target, offset, self);
    }
}

impl Image for Vec<i16> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_i16(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_i16(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_i16(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<u32> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_u32(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_u32(target, offset, self);
    }
}

impl Image for Vec<u32> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_u32(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_u32(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_u32(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<i32> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_i32(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_i32(target, offset, self);
    }
}

impl Image for Vec<i32> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_i32(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_i32(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_i32(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<f32> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_f32(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_f32(target, offset, self);
    }
}

impl Image for Vec<f32> {
    fn tex_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        internalformat: PixelCopyFormat,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_image_2d_f32(
            target,
            level,
            internalformat,
            width,
            height,
            0,
            format,
            pixel_type,
            self,
        )
    }
    fn tex_sub_image_2d(
        &self,
        context: &WebGL2RenderingContext,
        target: TextureBindPoint,
        level: u32,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        format: PixelCopyFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._tex_sub_image_2d_f32(
            target, level, xoffset, yoffset, width, height, format, pixel_type, self,
        )
    }
    fn read_pixels(
        &mut self,
        context: &WebGL2RenderingContext,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelReadFormat,
        pixel_type: PixelType,
    ) -> Result<(), JsValue> {
        context._read_pixels_f32(x, y, width, height, format, pixel_type, self)
    }
}

impl Buffer for Vec<f64> {
    fn buffer_data(&self, context: &WebGL2RenderingContext, target: BufferKind, usage: DataHint) {
        context._buffer_data_f64(target, self, usage);
    }
    fn buffer_sub_data(&self, context: &WebGL2RenderingContext, target: BufferKind, offset: i64) {
        context._buffer_sub_data_f64(target, offset, self);
    }
}
