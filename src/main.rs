use std::time::Duration;
use tide::{Response, http::headers::HeaderValues};
use rand::{thread_rng, RngCore};
use tide::http::mime;
use dns_lookup::lookup_addr;

#[async_std::main]
async fn main() -> tide::Result<()>{
    let mut app = tide::new();
    app.at("/").all(handle);
    app.at("*path").all(handle);
    app.listen("127.0.0.1:8090").await?;
    Ok(())
}

fn is_google(ip_addr: Option<&str>, user_agent: Option<&HeaderValues>) -> bool {
    if let (Some(ip_addr), Some(user_agent)) = (ip_addr, user_agent) {
        if user_agent.last().as_str().contains("googlebot") {
            if let Ok(ip) = ip_addr.parse() {
                if let Ok(host) = lookup_addr(&ip) {
                    host.ends_with("googlebot.com") || host.ends_with("google.com")
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

async fn handle(request: tide::Request<()>) -> tide::Result {
    if is_google(request.peer_addr(), request.header("User-Agent")) {
        return Ok(Response::builder(301).header("Location", "https://rpi.heep.sax.de/").build())
    }
    async_std::task::sleep(Duration::from_secs(15)).await;
    let mut rng = thread_rng();
    let mut body = [0u8; 4096];
    rng.fill_bytes(&mut body);
    let body_str = unsafe {
        std::str::from_utf8_unchecked(&body)
    };
    Ok(Response::builder(200)
        .body(format!("<!DOCTYPE html><html>{}</html>", body_str))
        .header("x-powered-by", "PHP/7.3.10")
        .header("server", "Apache/2.4.50 (Unix), mod_killing_mtm")
        .content_type(mime::HTML)
        .build())
}
