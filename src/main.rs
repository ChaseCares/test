use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

fn update_helper(ui: &MainWindow, app_state: Rc<RefCell<AppState>>) {
    let mut app_state = app_state.borrow_mut();

    if ui.get_checking() {
        app_state.add_to_log("Already checking for updates...", ui);
        return;
    }

    ui.set_checking(true);

    if !app_state.self_update {
        app_state.add_to_log("Checking for updates...", ui);
    }

    let current_version = env!("CARGO_PKG_VERSION");
    if !app_state.self_update {
        app_state.add_to_log(&format!("Current version: v{}", current_version), ui);
    }

    let status = match self_update::backends::github::Update::configure()
        .repo_owner("ChaseCares")
        .repo_name("test")
        .bin_name("test")
        .bin_path_in_archive("{{ bin }}-{{ version }}-{{ target }}/{{ bin }}")
        .show_download_progress(true)
        .current_version(current_version)
        .build()
    {
        Ok(status) => status,
        Err(e) => {
            app_state.add_to_log(&format!("Error configuring update: {}", e), ui);
            ui.set_checking(false);
            return;
        }
    };

    let latest = match status.get_latest_release() {
        Ok(latest) => latest,
        Err(e) => {
            app_state.add_to_log(&format!("Error fetching latest release: {}", e), ui);
            ui.set_checking(false);
            return;
        }
    };

    match self_update::version::bump_is_greater(current_version, &latest.version) {
        Ok(true) => {
            println!(
                "New update available: v{}, current version: v{}",
                latest.version, current_version
            );
            if !app_state.self_update {
                app_state.add_to_log(&format!("New update available: v{}", latest.version), ui);
            }

            if app_state.self_update {
                // TODO! move in to match statement
                ui.set_update_button_text("Up to date".into());
                // TODO! Remove
                app_state.add_to_log("Update successful!", ui);

                // match status.update() {
                //     Ok(_) => {
                //         println!("Update successful!");
                //         app_state.add_to_log("Update successful!", ui);
                //     }
                //     Err(e) => {
                //         println!("Error updating: {}", e);
                //         app_state.add_to_log(&format!("Error updating: {}", e), ui);
                //     }
                // }
            } else {
                ui.set_update_button_text(format!("Update to v{}", latest.version).into());
                println!("app_state.self_update: {}", app_state.self_update);
                app_state.self_update = true;
            }
        }
        Ok(false) if current_version == latest.version => {
            println!("You are already using the latest version.");
            app_state.add_to_log("You are already using the latest version.", ui);
        }
        Ok(false) => {
            println!("You are using a newer version than the latest.");
            app_state.add_to_log("You are using a newer version than the latest.", ui);
        }
        Err(e) => {
            println!("Error comparing versions: {}", e);
            app_state.add_to_log(&format!("Error comparing versions: {}", e), ui);
        }
    }

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

    fn add_to_log(&mut self, message: &str, ui: &MainWindow) {
        self.log.push_str(message);
        self.log.push('\n');
        ui.set_log(self.log.clone().into());
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_weak1 = ui.as_weak();

    let app_state = Rc::new(RefCell::new(AppState::new()));
    ui.set_log(app_state.borrow().log.as_str().into());

    update_helper(&ui, app_state.clone());

    ui.on_check_update({
        let app_state = app_state.clone();
        move || {
            if let Some(ui) = ui_weak1.upgrade() {
                update_helper(&ui, app_state.clone());
            }
        }
    });

    ui.run()
}
