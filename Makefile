.PHONY: elf_build server_run server_start_holesky server_start_holesky_stage

elf_build:
	@echo "Building ELF file..."
	@cd program && cargo prove build
	@echo "ELF file built successfully."

server_run:
	@echo "Running server..."
	@cd server && cargo run --release

server_start_holesky:
	@echo "Running server with Holesky config..."
	RPC_URL=https://ethereum-holesky.publicnode.com NETWORK=holesky cargo run --manifest-path server/Cargo.toml

server_start_holesky_stage:
	@echo "Running server with HoleskyStage config..."
	RPC_URL=https://ethereum-holesky.publicnode.com NETWORK=holesky-stage cargo run --manifest-path server/Cargo.toml
