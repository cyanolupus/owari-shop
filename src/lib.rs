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
        .get("/", |req, ctx| {
            let host = req
                .headers()
                .get("host")
                .unwrap_or_default()
                .unwrap_or_default();
            let domain = get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_DOMAIN", "owari.shop");
            let hostdata = Hostdata::new(host, domain);
            Response::from_html(hostdata.create_html(format!(
                "{}{}",
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_TOP_HALF_TEXT", "おわ"),
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT", "りや")
            )))
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/favicon.ico", |req, ctx| async move {
            owariya_response(req, ctx, ImageOutputFormat::Ico).await
        })
        .get_async("/owariya.png", |req, ctx| async move {
            owariya_response(req, ctx, ImageOutputFormat::Png).await
        })
        .run(req, env)
        .await
}

async fn owariya_response<D>(
    req: Request,
    ctx: RouteContext<D>,
    image_format: ImageOutputFormat,
) -> Result<Response> {
    let host = req
        .headers()
        .get("host")
        .unwrap_or_default()
        .unwrap_or_default();
    let domain = get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_DOMAIN", "owari.shop");
    let hostdata = Hostdata::new(host, domain);

    let image_properties = ImageProperties::new(
        get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_PNG_HEIGHT", "256")
            .parse::<u32>()
            .unwrap_or(256),
        get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_PNG_WIDTH", "256")
            .parse::<u32>()
            .unwrap_or(256),
        rgba_from_hex(&get_var_or_default(
            &ctx,
            "WILDCARDSUBDOMAIN_BACKGROUND_COLOR",
            "#c0c0c0ff",
        )),
        rgba_from_hex(&get_var_or_default(
            &ctx,
            "WILDCARDSUBDOMAIN_FONT_COLOR",
            "#000000ff",
        )),
    );
    let favicon_generator = FaviconGenerator::new(
        get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_FONT", "font.ttf"),
        owariya_text(&ctx, hostdata.decoded_subdomain),
        image_properties,
    );

    let image = match favicon_generator.write_image(&ctx, image_format).await {
        Some(image) => image,
        None => return Response::error("Internal server error: cant create image", 500),
    };

    Response::from_bytes(image)
}

fn owariya_text<D>(ctx: &RouteContext<D>, decoded_subdomain: String) -> Vec<String> {
    if decoded_subdomain.is_empty() {
        vec![
            get_var_or_default(ctx, "WILDCARDSUBDOMAIN_TOP_HALF_TEXT", "おわ"),
            get_var_or_default(ctx, "WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT", "りや"),
        ]
    } else {
        vec![
            decoded_subdomain,
            format!(
                "{}{}",
                get_var_or_default(ctx, "WILDCARDSUBDOMAIN_TOP_HALF_TEXT", "おわ"),
                get_var_or_default(ctx, "WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT", "りや")
            ),
        ]
    }
}

fn get_var_or_default<D>(ctx: &RouteContext<D>, key: &str, default: &str) -> String {
    match ctx.var(key) {
        Ok(value) => value.to_string(),
        Err(_) => default.to_string(),
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
