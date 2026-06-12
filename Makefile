run:
	make compthemes
	rm -f .temp.html .temp.seed
	cargo run -- ./README.md

all: test build

compthemes:
	node scripts/inline-theme.mjs theme_src/theme1/template-src.html > asset/theme1/template.html

# Fail if the committed theme artifact is out of sync with theme_src/.
# Regenerates into a temp file (never touches the committed artifact) and diffs.
check-themes:
	@tmp=$$(mktemp); \
	node scripts/inline-theme.mjs theme_src/theme1/template-src.html > $$tmp; \
	if ! diff -q $$tmp asset/theme1/template.html >/dev/null; then \
		echo "error: asset/theme1/template.html is out of sync with theme_src/"; \
		echo "hint: edit theme_src/, run 'make compthemes'; never edit asset/theme1/template.html directly"; \
		rm -f $$tmp; \
		exit 1; \
	fi; \
	rm -f $$tmp

build:
	yarn
	make compthemes
	cargo build --release

lint:
	cargo fmt --check && cargo clippy -- -D warnings

check: lint check-themes test

test:
	cargo test

coverage:
	bash scripts/update-coverage.sh

clean:
	rm -fv ./target

release:
	bash scripts/release.sh
