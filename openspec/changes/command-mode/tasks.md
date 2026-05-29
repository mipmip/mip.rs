## 1. Window layout

- [ ] 1.1 Change window child from bare WebView to a vertical `gtk4::Box` containing WebView (expand) + Entry (hidden)
- [ ] 1.2 Style the Entry with monospace font and minimal padding (vim command-line look)

## 2. Key event handling

- [ ] 2.1 On `:` keypress (when entry is hidden), show entry, set text to ":", place cursor at end, grab focus
- [ ] 2.2 On Escape in entry, hide entry, clear text, return focus to WebView
- [ ] 2.3 On Enter in entry, parse and execute command, then hide entry and return focus

## 3. Command infrastructure

- [ ] 3.1 Parse entry text: strip leading `:`, split on whitespace into command + argument
- [ ] 3.2 Implement command dispatch with match on command name

## 4. Built-in commands

- [ ] 4.1 `:q` / `:close` — quit the application
- [ ] 4.2 `:open <path>` / `:o <path>` — re-render with new file (update watcher + server paths)

## 5. Tab completion

- [ ] 5.1 On Tab in entry when text starts with `:open ` or `:o `, extract path fragment
- [ ] 5.2 List directory entries matching the prefix, replace path fragment with first match
- [ ] 5.3 Cycle through matches on repeated Tab presses

## 6. Verify

- [ ] 6.1 `cargo build` succeeds
- [ ] 6.2 `:` shows command bar at bottom
- [ ] 6.3 Escape dismisses command bar
- [ ] 6.4 `:q` quits the application
- [ ] 6.5 `:open examples/with-front-matter.md` switches to that file
- [ ] 6.6 Tab completion works for file paths
