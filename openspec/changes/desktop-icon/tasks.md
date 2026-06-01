## 1. Clean up icons

- [x] 1.1 Delete `icons/mip-icon-16.png`, `icons/mip-icon-32.png`, `icons/mip-icon-48.png`, `icons/mip-icon-128.png`, `icons/mip-icon-256.png`, `icons/mip-icon-512.png`
- [x] 1.2 Keep `icons/mip-icon.svg`

## 2. Create .desktop file

- [x] 2.1 Create `mip.desktop` in project root with Name, Comment, Exec, Icon, Terminal, Type, Categories, MimeType fields

## 3. Set GTK application icon

- [x] 3.1 In `view.rs`: call `gtk4::Window::set_default_icon_name("mip")` before window creation
- [x] 3.2 Alternatively, ensure the application ID `org.mipmip.mip` matches the icon name convention

## 4. Nix packaging

- [x] 4.1 Add `postInstall` to `package.nix` that installs SVG to `share/icons/hicolor/scalable/apps/mip.svg`
- [x] 4.2 Add `postInstall` step to install `mip.desktop` to `share/applications/mip.desktop`

## 5. Verify

- [x] 5.1 `cargo build` succeeds
- [x] 5.2 `nix build` succeeds
- [x] 5.3 Running mip shows the icon in the taskbar
- [x] 5.4 No PNG files in `icons/`
- [x] 5.5 `.desktop` file is valid (test with `desktop-file-validate` if available)
