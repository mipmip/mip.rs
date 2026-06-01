## 1. Search bar activation

- [ ] 1.1 Add `/` handler in main window key controller: open command bar with `/` prefix, set search mode cell
- [ ] 1.2 Add search mode state: `Rc<Cell<SearchMode>>` with variants `None`, `Document`, `Toc`
- [ ] 1.3 Determine mode by checking if sidetoc/quicktoc TreeView is focused at `/` press time

## 2. Document search (FindController)

- [ ] 2.1 On keystroke in search mode (document): call `webview.find_controller().search()` with case-insensitive + wrap-around options
- [ ] 2.2 Connect to `counted-matches` signal on FindController, display count in wildmenu label ("3/17")
- [ ] 2.3 On Enter: close search bar, store last search text in cell for n/N navigation
- [ ] 2.4 On Escape: close search bar, call `find_controller.search_finish()` to clear highlights

## 3. n/N navigation

- [ ] 3.1 Add `n` handler in main window key controller: call `find_controller.search_next()` if last search exists
- [ ] 3.2 Add `N` (Shift+n) handler: call `find_controller.search_previous()`
- [ ] 3.3 Track `last_search: Rc<RefCell<String>>` — set on Enter, cleared on Escape

## 4. TOC filtering

- [ ] 4.1 On keystroke in search mode (toc): rebuild active TOC store with entries matching the search text (case-insensitive substring)
- [ ] 4.2 On Escape in TOC mode: restore full unfiltered TOC
- [ ] 4.3 On Enter in TOC mode: close search bar, keep filter active

## 5. Verify

- [ ] 5.1 `cargo build` succeeds
- [ ] 5.2 `/` search highlights matches in document, n/N navigate
- [ ] 5.3 `/` in TOC filters headings live
- [ ] 5.4 Escape clears search/filter in both modes
