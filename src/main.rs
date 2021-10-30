use std::time::Duration;
use tide::Response;
use rand::{thread_rng, Rng, RngCore};
use tide::http::mime;

#[async_std::main]
async fn main() -> tide::Result<()>{
    let mut app = tide::new();
    app.at("/*path").all(handle);
    app.listen("127.0.0.1:8090").await?;
    Ok(())
}

async fn handle(_request: tide::Request<()>) -> Response {
    async_std::task::sleep(Duration::from_secs(15)).await;
    let mut rng = thread_rng();
    let mut body = [0u8; 4096];
    rng.fill_bytes(&mut body);
    let body_str = unsafe {
        std::str::from_utf8_unchecked(&body)
    };
    Response::builder(200)
        .body(format!("<html>{}</html>", body_str))
        .header("x-powered-by", "PHP/7.3.10")
        .header("server", "Apache/2.4.50 (Unix)")
        .content_type(mime::HTML)
        .build()
}
