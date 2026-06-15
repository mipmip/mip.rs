---
# mip.rs-8uxm
title: release workflow
status: completed
type: task
priority: normal
created_at: 2026-06-01T12:09:04Z
updated_at: 2026-06-15T18:30:00Z
openspec-link: openspec/changes/archive/2026-06-15-release-workflow
---

script:
- dropdown /major/minor/hotfix
- version single source of truth
- set version and date in changelog
- auto build common binaries (rpm/deb/appimage/) at release
- compatible with jj and git
