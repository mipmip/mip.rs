## vim-navigation

Vim-style document scrolling and heading navigation in the preview.

### Requirements

#### Scroll commands
- MUST support `scroll_down` / `scroll_up` — scroll by one step (~60px)
- MUST support `scroll_page_down` / `scroll_page_up` — scroll by one viewport height
- MUST support `scroll_half_down` / `scroll_half_up` — scroll by half viewport height
- MUST support `scroll_top` — scroll to top of document
- MUST support `scroll_bottom` — scroll to bottom of document
- All scroll commands MUST use instant scrolling (no animation), matching vim behavior

#### Heading navigation
- MUST support `scroll_next_heading` — jump to the next heading below the current viewport position
- MUST support `scroll_prev_heading` — jump to the previous heading above the current viewport position
- Heading navigation MUST work with the anchor `id` attributes on `h1`-`h6` elements
- MUST NOT jump to headings without `id` attributes
- If no next/prev heading exists, MUST do nothing (no wrap-around)

#### Default keybindings
- `j` / `down` → `scroll_down`
- `k` / `up` → `scroll_up`
- `ctrl+f` / `pagedown` → `scroll_page_down`
- `ctrl+b` / `pageup` → `scroll_page_up`
- `ctrl+d` → `scroll_half_down`
- `ctrl+u` → `scroll_half_up`
- `home` → `scroll_top`
- `end` / `shift+g` → `scroll_bottom`
- `g,g` → `scroll_top` (key sequence)
- `n` → `scroll_next_heading`
- `shift+n` → `scroll_prev_heading`

#### Focus context
- Scroll commands MUST only fire when the document (WebView) context is active
- MUST NOT intercept `j`/`k` when a TreeView (sidetoc or quicktoc) is focused
- MUST NOT fire when the command bar is open
- Pending key state (for sequences) MUST be cleared when the command bar opens
