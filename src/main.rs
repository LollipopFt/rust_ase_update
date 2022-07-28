use octocrab::models::repos::Release;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let aspath = Path::new(&args[1]);
    let skpath = Path::new(&args[2]);
    let octocrab = octocrab::instance();
    let aseprite = octocrab
        .repos("aseprite", "aseprite")
        .releases()
        .list()
        .per_page(5)
        .page(1u32)
        .send()
        .await
        .expect("failed to get aseprite");
    for release in aseprite {
        if let Some(name) = &release.name {
            if !name.contains("beta") {
                get_asset(release, "Aseprite", aspath).await;
                break;
            }
        }
    }

    let skia = octocrab
        .repos("aseprite", "skia")
        .releases()
        .list()
        .per_page(1)
        .page(1u32)
        .send()
        .await
        .expect("failed to get skia");
    get_asset(
        skia.into_iter().next().expect(""),
        "Windows-Release-x64",
        skpath,
    )
    .await;
}

async fn get_asset(release: Release, name: &str, path: &Path) {
    for asset in release.assets {
        if asset.name.contains(name) {
            let url_load: String = asset.browser_download_url.into();
            let response = reqwest::get(url_load).await.expect("");
            let mut file = File::create(path).expect("could not create file.");
            let content = response.bytes().await.expect("failed response");
            file.write_all(content.as_ref())
                .expect("could not write content.");
        }
    }
}
