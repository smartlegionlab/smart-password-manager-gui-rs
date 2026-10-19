use smartpasslib::{SmartPassword, SmartPasswordManager};

pub const APP_NAME: &str = "smart-password-manager";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Alexander Suvorov";
pub const APP_REPO_URL: &str = "https://github.com/smartlegionlab/smart-password-manager-gui-rs";
pub const APP_SITE_URL: &str = "https://smartlegionlab.github.io";
pub const APP_LICENSE: &str = "BSD-3-Clause";

pub const MIN_SECRET_LEN: usize = 12;
pub const MIN_PASSWORD_LEN: usize = 12;
pub const MAX_PASSWORD_LEN: usize = 100;
pub const MAX_DESCRIPTION_LEN: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialog {
    None,
    Add,
    Get,
    Edit,
    Verify,
    Delete,
    About,
    Help,
    Shortcuts,
}

#[derive(Debug)]
pub struct AddState {
    pub description: String,
    pub secret: String,
    pub secret_visible: bool,
    pub length: String,
    pub error: Option<String>,
}

impl AddState {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            secret: String::new(),
            secret_visible: false,
            length: "16".to_string(),
            error: None,
        }
    }

    pub fn valid(&self) -> bool {
        !self.description.trim().is_empty()
            && self.description.chars().count() <= MAX_DESCRIPTION_LEN
            && self.secret.chars().count() >= MIN_SECRET_LEN
            && self
                .length
                .parse::<usize>()
                .map(|v| (MIN_PASSWORD_LEN..=MAX_PASSWORD_LEN).contains(&v))
                .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct GetState {
    pub key: String,
    pub description: String,
    pub secret: String,
    pub secret_visible: bool,
    pub password: Option<String>,
    pub error: Option<String>,
}

impl GetState {
    pub fn new(entry: &SmartPassword) -> Self {
        Self {
            key: entry.public_key.clone(),
            description: entry.description.clone(),
            secret: String::new(),
            secret_visible: false,
            password: None,
            error: None,
        }
    }

    pub fn valid(&self) -> bool {
        self.secret.chars().count() >= MIN_SECRET_LEN
    }
}

#[derive(Debug)]
pub struct EditState {
    pub key: String,
    pub description: String,
    pub length: String,
    pub error: Option<String>,
}

impl EditState {
    pub fn new(entry: &SmartPassword) -> Self {
        Self {
            key: entry.public_key.clone(),
            description: entry.description.clone(),
            length: entry.length.to_string(),
            error: None,
        }
    }

    pub fn valid(&self) -> bool {
        !self.description.trim().is_empty()
            && self.description.chars().count() <= MAX_DESCRIPTION_LEN
            && self
                .length
                .parse::<usize>()
                .map(|v| (MIN_PASSWORD_LEN..=MAX_PASSWORD_LEN).contains(&v))
                .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct VerifyState {
    pub key: String,
    pub description: String,
    pub secret: String,
    pub secret_visible: bool,
    pub result: Option<bool>,
    pub error: Option<String>,
}

impl VerifyState {
    pub fn new(entry: &SmartPassword) -> Self {
        Self {
            key: entry.public_key.clone(),
            description: entry.description.clone(),
            secret: String::new(),
            secret_visible: false,
            result: None,
            error: None,
        }
    }

    pub fn valid(&self) -> bool {
        self.secret.chars().count() >= MIN_SECRET_LEN
    }
}

pub struct App {
    pub manager: SmartPasswordManager,
    pub keys: Vec<String>,
    pub filtered_keys: Vec<String>,
    pub selected: usize,
    pub search: String,
    pub dialog: Dialog,
    pub add: AddState,
    pub get: Option<GetState>,
    pub edit: Option<EditState>,
    pub verify: Option<VerifyState>,
    pub delete_target: Option<String>,
    pub message: Option<(String, bool)>,
    pub status: String,
    pub clipboard: Option<arboard::Clipboard>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let manager = SmartPasswordManager::new().expect("Failed to initialize password manager");
        let clipboard = arboard::Clipboard::new().ok();

        let mut app = Self {
            manager,
            keys: Vec::new(),
            filtered_keys: Vec::new(),
            selected: 0,
            search: String::new(),
            dialog: Dialog::None,
            add: AddState::new(),
            get: None,
            edit: None,
            verify: None,
            delete_target: None,
            message: None,
            status: "Ready".to_string(),
            clipboard,
        };
        app.refresh();
        app
    }

    pub fn refresh(&mut self) {
        let mut keys: Vec<String> = self.manager.passwords().keys().cloned().collect();
        keys.sort();
        self.keys = keys;
        self.apply_filter();
    }

    pub fn apply_filter(&mut self) {
        let q = self.search.trim().to_lowercase();
        if q.is_empty() {
            self.filtered_keys = self.keys.clone();
        } else {
            self.filtered_keys = self
                .keys
                .iter()
                .filter(|k| {
                    if let Some(entry) = self.manager.get(k) {
                        entry.description.to_lowercase().contains(&q)
                            || k.to_lowercase().contains(&q)
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();
        }

        if self.filtered_keys.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.filtered_keys.len() {
            self.selected = self.filtered_keys.len() - 1;
        }
    }

    pub fn selected_entry(&self) -> Option<&SmartPassword> {
        let key = self.filtered_keys.get(self.selected)?;
        self.manager.get(key)
    }

    pub fn selected_key(&self) -> Option<String> {
        self.filtered_keys.get(self.selected).cloned()
    }

    pub fn open_add(&mut self) {
        self.add = AddState::new();
        self.dialog = Dialog::Add;
    }

    pub fn open_get(&mut self) {
        if let Some(entry) = self.selected_entry() {
            self.get = Some(GetState::new(entry));
            self.dialog = Dialog::Get;
        }
    }

    pub fn open_edit(&mut self) {
        if let Some(entry) = self.selected_entry() {
            self.edit = Some(EditState::new(entry));
            self.dialog = Dialog::Edit;
        }
    }

    pub fn open_verify(&mut self) {
        if let Some(entry) = self.selected_entry() {
            self.verify = Some(VerifyState::new(entry));
            self.dialog = Dialog::Verify;
        }
    }

    pub fn open_delete(&mut self) {
        if let Some(key) = self.selected_key() {
            self.delete_target = Some(key);
            self.dialog = Dialog::Delete;
        }
    }

    pub fn open_about(&mut self) {
        self.dialog = Dialog::About;
    }

    pub fn open_help(&mut self) {
        self.dialog = Dialog::Help;
    }

    pub fn open_shortcuts(&mut self) {
        self.dialog = Dialog::Shortcuts;
    }

    pub fn close_dialog(&mut self) {
        self.dialog = Dialog::None;
        self.get = None;
        self.edit = None;
        self.verify = None;
        self.delete_target = None;
    }

    pub fn submit_add(&mut self) {
        let description = self.add.description.trim().to_string();
        let secret = self.add.secret.clone();
        let length = match self.add.length.parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                self.add.error = Some("Invalid length".to_string());
                return;
            }
        };

        let public_key = match smartpasslib::generate_public_key(&secret) {
            Ok(k) => k,
            Err(e) => {
                self.add.error = Some(format!("Failed: {e}"));
                return;
            }
        };

        if self.manager.get(&public_key).is_some() {
            self.add.error = Some("An entry with this secret already exists".to_string());
            return;
        }

        let entry = match SmartPassword::new(public_key, description, length) {
            Ok(e) => e,
            Err(e) => {
                self.add.error = Some(format!("Failed: {e}"));
                return;
            }
        };

        match self.manager.add(entry) {
            Ok(_) => {
                self.refresh();
                self.close_dialog();
                self.message = Some(("Entry added".to_string(), false));
                self.status = "Entry added".to_string();
            }
            Err(e) => {
                self.add.error = Some(format!("Save failed: {e}"));
            }
        }
    }

    pub fn submit_get(&mut self) {
        let state = match &mut self.get {
            Some(s) => s,
            None => return,
        };

        let entry = match self.manager.get(&state.key) {
            Some(e) => e.clone(),
            None => {
                state.error = Some("Entry not found".to_string());
                return;
            }
        };

        let valid = match smartpasslib::verify_secret(&state.secret, &entry.public_key) {
            Ok(v) => v,
            Err(e) => {
                state.error = Some(format!("Verification failed: {e}"));
                return;
            }
        };

        if !valid {
            state.error = Some("Wrong secret phrase".to_string());
            return;
        }

        let password = match smartpasslib::generate_smart_password_sync(&state.secret, entry.length)
        {
            Ok(p) => p,
            Err(e) => {
                state.error = Some(format!("Generation failed: {e}"));
                return;
            }
        };

        state.password = Some(password);
        state.error = None;
        self.status = format!("Password generated for '{}'", entry.description);
    }

    pub fn submit_edit(&mut self) {
        let state = match &mut self.edit {
            Some(s) => s,
            None => return,
        };

        let description = state.description.trim().to_string();
        let length = match state.length.parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                state.error = Some("Invalid length".to_string());
                return;
            }
        };

        match self
            .manager
            .update(&state.key, Some(&description), Some(length))
        {
            Ok(true) => {
                self.refresh();
                self.close_dialog();
                self.message = Some(("Entry updated".to_string(), false));
                self.status = "Entry updated".to_string();
            }
            Ok(false) => {
                state.error = Some("Entry not found".to_string());
            }
            Err(e) => {
                state.error = Some(format!("Update failed: {e}"));
            }
        }
    }

    pub fn submit_verify(&mut self) {
        let state = match &mut self.verify {
            Some(s) => s,
            None => return,
        };

        let entry = match self.manager.get(&state.key) {
            Some(e) => e.clone(),
            None => {
                state.error = Some("Entry not found".to_string());
                return;
            }
        };

        match smartpasslib::verify_secret(&state.secret, &entry.public_key) {
            Ok(v) => {
                state.result = Some(v);
                state.error = None;
                self.status = if v {
                    "Secret matches".to_string()
                } else {
                    "Secret does NOT match".to_string()
                };
            }
            Err(e) => {
                state.error = Some(format!("Verification failed: {e}"));
            }
        }
    }

    pub fn submit_delete(&mut self) {
        let key = match &self.delete_target {
            Some(k) => k.clone(),
            None => return,
        };

        match self.manager.delete(&key) {
            Ok(true) => {
                self.refresh();
                self.close_dialog();
                self.message = Some(("Entry deleted".to_string(), false));
                self.status = "Entry deleted".to_string();
            }
            Ok(false) => {
                self.close_dialog();
                self.message = Some(("Entry not found".to_string(), true));
            }
            Err(e) => {
                self.close_dialog();
                self.message = Some((format!("Delete failed: {e}"), true));
            }
        }
    }

    pub fn copy_password(&mut self, password: &str) -> bool {
        let ok = crate::clipboard::copy_to_clipboard(password, &mut self.clipboard);
        self.status = if ok {
            "Copied to clipboard".to_string()
        } else {
            "No clipboard available".to_string()
        };
        ok
    }

    pub fn handle_shortcuts(&mut self, ctx: &eframe::egui::Context) {
        use eframe::egui::Key;

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Q)) {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
            return;
        }

        if self.dialog != Dialog::None {
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                self.close_dialog();
            }
            return;
        }

        if ctx.memory(|m| m.focused().is_some()) {
            return;
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::N)) {
            self.open_add();
            return;
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::G)) {
            self.open_get();
            return;
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::E)) {
            self.open_edit();
            return;
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::R)) {
            self.open_verify();
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Delete)) {
            self.open_delete();
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.open_get();
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::F5)) {
            self.refresh();
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::F1)) {
            self.open_help();
            return;
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Slash)) {
            self.open_shortcuts();
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::ArrowDown) || i.key_pressed(Key::J)) {
            if !self.filtered_keys.is_empty() {
                self.selected = (self.selected + 1).min(self.filtered_keys.len() - 1);
            }
        }

        if ctx.input(|i| i.key_pressed(Key::ArrowUp) || i.key_pressed(Key::K)) {
            if self.selected > 0 {
                self.selected -= 1;
            }
        }
    }

    pub fn short_key(key: &str) -> String {
        if key.len() <= 8 {
            return key.to_string();
        }
        format!("{}…{}", &key[..4], &key[key.len() - 4..])
    }

    pub fn truncate(s: &str, max_chars: usize) -> String {
        if max_chars == 0 {
            return String::new();
        }
        let count = s.chars().count();
        if count <= max_chars {
            return s.to_string();
        }
        if max_chars == 1 {
            return "…".to_string();
        }
        let take = max_chars - 1;
        let truncated: String = s.chars().take(take).collect();
        format!("{truncated}…")
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_shortcuts(ui.ctx());
        crate::ui::render(ui, self);
    }
}
