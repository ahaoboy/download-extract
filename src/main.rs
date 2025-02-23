use binstalk_downloader::{
    download::{Download, PkgFmt},
    remote::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
};
use reqwest::{ClientBuilder, Url};
use std::num::NonZeroU16;

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append("User-Agent", HeaderValue::from_static("reqwest"));
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("token {token}")).expect("Authorization token error"),
        );
    };
    headers
}

pub async fn create_client() -> Client {
    let headers = get_headers();
    Client::from_builder(
        ClientBuilder::new().default_headers(headers),
        NonZeroU16::new(10).unwrap(),
        1.try_into().unwrap(),
    )
    .expect("failed to create_client")
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    args.next();
    if let (Some(url), Some(dir)) = (args.next(), args.next()) {
        println!("{url} {dir}");
        let client = create_client().await;
        let download = Download::new(client, Url::parse(&url).unwrap());
        let fmt = PkgFmt::guess_pkg_format(&url).unwrap();
        std::fs::create_dir_all(&dir).expect("failed to create_dir_all");
        download
            .and_extract(fmt, &dir)
            .await
            .expect("failed to extract");
    } else {
        println!("download-extract <url> <dir>");
    }
}
