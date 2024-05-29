# owari.shop

## Usage

```bash
# Install wrangler
npm install wrangler --save-dev

# Put assets (fonts)
npx wrangler r2 bucket create owari-shop
npx wrangler r2 object put owari-shop/font.ttf --file="static/Koruri-Extrabold.ttf"

# Deploy
npx wrangler deploy

# Or develop locally
npx wrangler dev
```

## Configuration

Edit `wrangler.toml` to configure your project.
https://developers.cloudflare.com/workers/cli-wrangler/configuration

### Workers Variables

- `WORKERS_RS_VERSION` - The version of the `workers-rs` crate to use. Defaults to `0.0.16`.
- `WILDCARDSUBDOMAIN_DOMAIN` - The domain to use for the wildcard subdomain.
- `WILDCARDSUBDOMAIN_FONT` - The path to favicon's font file in bucket.
- `WILDCARDSUBDOMAIN_TOP_HALF_TEXT` - The text to display in the top half of favicon.
- `WILDCARDSUBDOMAIN_BOTTOM_HALF_TEXT` - The text to display in the bottom half of favicon.
- `WILDCARDSUBDOMAIN_HEIGHT`, `WILDCARDSUBDOMAIN_WIDTH` - The height and width of favicon.
- `WILDCARDSUBDOMAIN_BACKGROUND_COLOR` - The background color of favicon.
- `WILDCARDSUBDOMAIN_FONT_COLOR` - The text color of favicon.

```toml
vars = { WORKERS_RS_VERSION = "0.3.0", WILDCARDSUBDOMAIN_DOMAIN = "owari.shop" }
```

### Workers Routes

```toml
routes = [
    "owari.shop/*",
    "*.owari.shop/*",
]
```

Read the latest `worker` crate documentation here: https://docs.rs/worker
