# owari.shop

## Usage

```bash
# Put assets (fonts)
wrangler r2 bucket create owari-shop
wrangler r2 object put owari-shop/font.ttf --file="static/Koruri-Extrabold.ttf"

# Deploy
wrangler publish

# Or develop locally
wrangler dev
```

## Configuration

Edit `wrangler.toml` to configure your project.
https://developers.cloudflare.com/workers/cli-wrangler/configuration

### Workers Variables

- `WORKERS_RS_VERSION` - The version of the `workers-rs` crate to use. Defaults to `0.0.16`.
- `WILDCARDSUBDOMAIN_DOMAIN` - The domain to use for the wildcard subdomain. Defaults to `owari.shop`.
- `WILDCARDSUBDOMAIN_FONT` - The path to favicon's font file in bucket. Defaults to `Koruri-Extrabold.ttf`.
- `WILDCARDSUBDOMAIN_TOP_HALF_TEXT` - The text to display in the top half of favicon. Defaults to `おわ`.
- `WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT` - The text to display in the bottom half of favicon. Defaults to `りや`.
- `WILDCARDSUBDOMAIN_ICO_HEIGHT`, `WILDCARDSUBDOMAIN_ICO_WIDTH` `WILDCARDSUBDOMAIN_PNG_HEIGHT` `WILDCARDSUBDOMAIN_PNG_WIDTH` - The height and width of favicon. Defaults to `256`.
- `WILDCARDSUBDOMAIN_BACKGROUND_COLOR` - The background color of favicon. Defaults to `#c0c0c0ff`.
- `WILDCARDSUBDOMAIN_FONT_COLOR` - The text color of favicon. Defaults to `#000000ff`.

```toml
vars = { WORKERS_RS_VERSION = "0.0.16", WILDCARDSUBDOMAIN_DOMAIN = "owari.shop" }
```

### Workers Routes

```toml
routes = [
    "owari.shop/*",
    "*.owari.shop/*",
]
```

Read the latest `worker` crate documentation here: https://docs.rs/worker
