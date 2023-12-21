use image::Rgba;
use imageproc::drawing::{draw_text_mut, Canvas};
use rusttype::{Font, Scale};
use worker::*;

mod r2;
mod utils;
mod wildcardsubdomain;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |req, ctx| {
            let host = req.headers().get("host").unwrap_or_default().unwrap_or_default();
            let domain = match ctx.var("WILDCARDSUBDOMAIN_DOMAIN") {
                Ok(domain) => domain.to_string(),
                Err(_) => "owari.shop".to_string(),
            };
            let hostdata = wildcardsubdomain::Hostdata::new(host, domain);
            Response::from_html(hostdata.create_html())
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/favicon.ico", |req, ctx| async move {
            let font = match r2::get(&ctx, "Koruri-Extrabold.ttf").await {
                Some(font_bytes) => Font::try_from_vec(font_bytes).unwrap(),
                None => return Response::error("Internal server error: cant find font", 500),
            };

            let host = req.headers().get("host").unwrap_or_default().unwrap_or_default();
            let domain = match ctx.var("WILDCARDSUBDOMAIN_DOMAIN") {
                Ok(domain) => domain.to_string(),
                Err(_) => "owari.shop".to_string(),
            };
            let hostdata = wildcardsubdomain::Hostdata::new(host, domain);
            let emoji = owariya_image(hostdata.decoded_subdomain, font);
            let emoji_png = match write_image(emoji, image::ImageOutputFormat::Ico) {
                Some(emoji_png) => emoji_png,
                None => return Response::error("Internal server error: cant create image", 500),
            };
            Response::from_bytes(emoji_png)
        })
        .get_async("/owariya.png", |req, ctx| async move {
            let font = match r2::get(&ctx, "Koruri-Extrabold.ttf").await {
                Some(font_bytes) => Font::try_from_vec(font_bytes).unwrap(),
                None => return Response::error("Internal server error: cant find font", 500),
            };

            let host = req.headers().get("host").unwrap_or_default().unwrap_or_default();
            let domain = match ctx.var("WILDCARDSUBDOMAIN_DOMAIN") {
                Ok(domain) => domain.to_string(),
                Err(_) => "owari.shop".to_string(),
            };
            let hostdata = wildcardsubdomain::Hostdata::new(host, domain);
            let emoji = owariya_image(hostdata.decoded_subdomain, font);
            let emoji_png = match write_image(emoji, image::ImageOutputFormat::Png) {
                Some(emoji_png) => emoji_png,
                None => return Response::error("Internal server error: cant create image", 500),
            };
            Response::from_bytes(emoji_png)
        })
        .run(req, env)
        .await
}

fn write_image(dynamic: image::DynamicImage, format: image::ImageOutputFormat) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    dynamic.write_to(&mut buf, format).ok()?;
    Some(buf)
}

fn owariya_image(subdomain: String, font: Font) -> image::DynamicImage {
    let height = 256;
    let width = 256;
    let background_color = Rgba([192u8, 192u8, 192u8, 255u8]);
    let font_color = Rgba([0u8, 0u8, 0u8, 255u8]);

    let mut img = image::DynamicImage::new_rgb8(width, height);

    let x = 0;
    let mut y = 0;
    let height_f32 = height as f32;
    let width_f32 = width as f32;

    // fill background gray
    for x in 0..width {
        for y in 0..height {
            img.draw_pixel(x, y, background_color)
        }
    }

    if subdomain.is_empty() {
        let owa = "おわ";
        let riya = "りや";
        let scale_owa = get_scale_by_font(height_f32 / 2.0, width_f32, &font, owa);
        let scale_riya = get_scale_by_font(height_f32 / 2.0, width_f32, &font, riya);
        draw_text_mut(&mut img, font_color, x, y, scale_owa, &font, owa);
        y += height / 2;
        draw_text_mut(&mut img, font_color, x, y, scale_riya, &font, riya);
    } else {
        let owariya = "おわりや";
        let scale_subdomain = get_scale_by_font(height_f32 / 2.0, width_f32, &font, &subdomain);
        let scale_owariya = get_scale_by_font(height_f32 / 2.0, width_f32, &font, owariya);
        draw_text_mut(
            &mut img,
            font_color,
            x,
            y,
            scale_subdomain,
            &font,
            &subdomain,
        );
        y += height / 2;
        draw_text_mut(&mut img, font_color, x, y, scale_owariya, &font, owariya);
    }

    img
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
