use image::Rgba;
use imageproc::drawing::{draw_text_mut, Canvas};
use rusttype::{Font, Scale};
use worker::RouteContext;

pub struct FaviconGenerator {
    fontpath: String,
    top_top_half_text: String,
    top_half_text: String,
    bottom_half_text: String,
    height: u32,
    width: u32,
    background_color: Rgba<u8>,
    font_color: Rgba<u8>,
}

impl FaviconGenerator {
    pub fn new(fontpath: String, top_top_half_text: String, top_half_text: String, bottom_half_text: String, height: u32, width: u32, background_color: Rgba<u8>, font_color: Rgba<u8>) -> Self {
        Self {
            fontpath,
            top_top_half_text,
            top_half_text,
            bottom_half_text,
            height,
            width,
            background_color,
            font_color,
        }
    }

    pub async fn write_ico<D>(&self, ctx: &RouteContext<D>) -> Option<Vec<u8>> {
        self.write_image(ctx, image::ImageOutputFormat::Ico).await
    }

    pub async fn write_png<D>(&self, ctx: &RouteContext<D>) -> Option<Vec<u8>> {
        self.write_image(ctx, image::ImageOutputFormat::Png).await
    }

    pub async fn write_image<D>(&self, ctx: &RouteContext<D>, format: image::ImageOutputFormat) -> Option<Vec<u8>> {
        let font = match self.get_font(ctx).await {
            Some(font) => font,
            None => return None
        };
        let mut buf = Vec::new();
        self.owariya_image(font).write_to(&mut buf, format).ok()?;
        Some(buf)
    }

    fn owariya_image(&self, font: Font<'_>) -> image::DynamicImage {
        let mut img = image::DynamicImage::new_rgb8(self.width, self.height);

        let x = 0;
        let mut y = 0;
        let height_f32 = self.height as f32;
        let width_f32 = self.width as f32;

        for x in 0..self.width {
            for y in 0..self.height {
                img.draw_pixel(x, y, self.background_color)
            }
        }

        if self.top_top_half_text.is_empty() {
            let scale_top_half = Self::get_scale_by_font(height_f32 / 2.0, width_f32, &font, &self.top_half_text);
            let scale_bottom_half = Self::get_scale_by_font(height_f32 / 2.0, width_f32, &font, &self.bottom_half_text);
            draw_text_mut(&mut img, self.font_color, x, y, scale_top_half, &font, &self.top_half_text);
            y += self.height / 2;
            draw_text_mut(&mut img, self.font_color, x, y, scale_bottom_half, &font, &self.bottom_half_text);
        } else {
            let bottom_bottom_half_text = format!("{}{}", self.top_half_text, self.bottom_half_text);
            let scale_top_top_half = Self::get_scale_by_font(height_f32 / 2.0, width_f32, &font, &self.top_top_half_text);
            let scale_bottom_bottom_half = Self::get_scale_by_font(height_f32 / 2.0, width_f32, &font, &bottom_bottom_half_text);
            draw_text_mut(&mut img, self.font_color, x, y, scale_top_top_half, &font, &self.top_top_half_text);
            y += self.height / 2;
            draw_text_mut(&mut img, self.font_color, x, y, scale_bottom_bottom_half, &font, &bottom_bottom_half_text);
        }

        img
    }

    async fn get_font<D>(&self, ctx: &RouteContext<D>) -> Option<Font<'static>> {
        match Self::r2_get(ctx, &self.fontpath).await {
            Some(font_bytes) => Font::try_from_vec(font_bytes),
            None => return None
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
