## 1. Implement decide-policy handler

- [x] 1.1 Connect to `decide-policy` signal on WebView in `view.rs` after creating the webview
- [x] 1.2 Downcast PolicyDecision to NavigationPolicyDecision to get URI and navigation type
- [x] 1.3 For link clicks with external URIs (not `localhost:{port}`), call `xdg-open` and ignore the policy
- [x] 1.4 Allow all other navigation (initial load, resource requests, internal)

## 2. Verify

- [x] 2.1 `cargo build` succeeds
- [x] 2.2 `mip README.md` loads normally (initial navigation allowed)
- [x] 2.3 Clicking an external link (e.g. GitHub URL in README) opens default browser
- [x] 2.4 WebView stays on the preview after clicking an external link
- [x] 2.5 Images and local assets still load
