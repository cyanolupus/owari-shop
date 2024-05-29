use image::{ImageOutputFormat, Rgba};
use worker::*;

mod favicon;
use favicon::{FaviconGenerator, ImageProperties};
mod utils;
mod wildcardsubdomain;
use wildcardsubdomain::Hostdata;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().unwrap().coordinates().unwrap_or_default(),
        req.cf()
            .unwrap()
            .region()
            .unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", index_response)
        .get("/worker-version", version_response)
        .get_async("/favicon.ico", owariya_response_ico)
        .get_async("/owariya.png", owariya_response_png)
        .run(req, env)
        .await
}

fn index_response<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    Response::from_html(
        Hostdata::new(host(&req), domain(&ctx)?).create_html(format!(
            "{}{}",
            ctx.var("WILDCARDSUBDOMAIN_TOP_HALF_TEXT")?,
            ctx.var("WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT")?
        )),
    )
}

fn version_response<D>(_: Request, ctx: RouteContext<D>) -> Result<Response> {
    Response::ok(ctx.var("WORKERS_RS_VERSION")?.to_string())
}

async fn owariya_response_ico<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    owariya_response(req, ctx, ImageOutputFormat::Ico).await
}

async fn owariya_response_png<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    owariya_response(req, ctx, ImageOutputFormat::Png).await
}

async fn owariya_response<D>(
    req: Request,
    ctx: RouteContext<D>,
    image_format: ImageOutputFormat,
) -> Result<Response> {
    let image_properties = ImageProperties::new(
        ctx.var("WILDCARDSUBDOMAIN_HEIGHT")?
            .to_string()
            .parse::<u32>()
            .or(Err("Invalid height"))?,
        ctx.var("WILDCARDSUBDOMAIN_WIDTH")?
            .to_string()
            .parse::<u32>()
            .or(Err("Invalid width"))?,
        rgba_from_hex(&ctx.var("WILDCARDSUBDOMAIN_BACKGROUND_COLOR")?.to_string()),
        rgba_from_hex(&ctx.var("WILDCARDSUBDOMAIN_FONT_COLOR")?.to_string()),
    );
    let favicon_generator = FaviconGenerator::new(
        ctx.var("WILDCARDSUBDOMAIN_FONT")?.to_string(),
        owariya_text(
            ctx.var("WILDCARDSUBDOMAIN_TOP_HALF_TEXT")?.to_string(),
            ctx.var("WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT")?.to_string(),
            Hostdata::new(host(&req), domain(&ctx)?).decoded_subdomain,
        ),
        image_properties,
    );

    let image = match favicon_generator.write_image(&ctx, image_format).await {
        Some(image) => image,
        None => return Response::error("Internal server error: cant create image", 500),
    };

    Response::from_bytes(image)
}

fn owariya_text(
    top_half_text: String,
    bottom_half_text: String,
    decoded_subdomain: String,
) -> Vec<String> {
    if decoded_subdomain.is_empty() {
        vec![top_half_text, bottom_half_text]
    } else {
        vec![
            decoded_subdomain,
            format!("{}{}", top_half_text, bottom_half_text),
        ]
    }
}

fn rgba_from_hex(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
    Rgba([r, g, b, a])
}

fn host(req: &Request) -> String {
    req.headers()
        .get("host")
        .unwrap_or_default()
        .unwrap_or_default()
}

fn domain<D>(ctx: &RouteContext<D>) -> Result<String> {
    ctx.var("WILDCARDSUBDOMAIN_DOMAIN").map(|v| v.to_string())
}
