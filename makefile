.DEFAULT_GOAL := help
PROJECTNAME=$(shell basename "$(PWD)")

.PHONY: help
help: makefile
	@echo
	@echo " Available actions in "$(PROJECTNAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo

## init: Install missing dependencies.
init:
	cargo install flutter_rust_bridge_codegen --version 2.13.0 --locked
## :

all: init generate-bindings

generate-bindings:
	@echo "[GENERATING FRB CODE] $@"
	tool/generate_bindings.sh && fvm dart run build_runner build
	@echo "[Done ✅]"







