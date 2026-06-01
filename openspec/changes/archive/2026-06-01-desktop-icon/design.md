## Context

The `icons/` directory has 6 PNGs (16–512px) and one SVG. The PNGs are ugly and should be removed. The SVG is good. No icon is set on the GTK window or application. No `.desktop` file exists.

## Goals / Non-Goals

**Goals:**
- Clean up `icons/` to SVG only
- Application icon visible in taskbar/dock/window manager
- `.desktop` file for app launchers (GNOME, KDE, etc.)
- Proper Nix packaging with icon and desktop file installation

**Non-Goals:**
- Multiple icon sizes (SVG scales; Nix/GTK can handle SVG directly)
- Windows/macOS icon formats (Linux-only)

## Decisions

### SVG as the single icon source

**Choice**: Keep only `icons/mip-icon.svg`. Delete all PNGs. The SVG is used for both the desktop file and the runtime icon.

**Rationale**: SVG scales to any size. No need to maintain multiple rasterized versions.

### Set icon via GTK icon theme name

**Choice**: Install the SVG to `share/icons/hicolor/scalable/apps/mip.svg` in package.nix. Set the icon name on the GTK Application via `set_default_icon_name("mip")` or by matching the application ID.

**Rationale**: This is the standard Freedesktop way. GTK looks up icons by name from the icon theme. The `hicolor` theme is the fallback that all desktops support.

### .desktop file

```ini
[Desktop Entry]
Name=MiP
Comment=Markdown Instant Preview
Exec=mip %f
Icon=mip
Terminal=false
Type=Application
Categories=Utility;TextEditor;
MimeType=text/markdown;text/x-markdown;
```

**Key fields**:
- `Exec=mip %f` — `%f` passes the file argument from file managers
- `MimeType` — associates mip with markdown files for "Open with"
- `Icon=mip` — references the installed icon by name

### package.nix installation

```nix
postInstall = ''
  mkdir -p $out/share/icons/hicolor/scalable/apps
  cp icons/mip-icon.svg $out/share/icons/hicolor/scalable/apps/mip.svg
  mkdir -p $out/share/applications
  cp mip.desktop $out/share/applications/mip.desktop
'';
```

## Risks / Trade-offs

- [SVG rendering in taskbar] → Some older window managers may not render SVG icons well. Mitigation: SVG is simple (no complex filters), should render fine everywhere.
- [Icon theme cache] → Some desktops need `gtk-update-icon-cache` after installing. Nix handles this via hooks.
