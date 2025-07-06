TARGET = calculator
SRC = main.rs

GREEN = \033[0;32m
RED = \033[0;31m
RESET = \033[0m

all: $(TARGET)

$(TARGET):
	@if [ -f $(TARGET) ] && [ ! -d $(TARGET) ]; then \
		echo "Removing existing file '$(TARGET)' to make room for Cargo project..."; \
		rm -f $(TARGET); \
	fi
	@if [ ! -f $(TARGET)/Cargo.toml ]; then \
		echo "Creating new Cargo project..."; \
		cargo new $(TARGET) --bin; \
		cat ./dependencies >> $(TARGET)/Cargo.toml; \
	fi
	cp $(SRC) $(TARGET)/src/
	cd $(TARGET) && cargo build
	@echo "$(GREEN)Build complete!\nRun $(RED)cargo run <args>$(GREEN) in the $(RED)'$(TARGET)' dir $(GREEN)to execute the program.$(RESET)"

clean:
	rm -rf $(TARGET)

re: clean all

.PHONY: all clean re