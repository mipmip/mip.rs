## Context

mip is a markdown previewer — the docs can be previewed with mip itself (`mip docs/usage.md`). The README currently contains installation, basic usage, a stale TODO list, and development instructions.

## Goals / Non-Goals

**Goals:**
- Single reference page for all user-facing features
- Can be previewed with mip itself
- README stays as a concise project overview

**Non-Goals:**
- Man page generation
- Auto-generated docs from code
- Developer/architecture documentation

## Decisions

### Single file: docs/usage.md

**Choice**: One markdown file covering everything, structured with clear sections.

**Why**: Easy to find, easy to preview with mip, no build tooling needed. The project isn't large enough to warrant multiple doc files.

### Structure

```
docs/usage.md
├── CLI Options (from argh struct)
├── Commands (from COMMANDS list with descriptions)
│   ├── Navigation
│   ├── TOC
│   ├── View
│   └── Settings
├── Configuration
│   ├── File location
│   ├── Settings reference
│   └── Keybindings
├── Keyboard Shortcuts (default bindings)
├── Search (/ and n/N)
└── Command Bar (: mode, history, tab completion)
```

### README slimdown

**Choice**: Keep README to: project description, screenshot, features list, platform notice, installation, link to docs/usage.md, development section, contributing.

**Why**: README is the landing page — keep it short and attractive. Detailed reference belongs in the usage guide.
