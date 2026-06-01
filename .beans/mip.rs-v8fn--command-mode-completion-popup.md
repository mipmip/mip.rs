---
# mip.rs-v8fn
title: "command mode: completion popup and command name completion"
status: completed
type: task
priority: normal
created_at: 2026-06-01T00:00:00Z
updated_at: 2026-06-01T12:00:00Z
openspec-link: openspec/changes/archive/2026-06-01-completion-popup
---

Two improvements to command mode completion:

1. Tab on partial command names should complete (e.g. :op<tab> → :open)
2. Path completion should show all matching options in a popup below the command bar (like vim's wildmenu), not just cycle through invisibly

Depends on command-mode infrastructure from mip.rs-2t32.
