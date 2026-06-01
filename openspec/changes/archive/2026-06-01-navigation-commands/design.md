## Context

Currently `--toc side` creates a Paned layout with a persistent sidebar, and `--toc zathura` creates a Stack that toggles between document and TOC on Tab. These are mutually exclusive, chosen at startup. The command bar already supports runtime commands. This change makes both TOC modes always available as commands.

## Goals / Non-Goals

**Goals:**
- Rename zathura → quicktoc, side → sidetoc everywhere
- Both sidetoc and quicktoc always available, hidden by default
- New runtime commands for toggling/controlling them
- `--runcmd` replaces `--toc` for startup configuration
- `;` command composition
- Sidetoc configurable width and position

**Non-Goals:**
- Configurable keybindings (separate change)
- New TOC display modes beyond sidetoc/quicktoc

## Decisions

### Always build both TOC widgets, show on demand

**Choice**: Always create the TreeView, Paned (for sidetoc), and Stack (for quicktoc). Start with both hidden. Commands show/hide them.

**Layout**:
```
┌─────────────────────────────────────────┐
│ Paned (sidetoc_position: left/right)    │
│ ┌──────────┬────────────────────────┐   │
│ │ TreeView │  Stack                 │   │
│ │ (sidetoc)│  ┌──────────────────┐  │   │
│ │          │  │ WebView (doc)    │  │   │
│ │          │  │ or               │  │   │
│ │          │  │ TreeView (qtoc)  │  │   │
│ │          │  └──────────────────┘  │   │
│ └──────────┴────────────────────────┘   │
│ ┌───────────────────────────────────┐   │
│ │ wildmenu label                    │   │
│ │ command entry                     │   │
│ └───────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

The Paned always exists. When sidetoc is closed, the paned start_child is hidden (paned collapses to full width for the end_child). The Stack inside the end_child handles quicktoc toggle.

**Rationale**: Building both at startup is cheap. Showing/hiding is simpler than creating/destroying widgets at runtime.

### Command composition with `;`

**Choice**: Add `execute_commands(text)` that splits on `;`, trims each part, and calls `execute_command` for each.

```rust
fn execute_commands(text: &str, app: &Application) {
    for part in text.split(';') {
        let (cmd, arg) = parse_command(part.trim());
        execute_command(cmd, arg, app);
    }
}
```

Used by: command bar Enter handler, `--runcmd` flag, and future keybindings.

### `--runcmd` replaces `--toc`

**Choice**: `--runcmd` takes a string argument, can be specified multiple times or use `;` internally.

```bash
# Old way:
mip --toc side README.md

# New way:
mip --runcmd sidetoc_open README.md
mip --runcmd "sidetoc_open; set theme dark" README.md
```

**Config equivalent**: `runcmd` setting in config.toml for persistent startup commands.

```toml
runcmd = "sidetoc_open"
```

### Sidetoc config

```toml
sidetoc_width = 250    # pixels, default 250
sidetoc_position = "left"  # "left" or "right", default "left"
```

These are applied when sidetoc_open is called.

## Risks / Trade-offs

- [Breaking change] → `--toc` flag removed. Config `toc` setting also removed. Mitigation: clear error message suggesting `--runcmd sidetoc_open` or `--runcmd quicktoc`.
- [Widget complexity] → Always having both Paned + Stack is more widgets. Mitigation: hidden widgets are essentially free in GTK4.
- [Command execution timing] → `--runcmd` commands need the window to be fully built. Run them in the `connect_activate` handler after widget setup.
