## Context

WebKitGTK's WebView navigates to any clicked link by default. mip loads HTML via `load_html()` with a `base_uri` of `http://localhost:{port}/`. Clicking an external link navigates the WebView away from the preview, which is unrecoverable without restarting.

## Goals / Non-Goals

**Goals:**
- External links open in the default browser
- Internal navigation (initial load, localhost assets, anchors) continues to work
- Minimal code change, all in `view.rs`

**Non-Goals:**
- Supporting link clicks in non-Linux environments (xdg-open is Linux-only, mip is Linux-only)
- Custom link handling UI or confirmation dialogs

## Decisions

### Use `decide-policy` signal with NavigationPolicyDecision

**Choice**: Connect to WebView's `decide-policy` signal. For `NavigationPolicyDecision` with `LINK_CLICKED` navigation type, check if the URI is external. If so, ignore the policy and spawn `xdg-open`.

**Rationale**: This is the standard WebKitGTK pattern for intercepting navigation. The signal fires before navigation happens, giving us a chance to block it.

### External vs internal classification

- **Internal**: URI starts with `http://localhost:{port}` or `about:` — allow through
- **External**: everything else (`https://`, `http://` to other hosts, `mailto:`, etc.) — open with `xdg-open`, block navigation

### Fire-and-forget `xdg-open`

**Choice**: `std::process::Command::new("xdg-open").arg(uri).spawn()` — fire and forget, don't wait for the browser to close.

**Rationale**: No need to track the child process. If `xdg-open` fails, silently ignore — the user sees nothing happen, which is acceptable.

## Risks / Trade-offs

- [xdg-open not available] → Extremely unlikely on any Linux desktop. If missing, link click is silently blocked. Acceptable.
- [Anchor links] → `#fragment` links within the page should work since they don't trigger a new navigation request (handled by the browser engine internally). If they do trigger `decide-policy`, the URI will still be `localhost:{port}` based, so they'll be allowed.
