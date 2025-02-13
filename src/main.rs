use binstalk_downloader::{
    download::{Download, PkgFmt},
    remote::{
        header::{HeaderMap, HeaderValue},
        Client,
    },
};
use reqwest::{ClientBuilder, Url};
use std::{num::NonZeroU16, time::UNIX_EPOCH};

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
    if let (Some(url), Some(dir)) = (std::env::args().next(), std::env::args().next()) {
        let client = create_client().await;
        let files = Download::new(client, Url::parse(&url).unwrap());
        let fmt = PkgFmt::guess_pkg_format(&url).unwrap();
        let mut tmp_dir = std::env::temp_dir();
        let start = std::time::SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_epoch.as_secs().to_string();
        tmp_dir.push(timestamp);
        std::fs::create_dir_all(&tmp_dir).expect("failed to create_dir_all");
        files
            .and_extract(fmt, &tmp_dir)
            .await
            .expect("failed to extract");
        std::fs::rename(&tmp_dir, dir).expect("failed to rename");
    } else {
        println!("download-extract <url> <dir>");
    }
}
