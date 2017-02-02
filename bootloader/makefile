OUTPUT_DIR=build
TEST_DIR=$(TOOL_DIR)/test
TEST_OUTPUT_DIR=$(OUTPUT_DIR)/test
COMPILE_DIR=$(OUTPUT_DIR)/.compiled

COMPILER_FLAGS=-std=c++11

ASM=nasm
CXX=g++


all: $(OUTPUT_DIR)/vbr_bootstrap.bin


$(OUTPUT_DIR)/vbr_bootstrap.bin: vbr_bootstrap.asm
	$(ASM) -f bin -o $(OUTPUT_DIR)/vbr_bootstrap.bin vbr_bootstrap.asm

clean:
	rm -rf $(OUTPUT_DIR)
	mkdir -p $(OUTPUT_DIR)
	mkdir -p $(TEST_OUTPUT_DIR)
	mkdir -p $(COMPILE_DIR)
