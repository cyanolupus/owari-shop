# owari.shop

## Usage 

```bash
# Put assets (fonts)
wrangler r2 bucket create owari-shop
wrangler r2 object put owari-shop/Koruri-Extrabold.ttf --file="static/Koruri-Extrabold.ttf"

# Deploy
wrangler publish

# Or develop locally
wrangler dev
```

Read the latest `worker` crate documentation here: https://docs.rs/worker
