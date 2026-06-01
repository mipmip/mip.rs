## 1. Clean up icons

- [ ] 1.1 Delete `icons/mip-icon-16.png`, `icons/mip-icon-32.png`, `icons/mip-icon-48.png`, `icons/mip-icon-128.png`, `icons/mip-icon-256.png`, `icons/mip-icon-512.png`
- [ ] 1.2 Keep `icons/mip-icon.svg`

## 2. Create .desktop file

- [ ] 2.1 Create `mip.desktop` in project root with Name, Comment, Exec, Icon, Terminal, Type, Categories, MimeType fields

## 3. Set GTK application icon

- [ ] 3.1 In `view.rs`: call `gtk4::Window::set_default_icon_name("mip")` before window creation
- [ ] 3.2 Alternatively, ensure the application ID `org.mipmip.mip` matches the icon name convention

## 4. Nix packaging

- [ ] 4.1 Add `postInstall` to `package.nix` that installs SVG to `share/icons/hicolor/scalable/apps/mip.svg`
- [ ] 4.2 Add `postInstall` step to install `mip.desktop` to `share/applications/mip.desktop`

## 5. Verify

- [ ] 5.1 `cargo build` succeeds
- [ ] 5.2 `nix build` succeeds
- [ ] 5.3 Running mip shows the icon in the taskbar
- [ ] 5.4 No PNG files in `icons/`
- [ ] 5.5 `.desktop` file is valid (test with `desktop-file-validate` if available)
