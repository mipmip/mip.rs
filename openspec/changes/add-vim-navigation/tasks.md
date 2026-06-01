## 1. Scroll commands

- [ ] 1.1 Add `scroll_down` to `execute_command()` — `window.scrollBy(0, 60)`
- [ ] 1.2 Add `scroll_up` — `window.scrollBy(0, -60)`
- [ ] 1.3 Add `scroll_page_down` — `window.scrollBy(0, window.innerHeight)`
- [ ] 1.4 Add `scroll_page_up` — `window.scrollBy(0, -window.innerHeight)`
- [ ] 1.5 Add `scroll_half_down` — `window.scrollBy(0, window.innerHeight/2)`
- [ ] 1.6 Add `scroll_half_up` — `window.scrollBy(0, -window.innerHeight/2)`
- [ ] 1.7 Add `scroll_top` — `window.scrollTo(0, 0)`
- [ ] 1.8 Add `scroll_bottom` — `window.scrollTo(0, document.body.scrollHeight)`

## 2. Heading navigation commands

- [ ] 2.1 Add `scroll_next_heading` to `execute_command()` — JS to find next `h1-h6[id]` below viewport, `scrollIntoView`
- [ ] 2.2 Add `scroll_prev_heading` — JS to find previous `h1-h6[id]` above viewport, `scrollIntoView`
- [ ] 2.3 Unit test: verify the JS strings are valid (render a document with headings, execute scroll_next/prev, check scroll position changes)

## 3. Key sequence support in keybinding registry

- [ ] 3.1 Add `BindingAction` enum to `command.rs`: `Command(String)` for single combos, `SequencePrefix(HashMap<KeyCombo, String>)` for first keys of sequences
- [ ] 3.2 Extend `parse_key_combo` (or add `parse_binding_str`) to detect comma-separated sequences and return `Vec<KeyCombo>`
- [ ] 3.3 Extend `KeybindingRegistry` storage from `HashMap<KeyCombo, String>` to `HashMap<KeyCombo, BindingAction>`
- [ ] 3.4 Update `register_str` to handle both `"g,g"` (sequence) and `"ctrl+p"` (single) binding strings
- [ ] 3.5 Add `lookup_sequence(&self, first: &KeyCombo, second: &KeyCombo) -> Option<&str>` method
- [ ] 3.6 Update existing `lookup()` to return a result indicating whether the match is a direct command or a sequence prefix
- [ ] 3.7 Unit tests: parse `"g,g"` into two KeyCombos, register sequence, lookup returns correct command
- [ ] 3.8 Unit tests: single combo bindings still work unchanged after refactor
- [ ] 3.9 Unit tests: sequence and single combo on the same first key (e.g. `g` → some command AND `g,g` → another) — sequence prefix takes priority, single binding on `g` is not possible
- [ ] 3.10 Unit test: parse `"ctrl+g,g"` (modifier on first key of sequence)

## 4. Pending-key state in window key handler

- [ ] 4.1 Add `pending_key: Rc<Cell<Option<(KeyCombo, std::time::Instant)>>>` in the window key handler closure
- [ ] 4.2 On keypress with pending_key set: check 500ms timeout, look up sequence, execute or discard
- [ ] 4.3 On keypress without pending_key: look up in registry, if SequencePrefix set pending and consume, if Command execute, else propagate
- [ ] 4.4 Clear pending_key when command bar opens
- [ ] 4.5 Skip registry lookup when focused widget is a TreeView (let TreeView j/k handlers work)

## 5. Default keybindings

- [ ] 5.1 Register default scroll bindings in `KeybindingRegistry::with_defaults()`: j/k, up/down, ctrl+f/b, ctrl+d/u, pageup/pagedown, home/end, shift+g, n/shift+n
- [ ] 5.2 Register `"g,g" = "scroll_top"` as default sequence binding
- [ ] 5.3 Add all scroll commands and `g,g` example to the `--initconf` default config template
- [ ] 5.4 Add scroll commands to the command list comment in the initconf template

## 6. Automated tests

- [ ] 6.1 Unit tests in `command.rs`: `parse_binding_str("g,g")` returns vec of two KeyCombos
- [ ] 6.2 Unit tests: `parse_binding_str("ctrl+p")` returns single KeyCombo (backward compat)
- [ ] 6.3 Unit tests: registry with sequence — lookup first key returns SequencePrefix, lookup_sequence returns command
- [ ] 6.4 Unit tests: registry with mixed single + sequence bindings
- [ ] 6.5 Integration tests in `tests/config_test.rs`: keybindings config with sequence syntax `"g,g"` parses and registers correctly

## 7. Manual verification

- [ ] 7.1 Test j/k scrolls document up/down
- [ ] 7.2 Test Ctrl+f/Ctrl+b scrolls full page
- [ ] 7.3 Test Ctrl+d/Ctrl+u scrolls half page
- [ ] 7.4 Test shift+G scrolls to bottom, g+g scrolls to top
- [ ] 7.5 Test n/N jumps between headings
- [ ] 7.6 Test j/k works in TOC (not intercepted by scroll commands)
- [ ] 7.7 Test keys don't fire when command bar is open
- [ ] 7.8 Test overriding default bindings via config.toml
- [ ] 7.9 Test g+g with slow second keypress (>500ms) does nothing
