# Colors
RED=\033[0;31m
GREEN=\033[0;32m
MAGENTA=\033[0;35m
NC=\033[0m # No Color

PROGRAM = dn
RC = cargo
RP = run
BP = build --release
CP = clean
LT = clippy
run:
	@printf "${MAGENTA}Running the program...${NC}\n"
	@echo ""
	@$(RC) $(RP)
	
build:
	@printf "${MAGENTA}Building the program...${NC}\n"
	@$(RC) $(BP)
	@sudo cp target/release/$(PROGRAM) /usr/bin
lint:
	@printf "${MAGENTA}Running the program...${NC}\n"
	@$(RC) ${LT}
clean:
	@printf "${MAGENTA}Cleaning the program...${NC}\n"
	@$(RC) $(CP)
