---
# mip.rs-8uxm
title: release workflow
status: in-progress
type: task
priority: normal
created_at: 2026-06-01T12:09:04Z
updated_at: 2026-06-01T14:30:00Z
---

script:
- dropdown /major/minor/hotfix
- version single source of truth
- set version and date in changelog
- auto build common binaries (rpm/deb/appimage/) at release
- compatible with jj and git
