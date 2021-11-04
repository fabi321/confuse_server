use std::time::Duration;
use tide::{Response, http::headers::HeaderValues};
use rand::{thread_rng, RngCore};
use tide::http::mime;
use async_std_resolver::resolver_from_system_conf;
use trust_dns_resolver::Name;

struct Config {
    pub googlebot_redirect: String,

}

#[async_std::main]
async fn main() -> tide::Result<()>{
    let mut app = tide::new();
    app.at("/").all(handle);
    app.at("*path").all(handle);
    app.listen("127.0.0.1:8090").await?;
    Ok(())
}

async fn is_google(ip_addr: Option<&str>, user_agent: Option<&HeaderValues>) -> bool {
    if let (Some(ip_addr), Some(user_agent)) = (ip_addr, user_agent) {
        if user_agent.last().as_str().contains("googlebot") {
            if let Ok(ip) = ip_addr.parse() {
                let resolver = resolver_from_system_conf().await.unwrap();
                if let Ok(lookup) = resolver.reverse_lookup(ip).await {
                    if let Some(host) = lookup.into_iter().next() {
                        let googlebot = Name::from_utf8("googlebot.com").unwrap();
                        let google = Name::from_utf8("google.com").unwrap();
                        googlebot.zone_of(&host) || google.zone_of(&host)
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
    } else {
        false
    }
}

async fn handle(request: tide::Request<()>) -> tide::Result {
    if is_google(request.peer_addr(), request.header("User-Agent")).await {
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
        .body(format!("<!DOCTYPE html><html><h1>Wordpress admin page phpinfo pwned you hacked{}</html>", body_str))
        .header("x-powered-by", "PHP/7.3.10")
        .header("server", "Apache/2.4.50 (Unix), mod_killing_mtm")
        .header("x-killed-by", "confuse_server -> https://github.com/fabi321/confuse_server")
        .content_type(mime::HTML)
        .build())
}
