use reqwest::header::ACCEPT;
use self_update::{
    self, backends::github::ReleaseList, ArchiveKind, Compression, Download, Extract,
};
use std::error::Error;
use std::fs::File;
use std::{env, thread::sleep, time::Duration};
use tempfile::Builder;

fn update() -> Result<(), Box<dyn Error>> {
    // Configure and fetch release list from GitHub
    let releases = ReleaseList::configure()
        .repo_owner("ChaseCares")
        .repo_name("test")
        .build()
        .unwrap()
        .fetch()
        .unwrap();

    println!("Found releases:");
    println!("{:#?}\n", releases);

    println!("{:#?}\n", self_update::get_target());

    // Get the first available release asset for the current target
    let asset = releases
        .first()
        .and_then(|release| release.asset_for(self_update::get_target(), None))
        .ok_or("No suitable release asset found")
        .unwrap();

    println!("Using release asset: {:#?}", asset);

    // Create a temporary directory for the download
    let tmp_dir = Builder::new()
        .prefix("test_")
        .tempdir_in(env::current_dir().unwrap())
        .unwrap();

    println!("Using temp dir: {:#?}", tmp_dir);

    let tmp_tarball_path = tmp_dir.path().join(&asset.name);

    println!("Using temp tarball: {:#?}", tmp_tarball_path);

    // Open a file to write the downloaded asset
    let tmp_tarball = File::create(&tmp_tarball_path).unwrap();

    // Download the release asset
    Download::from_url(&asset.download_url)
        .set_header(ACCEPT, "application/octet-stream".parse().unwrap())
        .download_to(&tmp_tarball)
        .unwrap();

    sleep(Duration::from_secs(15));

    // Extract the downloaded tarball to get the new binary
    let bin_name = std::path::PathBuf::from("test");
    let out = std::path::PathBuf::from("test123");
    Extract::from_source(&tmp_tarball_path)
        .archive(ArchiveKind::Tar(None))
        .extract_into(&out)
        .unwrap();

    // // Replace the current executable with the new binary
    // let new_exe = tmp_dir.path().join(bin_name);
    // self_replace::self_replace(new_exe).unwrap();

    Ok(())
}

fn main() {
    println!("Hello, world!");
    update().unwrap();
}
