## Why

mip has no application icon — it shows a generic GTK icon in the taskbar/dock. The `icons/` directory has ugly pre-rendered PNGs alongside a good SVG. There's no `.desktop` file for Linux desktop integration.

Bean: mip.rs-oifq

## What Changes

- Delete all PNG files from `icons/` (keep `mip-icon.svg`)
- Set the GTK application icon from the SVG at runtime
- Create a `mip.desktop` file for Linux desktop integration
- Install the icon and `.desktop` file in `package.nix`

## Capabilities

### New Capabilities
- `desktop-icon`: Application icon in taskbar/dock and desktop file for app launchers

### Modified Capabilities

## Impact

- `icons/`: delete PNG files, keep SVG
- `src/view.rs`: set window/app icon from embedded or installed SVG
- `mip.desktop`: new file for desktop integration
- `package.nix`: install icon to `share/icons/` and `.desktop` to `share/applications/`
