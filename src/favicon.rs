use image::Rgba;
use imageproc::drawing::{draw_text_mut, Canvas};
use rusttype::{Font, Scale};
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
        format: image::ImageOutputFormat,
    ) -> Option<Vec<u8>> {
        let font = match self.get_font(ctx).await {
            Some(font) => font,
            None => return None,
        };
        let mut buf = Vec::new();
        self.create_image(font).write_to(&mut buf, format).ok()?;
        Some(buf)
    }

    fn create_image(&self, font: Font<'_>) -> image::DynamicImage {
        let mut img = image::DynamicImage::new_rgb8(
            self.image_properties.width,
            self.image_properties.height,
        );

        let x = 0;
        let mut y = 0;
        let height_f32 = self.image_properties.height as f32;
        let width_f32 = self.image_properties.width as f32;

        for x in 0..self.image_properties.width {
            for y in 0..self.image_properties.height {
                img.draw_pixel(x, y, self.image_properties.background_color)
            }
        }

        let text_height = height_f32 / (self.target_text.len() as f32);
        for text in &self.target_text {
            let scale = Self::get_scale_by_font(text_height, width_f32, &font, text);
            draw_text_mut(
                &mut img,
                self.image_properties.font_color,
                x,
                y,
                scale,
                &font,
                text,
            );
            y += text_height as u32;
        }

        img
    }

    async fn get_font<D>(&self, ctx: &RouteContext<D>) -> Option<Font<'static>> {
        match Self::r2_get(ctx, &self.fontpath).await {
            Some(font_bytes) => Font::try_from_vec(font_bytes),
            None => None,
        }
    }

    async fn r2_get<D>(ctx: &RouteContext<D>, key: &str) -> Option<Vec<u8>> {
        let bucket = ctx.bucket("BUCKET").unwrap();
        let item = bucket.get(key).execute().await.ok()??;
        item.body()?.bytes().await.ok()
    }

    fn get_scale_by_font(height: f32, width: f32, font: &Font, text: &str) -> Scale {
        let mut glyph_width_sum = 0.0;
        for c in text.chars() {
            let glyph = font.glyph(c).scaled(Scale::uniform(height));
            glyph_width_sum += glyph.h_metrics().advance_width;
        }
        if glyph_width_sum == 0.0 {
            glyph_width_sum = 1.0;
        }
        Scale {
            x: height * width / glyph_width_sum,
            y: height,
        }
    }
}
