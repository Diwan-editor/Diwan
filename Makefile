# Makefile for Rust Project

# Project settings
TARGET = target/release/dn
INSTALL_PATH = /usr/local/bin/dn

# Colors
GREEN = \033[0;32m
BLUE = \033[0;34m
YELLOW = \033[1;33m
CYAN = \033[0;36m
RED = \033[0;31m
RESET = \033[0m

.PHONY: all build run clean check format install uninstall help

# Default target
all: build

# Build the project
build:
	@echo -e "$(CYAN)Building the project in release mode...$(RESET)"
	@cargo build --release
	@echo -e "$(GREEN)Build complete!$(RESET)"

# Run the project (debug mode by default)
run:
	@echo -e "$(CYAN)Running the project in debug mode...$(RESET)"
	@cargo run
	@echo -e "$(GREEN)Run complete!$(RESET)"

# Clean build artifacts
clean:
	@echo -e "$(YELLOW)Cleaning up build artifacts...$(RESET)"
	@cargo clean
	@echo -e "$(GREEN)Clean complete!$(RESET)"

# Check for errors and warnings
check:
	@echo -e "$(CYAN)Checking for errors and warnings...$(RESET)"
	@cargo check
	@echo -e "$(GREEN)Check complete!$(RESET)"

# Format the code
format:
	@echo -e "$(CYAN)Formatting the code...$(RESET)"
	@cargo fmt
	@echo -e "$(GREEN)Formatting complete!$(RESET)"

# Install the binary to INSTALL_PATH
install: build
	@echo -e "$(CYAN)Installing binary to $(INSTALL_PATH)...$(RESET)"
	@install -Dm755 $(TARGET) $(INSTALL_PATH)
	@echo -e "$(GREEN)Installed to $(INSTALL_PATH)$(RESET)"

# Uninstall the binary from INSTALL_PATH
uninstall:
	@echo -e "$(RED)Uninstalling binary from $(INSTALL_PATH)...$(RESET)"
	@rm -f $(INSTALL_PATH)
	@echo -e "$(GREEN)Uninstall complete!$(RESET)"

# Display help information
help:
	@echo -e "$(BLUE)Makefile for Rust project$(RESET)"
	@echo ""
	@echo -e "$(YELLOW)Usage: make [target]$(RESET)"
	@echo ""
	@echo -e "$(BLUE)Targets:$(RESET)"
	@echo -e "  $(CYAN)all$(RESET)        - Build the project (release mode)"
	@echo -e "  $(CYAN)build$(RESET)      - Build the project in release mode"
	@echo -e "  $(CYAN)run$(RESET)        - Run the project in debug mode"
	@echo -e "  $(CYAN)clean$(RESET)      - Remove build artifacts"
	@echo -e "  $(CYAN)check$(RESET)      - Check the project for errors and warnings"
	@echo -e "  $(CYAN)format$(RESET)     - Format the source code"
	@echo -e "  $(CYAN)install$(RESET)    - Install the release binary to $(INSTALL_PATH)"
	@echo -e "  $(CYAN)uninstall$(RESET)  - Remove the installed binary from $(INSTALL_PATH)"
	@echo -e "  $(CYAN)help$(RESET)       - Show this help information"
