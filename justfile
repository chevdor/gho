VERSION := `toml get Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"

# List available commands
default:
  @just --list --unsorted

# Generate the readme as .md
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

# Generate usage samples
_usage:
	cargo run -q -- --help > doc/usage.adoc

# Generate documentation
doc:_usage
	cargo doc --all-features --no-deps

# Run rustfmt
_fmt:
	cargo fmt --all

# Run clippy
_clippy:
	cargo +nightly clippy --all-features --all-targets

# Run checks such as clippy, rustfmt, etc...
check: _clippy _fmt

# Create a new tag using the current version
git_tag:
	@git tag v{{ VERSION }} -f
	@git tag | sort -rn | head

# Push the newly created tag
git_tag_push:
	@git push origin v{{ VERSION }} -f
