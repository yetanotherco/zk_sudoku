.PHONY: elf_build server_run

elf_build:
	@echo "Building ELF file..."
	@cd program && cargo prove build
	@echo "ELF file built successfully."

server_run:
	@echo "Running server..."
	@cd server && cargo run --release
