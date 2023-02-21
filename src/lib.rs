use worker::*;
use punycode::decode;
use image::Rgba;
use imageproc::drawing::{draw_text_mut, Canvas};
use rusttype::{Font, Scale};

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();
    router
        // return subdomain html
        .get("/", |req, _| {
            let host = req.headers().get("host").unwrap_or_default();
            match host {
                Some(host) => {
                    let (subdomain, host) = parse_host(host.to_string());
                    let subdomain_punycode = convert_punycode(subdomain);
                    let mut title = format!("{}おわりや", subdomain_punycode);
                    let mut message = format!("{}のおわりが売ってる", subdomain_punycode);

                    match subdomain_punycode.as_str() {
                        "jinsei" => {
                            title = "人生おわりや".to_string();
                            message = "もうだめ".to_string();
                        }
                        "konnendomo" => {
                            title = "今年度もおわりや".to_string();
                            message = "おめでとうございます".to_string();
                        }
                        "kotoshimo" => {
                            title = "今年もおわりや".to_string();
                            message = "あけましておめでとうございます".to_string();
                        }
                        "" => {
                            title = "おわりや".to_string();
                            message = "おわりが売ってる".to_string();
                        }
                        _ => {}
                    }
                    let html = create_html(title, message, host);
                    Response::from_html(html)
                }
                None => Response::ok(""),
            }
        })

        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })

        .get("/favicon.ico", |req, _| {
            let host = req.headers().get("host").unwrap_or_default();
            match host {
                Some(host) => {
                    let (subdomain, _) = parse_host(host.to_string());
                    let subdomain_punycode = convert_punycode(subdomain);
                    let emoji = owariya_image(subdomain_punycode);
                    let mut emoji_ico = Vec::new();
                    match emoji.write_to(&mut emoji_ico, image::ImageOutputFormat::Ico) {
                        Ok(_) => Response::from_bytes(emoji_ico),
                        Err(_) => Response::ok(""),
                    }
                }
                None => Response::ok(""),
            }
        })

        .get("/owariya.png", |req, _| {
            let host = req.headers().get("host").unwrap_or_default();
            match host {
                Some(host) => {
                    let (subdomain, _) = parse_host(host.to_string());
                    let subdomain_punycode = convert_punycode(subdomain);
                    let emoji = owariya_image(subdomain_punycode);
                    let mut emoji_png = Vec::new();
                    match emoji.write_to(&mut emoji_png, image::ImageOutputFormat::Png) {
                        Ok(_) => Response::from_bytes(emoji_png),
                        Err(_) => Response::ok(""),
                    }
                }
                None => Response::ok(""),
            }
        })

        .run(req, env)
        .await
}

fn owariya_image(subdomain: String) -> image::DynamicImage {
    let height = 256;
    let width = 256;
    let background_color = Rgba([192u8, 192u8, 192u8, 255u8]);
    let font_color = Rgba([0u8, 0u8, 0u8, 255u8]);
    let font = Font::try_from_vec(Vec::from(include_bytes!("../static/Koruri-Extrabold-web.ttf") as &[u8])).unwrap();

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

    if subdomain == "" {
        let owa = "おわ".to_string();
        let riya = "りや".to_string();
        let scale_owa = get_scale_by_font(height_f32/2.0, width_f32, &font, &owa);
        let scale_riya = get_scale_by_font(height_f32/2.0, width_f32, &font, &riya);
        draw_text_mut(&mut img, font_color, x, y, scale_owa, &font, &owa);
        y += height/2;
        draw_text_mut(&mut img, font_color, x, y, scale_riya, &font, &riya);
    } else {
        let owariya = "おわりや".to_string();
        let scale_subdomain = get_scale_by_font(height_f32/2.0, width_f32, &font, &subdomain);
        let scale_owariya = get_scale_by_font(height_f32/2.0, width_f32, &font, &owariya);
        draw_text_mut(&mut img, font_color, x, y, scale_subdomain, &font, &subdomain);
        y += height/2;
        draw_text_mut(&mut img, font_color, x, y, scale_owariya, &font, &owariya);
    }

    img
}

fn get_scale_by_font(height: f32, width: f32, font: &Font, text: &String) -> Scale {
    let mut glyph_width_sum = 0.0;
    for c in text.chars() {
        let glyph = font.glyph(c).scaled(Scale::uniform(height));
        glyph_width_sum += glyph.h_metrics().advance_width;
    }
    if glyph_width_sum == 0.0 {
        glyph_width_sum = 1.0;
    }
    let scale = Scale {
        x: height * width / glyph_width_sum,
        y: height,
    };
    scale
}

fn parse_host(host: String) -> (String, String) {
    // if owari.shop subdomain will be empty
    let mut subdomain = String::new();
    let domain = host;
    if domain.contains(".owari.shop") {
        subdomain = domain.replace(".owari.shop", "");
    }
    (subdomain, domain)
}

fn convert_punycode(sub: String) -> String {
    let mut subdomain = sub;
    if subdomain.contains("xn--") {
        subdomain = subdomain.replace("xn--", "");
        subdomain = decode(&subdomain).unwrap_or_default();
    }
    subdomain
}

fn create_html(title: String, message: String, domain: String) -> String {
    let html = include_str!("../static/index.html.tmpl");
    html.replace("{{ .Title }}", &title)
        .replace("{{ .Message }}", &message)
        .replace("{{ .Domain }}", &domain)
}