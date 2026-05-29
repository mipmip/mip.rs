## Beans OpenSpec Integration

When I refer to issues like <this-dirname>-rn3b checkout the task
in @.beans/<this-dirname>-rn3b-*.md

In this project we will use these tasks as epics for making openspec proposals.

WHEN you create a proposal at a link to this task in the proposal.md.
WHEN a bean is used to create an proposal change the status to "in-progress"
WHEN a proposal is archived add the link to the archived proposal in the frontmatter of this task like this:

```
openspec-link: openspec/changes/archive/....
```

You are allowed to update these statuses in the task frontmatter:

- in-progress
- todo
- draft
- completed
- scrapped

When making changes you are allowed to update the date/time in `updated_at` in the task frontmatter

Besides updating status and openspec-link, you are NOT ALLOWED to modify the contents of the task file.

## Git/jj

- We use jj
- I don't want Claude to appear as committer or coworker in the git-history

## OpenSpec Archive triggers

- When opsx:archive is called the work should be committed in git. We use jj.
- jj describe/jj new (so the working copy is clean after archiving)
- the changelog should also be updated
