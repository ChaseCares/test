slint::include_modules!();

fn update_helper(ui: MainWindow, mut app_state: AppState) {
    if ui.get_checking() {
        app_state.add_to_log("Already checking for updates...");
        ui.set_log(app_state.log.clone().into());
        return;
    } else {
        ui.set_checking(true);
        app_state.add_to_log("Checking for updates...");
        ui.set_log(app_state.log.clone().into());
    }

    let current_version = env!("CARGO_PKG_VERSION");
    app_state.add_to_log(format!("Current version: v{}", current_version).as_str());
    ui.set_log(app_state.log.clone().into());

    let status = self_update::backends::github::Update::configure()
        .repo_owner("ChaseCares")
        .repo_name("test")
        .bin_name("test")
        .bin_path_in_archive("{{ bin }}-{{ version }}-{{ target }}/{{ bin }}")
        .show_download_progress(true)
        .current_version(current_version)
        .build()
        .unwrap();

    let latest = status.get_latest_release().unwrap();

    if self_update::version::bump_is_greater(current_version, &latest.version).unwrap() {
        println!(
            "New update Available: v{}, current_version: v{current_version}",
            latest.version
        );
        app_state.add_to_log(format!("New update Available: v{}", latest.version).as_str());
        ui.set_log(app_state.log.clone().into());

        ui.set_ver(latest.version.into());
    } else if current_version == latest.version {
        println!("You are already using the latest version.");
        app_state.add_to_log("You are already using the latest version.");
        ui.set_log(app_state.log.clone().into());
    } else {
        println!("You are using a newer version than the latest.");
        app_state.add_to_log("You are using a newer version than the latest.");
        ui.set_log(app_state.log.clone().into());
    }
}

#[derive(Default, Clone)]
struct AppState {
    log: String,
    self_update: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            log: String::new(),
            self_update: false,
        }
    }

    fn add_to_log(&mut self, message: &str) {
        self.log.push_str(message);
        self.log.push('\n');
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_weak = ui.as_weak();

    let mut app_state = AppState::new();
    app_state.add_to_log("Welcome to the test app!");
    ui.set_log(app_state.log.as_str().into());

    ui.on_check_update(move || {
        update_helper(ui_weak.upgrade().unwrap(), app_state.clone());
    });

    ui.on_self_update(move || {
        app_state.self_update = true;
        ui_weak
            .upgrade()
            .unwrap()
            .set_self_updateing(app_state.self_update);
    });

    ui.run()
}
