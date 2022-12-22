run:
	make compthemes
	rm -f .temp.html .temp.seed
	cargo run -- ./README.md

all: test build

compthemes:
	yarn run inliner theme_src/theme1/template-src.html | tail -n +3 | head -n -1 > asset/theme1/template.html

build:
	yarn
	shards
	make compthemes
	crystal build --release src/mip.cr

test:
	crystal spec
clean:
	rm -f ./mip

release:
	@echo run crystal2nix and commit shards.nix
	@echo "you should execute: crelease x.x.x"
