## Why

There's no way to search within the preview document or filter the table of contents. Both are basic navigation features expected in any document viewer. WebKitGTK provides a built-in `FindController` API for document search with highlighting, and GTK's `TreeModelFilter` or simple store rebuilds can filter the TOC.

Bean: [mip.rs-vaoq](/home/pim/cLinden/mip.rs/.beans/mip.rs-vaoq--search.md)

## What Changes

- `/` opens a search bar (reusing the command bar widget with `/` prefix)
- **Document focused**: WebKit `FindController` highlights matches live as you type, Enter closes bar and positions at first match, `n`/`N` navigate next/previous
- **TOC focused** (sidetoc or quicktoc): live-filters TOC entries, hiding non-matching headings
- Escape clears search highlights / restores full TOC

## Capabilities

### New Capabilities
- `search`: Vim-style `/` search with document find (WebKit FindController) and TOC filtering

### Modified Capabilities

_(none)_

## Impact

- `src/view.rs`: add `/` key handler to open search bar, `n`/`N` handlers for next/prev, search bar behavior for find and TOC filter modes
- `src/command.rs`: no changes needed (search is not a `:` command, it's a separate `/` mode)
- `src/config.rs`: add search commands to config template documentation
