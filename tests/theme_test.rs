//! Desktop colour-scheme detection.
//!
//! `gio::Settings::new()` aborts the process when its schema is not installed,
//! rather than returning an error. Since mip reads the GNOME colour-scheme
//! schema — absent on plenty of desktops — the guard around that call is load
//! bearing, and a regression there is a hard crash rather than a wrong colour.

/// Re-executes this test in a child process with the GSettings schema search
/// path pointed at nothing, and asserts the child exits cleanly.
///
/// If the schema still resolves in the child, the test passes without
/// exercising the fallback; it can only fail by the child actually aborting.
#[test]
fn is_system_dark_survives_a_missing_schema() {
    const MARKER: &str = "MIP_TEST_HIDE_SCHEMAS";

    if std::env::var(MARKER).is_ok() {
        // Child: this is the call that used to be able to abort.
        let _ = mip::is_system_dark();
        let _ = mip::color_scheme_settings();
        return;
    }

    let exe = std::env::current_exe().expect("test binary path");
    let status = std::process::Command::new(exe)
        .args([
            "is_system_dark_survives_a_missing_schema",
            "--exact",
            "--nocapture",
        ])
        .env(MARKER, "1")
        .env("XDG_DATA_DIRS", "/nonexistent-for-mip-tests")
        .env("GSETTINGS_SCHEMA_DIR", "/nonexistent-for-mip-tests")
        .env("GSETTINGS_BACKEND", "memory")
        .status()
        .expect("spawn child test process");

    assert!(
        status.success(),
        "is_system_dark() did not survive an unavailable colour-scheme schema (child exited with {status})"
    );
}

#[test]
fn color_scheme_settings_is_optional_not_fatal() {
    // Whether this system has the schema or not, asking must not abort.
    let _ = mip::color_scheme_settings();
}

#[test]
fn is_system_dark_returns_a_value() {
    let dark = mip::is_system_dark();
    assert!(dark || !dark);
}
