TARGET = hello-world

GREEN = \033[0;32m
RED = \033[0;31m
RESET = \033[0m

all: $(TARGET)

$(TARGET):
	@if [ ! -f $(TARGET)/Cargo.toml ]; then \
		echo "Creating new Cargo project..."; \
		cargo new $(TARGET) --bin; \
	fi
	cd $(TARGET) && cargo build
	@echo "$(GREEN)Build complete!\nRun $(RED)make run$(GREEN) or $(RED)./$(TARGET)/target/debug/$(TARGET) $(GREEN)to execute the program.$(RESET)"

run:
	cd $(TARGET) && cargo run

clean:
	rm -rf $(TARGET)

re: clean all

.PHONY: all clean re run