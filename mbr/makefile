TOOL_DIR=partition_table_constructor
OUTPUT_DIR=build
TEST_DIR=$(TOOL_DIR)/test
TEST_OUTPUT_DIR=$(OUTPUT_DIR)/test
COMPILE_DIR=$(OUTPUT_DIR)/.compiled

COMPILER_FLAGS=-std=c++11

ASM=nasm
CXX=g++

#---------------------------------------------------------------------------------------------------

all: $(OUTPUT_DIR)/partition_table_constructor $(OUTPUT_DIR)/mbr_bootstrap.bin


# executables --------------------------------------------------------------------------------------

$(OUTPUT_DIR)/partition_table_constructor: $(COMPILE_DIR)/partition_table_constructor.o $(COMPILE_DIR)/config_parser.o $(COMPILE_DIR)/utilities.o $(COMPILE_DIR)/partition_table.o $(COMPILE_DIR)/partition_entry.o $(COMPILE_DIR)/chs_address.o
	$(CXX) $(COMPILER_FLAGS) -o $(OUTPUT_DIR)/partition_table_constructor $(COMPILE_DIR)/partition_table_constructor.o $(COMPILE_DIR)/config_parser.o $(COMPILE_DIR)/utilities.o $(COMPILE_DIR)/partition_table.o $(COMPILE_DIR)/partition_entry.o $(COMPILE_DIR)/chs_address.o



# test builds --------------------------------------------------------------------------------------

test_all: clean config_parser_test partition_table_test
	@echo Test build successful!

config_parser_test: $(TEST_OUTPUT_DIR)/config_parser_test
	@echo config_parser_test compiled without errors!

partition_table_test: $(TEST_OUTPUT_DIR)/partition_table_test
	@echo partition_table_test compiled without errors!

$(TEST_OUTPUT_DIR)/config_parser_test: $(COMPILE_DIR)/config_parser_test.o $(COMPILE_DIR)/config_parser.o
	$(CXX) $(COMPILER_FLAGS) -o $(TEST_OUTPUT_DIR)/config_parser_test $(COMPILE_DIR)/config_parser_test.o $(COMPILE_DIR)/config_parser.o

$(TEST_OUTPUT_DIR)/partition_table_test: $(COMPILE_DIR)/partition_table_test.o $(COMPILE_DIR)/partition_table.o $(COMPILE_DIR)/partition_entry.o $(COMPILE_DIR)/chs_address.o $(COMPILE_DIR)/utilities.o
	$(CXX) $(COMPILER_FLAGS) -o $(TEST_OUTPUT_DIR)/partition_table_test $(COMPILE_DIR)/partition_table_test.o $(COMPILE_DIR)/partition_table.o $(COMPILE_DIR)/partition_entry.o $(COMPILE_DIR)/chs_address.o $(COMPILE_DIR)/utilities.o


# *.o files ----------------------------------------------------------------------------------------

$(COMPILE_DIR)/partition_table_constructor.o: $(TOOL_DIR)/partition_table_constructor.cc $(TOOL_DIR)/config_parser.h $(TOOL_DIR)/exit_codes.h $(TOOL_DIR)/required_params.h $(TOOL_DIR)/utilities.h $(TOOL_DIR)/partition_table.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/partition_table_constructor.o $(TOOL_DIR)/partition_table_constructor.cc

$(COMPILE_DIR)/partition_entry.o: $(TOOL_DIR)/partition_entry.cc $(TOOL_DIR)/partition_entry.h $(TOOL_DIR)/utilities.h $(TOOL_DIR)/typedefs.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/partition_entry.o $(TOOL_DIR)/partition_entry.cc

$(COMPILE_DIR)/chs_address.o: $(TOOL_DIR)/chs_address.cc $(TOOL_DIR)/chs_address.h $(TOOL_DIR)/typedefs.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/chs_address.o $(TOOL_DIR)/chs_address.cc

$(COMPILE_DIR)/partition_table.o: $(TOOL_DIR)/partition_table.cc $(TOOL_DIR)/partition_entry.h $(TOOL_DIR)/partition_table.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/partition_table.o $(TOOL_DIR)/partition_table.cc

$(COMPILE_DIR)/config_parser.o: $(TOOL_DIR)/config_parser.cc $(TOOL_DIR)/config_parser.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/config_parser.o $(TOOL_DIR)/config_parser.cc

$(COMPILE_DIR)/utilities.o: $(TOOL_DIR)/utilities.cc $(TOOL_DIR)/utilities.h $(TOOL_DIR)/chs_address.h $(TOOL_DIR)/typedefs.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/utilities.o $(TOOL_DIR)/utilities.cc

$(COMPILE_DIR)/config_parser_test.o: $(TEST_DIR)/config_parser_test.cc $(TOOL_DIR)/config_parser.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/config_parser_test.o $(TEST_DIR)/config_parser_test.cc

$(COMPILE_DIR)/partition_table_test.o: $(TEST_DIR)/partition_table_test.cc $(TOOL_DIR)/partition_table.h $(TOOL_DIR)/partition_entry.h
	$(CXX) $(COMPILER_FLAGS) -c -o $(COMPILE_DIR)/partition_table_test.o $(TEST_DIR)/partition_table_test.cc


# header dependencies ------------------------------------------------------------------------------

$(TOOL_DIR)/chs_address.h: $(TOOL_DIR)/typedefs.h ;

$(TOOL_DIR)/config_parser.h: ;

$(TOOL_DIR)/exit_codes.h: ;

$(TOOL_DIR)/partition_entry.h: $(TOOL_DIR)/typedefs.h $(TOOL_DIR)/chs_address.h ;

$(TOOL_DIR)/partition_table.h: $(TOOL_DIR)/partition_entry.h ;

$(TOOL_DIR)/required_params.h: ;

$(TOOL_DIR)/utilities.h: $(TOOL_DIR)/typedefs.h $(TOOL_DIR)/chs_address.h ;

$(TOOL_DIR)/typedefs.h: ;


# bootcode binaries --------------------------------------------------------------------------------

$(OUTPUT_DIR)/mbr_bootstrap.bin: mbr_bootstrap.asm
	$(ASM) -f bin -o $(OUTPUT_DIR)/mbr_bootstrap.bin mbr_bootstrap.asm


# utilities ----------------------------------------------------------------------------------------

clean:
	rm -rf $(OUTPUT_DIR)
	mkdir -p $(OUTPUT_DIR)
	mkdir -p $(TEST_OUTPUT_DIR)
	mkdir -p $(COMPILE_DIR)
