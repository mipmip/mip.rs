pub mod markdown;
pub mod config;
pub mod server;
pub mod view;

pub fn is_system_dark() -> bool {
    // Check GNOME/freedesktop color-scheme setting
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("prefer-dark") {
            return true;
        }
    }
    // Fallback: check GTK_THEME env var for dark variants
    if let Ok(gtk_theme) = std::env::var("GTK_THEME") {
        if gtk_theme.to_lowercase().contains("dark") {
            return true;
        }
    }
    false
}
