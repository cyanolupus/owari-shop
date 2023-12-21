use worker::*;

mod utils;
mod wildcardsubdomain;
mod favicon;

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
            let domain = get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_DOMAIN", "owari.shop");
            let hostdata = wildcardsubdomain::Hostdata::new(host, domain);
            Response::from_html(hostdata.create_html())
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/favicon.ico", |req, ctx| async move {
            let host = req.headers().get("host").unwrap_or_default().unwrap_or_default();
            let domain = get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_DOMAIN", "owari.shop");
            let hostdata = wildcardsubdomain::Hostdata::new(host, domain);

            let favicon_generator = favicon::FaviconGenerator::new(
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_FONT", "font.ttf"),
                hostdata.decoded_subdomain,
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_TOP_HALF_TEXT", "おわ"),
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT", "りや"),
            );

            let image_ico = match favicon_generator.write_ico(&ctx).await {
                Some(ico) => ico,
                None => return Response::error("Internal server error: cant create image", 500),
            };
            Response::from_bytes(image_ico)
        })
        .get_async("/owariya.png", |req, ctx| async move {
            let host = req.headers().get("host").unwrap_or_default().unwrap_or_default();
            let domain = get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_DOMAIN", "owari.shop");
            let hostdata = wildcardsubdomain::Hostdata::new(host, domain);

            let favicon_generator = favicon::FaviconGenerator::new(
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_FONT", "font.ttf"),
                hostdata.decoded_subdomain,
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_TOP_HALF_TEXT", "おわ"),
                get_var_or_default(&ctx, "WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT", "りや"),
            );
            let image_png = match favicon_generator.write_png(&ctx).await {
                Some(png) => png,
                None => return Response::error("Internal server error: cant create image", 500),
            };
            Response::from_bytes(image_png)
        })
        .run(req, env)
        .await
}

fn get_var_or_default<D>(ctx: &RouteContext<D>, key: &str, default: &str) -> String {
    match ctx.var(key) {
        Ok(value) => value.to_string(),
        Err(_) => default.to_string(),
    }
}