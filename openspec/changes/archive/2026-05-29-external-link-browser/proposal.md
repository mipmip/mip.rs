## Why

Clicking a link in the mip preview navigates the WebView away from the rendered markdown. External links (GitHub, docs, etc.) should open in the user's default browser instead. mip is a previewer, not a browser.

Bean: mip.rs-unzu

## What Changes

- Intercept link clicks in the WebView via WebKitGTK's `decide-policy` signal
- External URLs (anything not `localhost:{port}`) open with `xdg-open` in the default browser
- Internal navigation (initial load, JS updates, anchors) is allowed through
- No new dependencies — uses `std::process::Command` with `xdg-open`

## Capabilities

### New Capabilities
- `external-links`: External link clicks open in default browser via `xdg-open`

### Modified Capabilities

## Impact

- `src/view.rs`: connect `decide-policy` signal on WebView, add link classification and `xdg-open` logic
