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
	cd server && RPC_URL=https://ethereum-holesky.publicnode.com ALIGNED_EXPLORER_URL=https://holesky.explorer.alignedlayer.com NETWORK=holesky cargo run --release

server_start_holesky_stage:
	@echo "Running server with HoleskyStage config..."
	cd server && RPC_URL=https://ethereum-holesky.publicnode.com ALIGNED_EXPLORER_URL=https://stage.explorer.alignedlayer.com NETWORK=holesky-stage cargo run --release

frontend_build:
	@echo "Building frontend..."
	@cd frontend && npm install && npm run build
	@echo "Frontend built successfully."
