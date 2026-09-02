pub mod command;
pub mod config;
pub mod history;
pub mod markdown;
pub mod server;
pub mod view;
pub mod watch;

use gtk4::gio;
use gtk4::prelude::SettingsExt;

/// The GSettings schema and key carrying the desktop colour-scheme preference.
pub const COLOR_SCHEME_SCHEMA: &str = "org.gnome.desktop.interface";
pub const COLOR_SCHEME_KEY: &str = "color-scheme";

/// A `gio::Settings` for the desktop colour scheme, or `None` when it is not
/// available on this system.
///
/// The schema lookup is not defensive politeness: `gio::Settings::new()`
/// **aborts the process** when the schema is not installed, which is the normal
/// situation on desktops that are not GNOME. Callers get `None` there and fall
/// back to [`is_system_dark`]'s one-shot detection.
pub fn color_scheme_settings() -> Option<gio::Settings> {
    let source = gio::SettingsSchemaSource::default()?;
    let schema = source.lookup(COLOR_SCHEME_SCHEMA, true)?;
    if !schema.has_key(COLOR_SCHEME_KEY) {
        return None;
    }
    Some(gio::Settings::new(COLOR_SCHEME_SCHEMA))
}

/// Is the desktop currently set to a dark colour scheme?
///
/// Read in-process through `gio::Settings` where the schema exists. The
/// `gsettings` subprocess below is the fallback for systems without it — it
/// used to be the only implementation, and being called from a 500ms timer it
/// meant two process spawns per second for the life of the window.
pub fn is_system_dark() -> bool {
    if let Some(settings) = color_scheme_settings() {
        return settings.string(COLOR_SCHEME_KEY).contains("prefer-dark");
    }

    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", COLOR_SCHEME_SCHEMA, COLOR_SCHEME_KEY])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("prefer-dark") {
            return true;
        }
    }
    // Last resort: some setups only express the preference via GTK_THEME.
    if let Ok(gtk_theme) = std::env::var("GTK_THEME")
        && gtk_theme.to_lowercase().contains("dark")
    {
        return true;
    }
    false
}
