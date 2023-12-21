use worker::*;

pub async fn get(ctx: &RouteContext<()>, key: &str) -> Option<Vec<u8>> {
    let bucket = ctx.bucket("BUCKET").unwrap();
    let item = bucket.get(key).execute().await.ok()??;
    item.body()?.bytes().await.ok()
}
