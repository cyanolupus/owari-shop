use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use worker::RouteContext;

pub struct FaviconGenerator {
    fontpath: String,
    target_text: Vec<String>,
    image_properties: ImageProperties,
}

pub struct ImageProperties {
    height: u32,
    width: u32,
    background_color: Rgba<u8>,
    font_color: Rgba<u8>,
}

impl ImageProperties {
    pub fn new(height: u32, width: u32, background_color: Rgba<u8>, font_color: Rgba<u8>) -> Self {
        Self {
            height,
            width,
            background_color,
            font_color,
        }
    }
}

impl FaviconGenerator {
    pub fn new(
        fontpath: String,
        target_text: Vec<String>,
        image_properties: ImageProperties,
    ) -> Self {
        Self {
            fontpath,
            target_text,
            image_properties,
        }
    }

    pub async fn write_image<D>(
        &self,
        ctx: &RouteContext<D>,
        format: image::ImageFormat,
    ) -> Option<Vec<u8>> {
        let font_bytes = match self.get_font_bytes(ctx).await {
            Some(bytes) => bytes,
            None => return None,
        };

        let font = match FontRef::try_from_slice(&font_bytes) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut buf = Vec::new();
        self.create_image(&font)
            .write_to(&mut std::io::Cursor::new(&mut buf), format)
            .ok()?;
        Some(buf)
    }

    fn create_image(&self, font: &FontRef) -> DynamicImage {
        let mut img =
            image::ImageBuffer::new(self.image_properties.width, self.image_properties.height);

        for x in 0..self.image_properties.width {
            for y in 0..self.image_properties.height {
                img.put_pixel(x, y, self.image_properties.background_color);
            }
        }

        let height_f32 = self.image_properties.height as f32;
        let width_f32 = self.image_properties.width as f32;
        let text_height = height_f32 / (self.target_text.len() as f32);

        let mut y_pos = 0.0;
        for text in &self.target_text {
            let scale = self.get_optimal_scale(text_height, width_f32, font, text);

            draw_text_mut(
                &mut img,
                self.image_properties.font_color,
                0,
                y_pos as i32,
                scale,
                font,
                text,
            );
            y_pos += text_height;
        }

        image::DynamicImage::ImageRgba8(img)
    }

    async fn get_font_bytes<D>(&self, ctx: &RouteContext<D>) -> Option<Vec<u8>> {
        Self::r2_get(ctx, &self.fontpath).await
    }

    async fn r2_get<D>(ctx: &RouteContext<D>, key: &str) -> Option<Vec<u8>> {
        let bucket = ctx.bucket("BUCKET").unwrap();
        let item = bucket.get(key).execute().await.ok()??;
        item.body()?.bytes().await.ok()
    }

    fn get_optimal_scale(
        &self,
        target_height: f32,
        target_width: f32,
        font: &FontRef,
        text: &str,
    ) -> PxScale {
        let scale = PxScale::from(target_height);
        let scaled_font = font.as_scaled(scale);

        let mut width_sum = 0.0;
        for c in text.chars() {
            width_sum += scaled_font.h_advance(scaled_font.glyph_id(c));
        }

        if width_sum > target_width {
            let factor = target_width / width_sum;
            PxScale::from(target_height * factor)
        } else {
            scale
        }
    }
}
