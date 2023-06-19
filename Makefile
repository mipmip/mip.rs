run:
	make compthemes
	rm -f .temp.html .temp.seed
	cargo run -- ./README.md

all: test build

compthemes:
	yarn run inliner theme_src/theme1/template-src.html | tail -n +3 | head -n -1 > asset/theme1/template.html

build:
	yarn
	make compthemes
	cargo build --release

test:
	@echo WIP

clean:
	rm -fv ./target

release:
	@echo WIP
