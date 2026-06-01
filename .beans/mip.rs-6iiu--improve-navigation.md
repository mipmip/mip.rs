---
# mip.rs-6iiu
title: improve navigation
status: completed
type: task
priority: normal
created_at: 2026-06-01T10:13:03Z
updated_at: 2026-06-01T12:00:00Z
openspec-link: openspec/changes/archive/2026-06-01-navigation-commands
---

The sidebar and zathura navigation need improvements.

- zathura style navbar should be renamed to quicktoc and sidebar should be renamed to sidetoc everywhere in the application and in the specs
- both functions are standard functionality and should be always available to open and close
- we might need to loose the cli option. We could replace this with a generic --runcmd option
- new commands: :sidetoc_open, :sidetoc_close, :sidetoc_toggle, sidetoc_expand_width, quicktoc
- sidetoc should have configuration settings: sidetoc_width sidetoc_position
- the tab keybinding should be removed and replaced with the possibility to configure keybindings
