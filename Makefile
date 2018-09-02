# Compiler configuration
GENERAL_ARGS = --release
FRONTEND_ARGS = $(GENERAL_ARGS) --target wasm32-unknown-unknown

.PHONY: \
	build \
	build-doc \
	coverage \
	lint-rustfmt \
	lint-clippy

ifndef VERBOSE
.SILENT:
else
GENERAL_ARGS += -v
endif

all: build

build:
	cargo web build $(FRONTEND_ARGS)

build-doc:
	cargo doc --all --no-deps

coverage:
	cargo kcov

lint-clippy:
	cargo clippy -- -D warnings

lint-rustfmt:
	cargo fmt
	git diff --exit-code
