## ADDED Requirements

### Requirement: Self-closing Hugo shortcodes
The preprocessor SHALL detect self-closing Hugo shortcodes in both `{{< name args >}}` and `{{% name args %}}` forms and replace them with styled HTML inline elements.

#### Scenario: Angle-bracket shortcode
- **WHEN** the markdown contains `{{< figure src="img.png" title="Caption" >}}`
- **THEN** it SHALL be replaced with a `<span class="dialect-inline dialect-hugo">` element displaying the shortcode name and arguments

#### Scenario: Percent shortcode without closing tag
- **WHEN** the markdown contains `{{% ref "other-post.md" %}}`
- **THEN** it SHALL be replaced with a `<span class="dialect-inline dialect-hugo">` element displaying the shortcode name and arguments

### Requirement: Paired Hugo shortcodes
The preprocessor SHALL detect paired Hugo shortcodes (`{{% name %}}...{{% /name %}}`) and render the opening/closing tags as dialect labels while preserving the inner content for normal markdown rendering.

#### Scenario: Paired notice shortcode
- **WHEN** the markdown contains `{{% notice warning %}}Be careful!{{% /notice %}}`
- **THEN** the opening tag SHALL be replaced with a `<div class="dialect-block dialect-hugo">` with a label showing "notice warning", the inner content "Be careful!" SHALL be preserved as renderable markdown, and the closing tag SHALL close the div

#### Scenario: Multiline paired shortcode
- **WHEN** a paired shortcode spans multiple lines with markdown content inside
- **THEN** the inner markdown content SHALL be rendered normally (headings, lists, etc.) within the dialect block wrapper
