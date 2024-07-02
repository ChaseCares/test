slint::include_modules!();

fn update_helper(ui: MainWindow, mut app_state: AppState) {
    if ui.get_checking() {
        app_state.add_to_log("Already checking for updates...");
        ui.set_log(app_state.log.clone().into());
        return;
    }

    ui.set_checking(true);
    app_state.add_to_log("Checking for updates...");
    ui.set_log(app_state.log.clone().into());

    let current_version = env!("CARGO_PKG_VERSION");
    app_state.add_to_log(&format!("Current version: v{}", current_version));
    ui.set_log(app_state.log.clone().into());

    let status = match self_update::backends::github::Update::configure()
        .repo_owner("repo_owner")
        .repo_name("test")
        .bin_name("test")
        .bin_path_in_archive("{{ bin }}-{{ version }}-{{ target }}/{{ bin }}")
        .show_download_progress(true)
        .current_version(current_version)
        .build()
    {
        Ok(status) => status,
        Err(e) => {
            app_state.add_to_log(&format!("Error configuring update: {}", e));
            ui.set_log(app_state.log.clone().into());
            ui.set_checking(false);
            return;
        }
    };

    let latest = match status.get_latest_release() {
        Ok(latest) => latest,
        Err(e) => {
            app_state.add_to_log(&format!("Error fetching latest release: {}", e));
            ui.set_log(app_state.log.clone().into());
            ui.set_checking(false);
            return;
        }
    };

    if self_update::version::bump_is_greater(current_version, &latest.version).unwrap_or(false) {
        println!(
            "New update available: v{}, current version: v{}",
            latest.version, current_version
        );
        app_state.add_to_log(&format!("New update available: v{}", latest.version));
        ui.set_ver(latest.version.into());
    } else if current_version == latest.version {
        println!("You are already using the latest version.");
        app_state.add_to_log("You are already using the latest version.");
    } else {
        println!("You are using a newer version than the latest.");
        app_state.add_to_log("You are using a newer version than the latest.");
    }

    ui.set_log(app_state.log.clone().into());
    ui.set_checking(false);
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
        if let Some(ui) = ui_weak.upgrade() {
            update_helper(ui, app_state.clone());
        }
    });

    ui.run()
}
