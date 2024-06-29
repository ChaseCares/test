fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("ChaseCares")
        .repo_name("test")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{releases:#?}\n");

    // get the first available release
    let asset = releases[0]
        .asset_for(self_update::get_target(), Some("x86_64-unknown-linux-gnu"))
        .unwrap();

    let tmp_dir = tempfile::Builder::new()
        .prefix("self_update")
        .tempdir_in(::std::env::current_dir()?)?;
    let tmp_tarball_path = tmp_dir.path().join(&asset.name);
    let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

    self_update::Download::from_url(&asset.download_url)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .download_to(&tmp_tarball)?;

    let bin_name = std::path::PathBuf::from("self_update_bin");
    self_update::Extract::from_source(&tmp_tarball_path)
        .archive(self_update::ArchiveKind::Tar(Some(
            self_update::Compression::Gz,
        )))
        .extract_file(tmp_dir.path(), &bin_name)?;

    let new_exe = tmp_dir.path().join(bin_name);
    self_replace::self_replace(new_exe)?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
    update().unwrap();
    println!("Updated!")
}
