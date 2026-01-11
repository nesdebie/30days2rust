TARGET = unit_converter
SRC = main.rs
MODULE = unit_converter.rs

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
	fi
	cp $(SRC) $(MODULE) $(TARGET)/src/
	cd $(TARGET) && cargo build
	@echo "$(GREEN)Build complete!$(RESET)"
	@echo "$(RED)+++ USAGE +++$(RESET)"
	@echo "$(GREEN)cd $(TARGET) ; cargo run -- km2m 5$(RESET)"
	@echo "$(RED)OR$(RESET)"
	@echo "$(GREEN)./$(TARGET)/target/debug/$(TARGET) km2m 5$(RESET)"

clean:
	rm -rf $(TARGET)

re: clean all

.PHONY: all clean re