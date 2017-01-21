TOOL_DIR=mbr_constructor
BOOTCODE_DIR=boot_code
OUTPUT_DIR=build
COMPILE_DIR=$(OUTPUT_DIR)/.compiled

ASM=nasm
CXX=g++

#---------------------------------------------------------------------------------------------------

all: mbr_constructor;

mbr: mbr_skeleton bootcode partition_table
	cp mbr_skeleton mbr.img
	dd if=$(OUTPUT_DIR)/bootcode_1.bin of=$(OUTPUT_DIR)/mbr.img
	dd if=$(OUTPUT_DIR)/bootcode_2.bin of=$(OUTPUT_DIR)/mbr.img bs=1 seek=224
	dd if=$(OUTPUT_DIR)/partition_table.bin of=$(OUTPUT_DIR)/mbr.img bs=1 seek=446

mbr_skeleton: $(OUTPUT_DIR)/mbr_skeleton.bin

partition_table: $(OUTPUT_DIR)/partition_table.bin

bootcode: $(OUTPUT_DIR)/bootcode_1.bin $(OUTPUT_DIR)/bootcode_2.bin


# executables --------------------------------------------------------------------------------------

mbr_constructor: $(COMPILE_DIR)/mbr_constructor.o
	$(CXX) -o $(OUTPUT_DIR)/$(TARGET) $(COMPILE_DIR)/mbr_constructor.o


# debug builds -------------------------------------------------------------------------------------

debug_all: clean chs_address partition_entry partition_table_class utilities
	@echo Debug build successful!

chs_address: $(COMPILE_DIR)/chs_address.o
	@echo chs_address compiled without errors!

partition_entry: $(COMPILE_DIR)/partition_entry.o
	@echo partition_entry compiled without errors!

partition_table_class: $(COMPILE_DIR)/partition_table.o
	@echo partition_table compiled without errors!

utilities: $(COMPILE_DIR)/utilities.o
	@echo utilities compiled without errors!


# *.o files ----------------------------------------------------------------------------------------

$(COMPILE_DIR)/mbr_constructor.o: $(TOOL_DIR)/mbr_constructor.cc
	$(CXX) -c -o $(COMPILE_DIR)/mbr_constructor.o $(TOOL_DIR)/mbr_constructor.cc

$(COMPILE_DIR)/partition_entry.o: $(TOOL_DIR)/partition_entry.cc $(TOOL_DIR)/partition_entry.h $(TOOL_DIR)/utilities.h $(TOOL_DIR)/typedefs.h
	$(CXX) -c -o $(COMPILE_DIR)/partition_entry.o $(TOOL_DIR)/partition_entry.cc

$(COMPILE_DIR)/chs_address.o: $(TOOL_DIR)/chs_address.cc $(TOOL_DIR)/chs_address.h $(TOOL_DIR)/typedefs.h
	$(CXX) -c -o $(COMPILE_DIR)/chs_address.o $(TOOL_DIR)/chs_address.cc

$(COMPILE_DIR)/partition_table.o: $(TOOL_DIR)/partition_table.cc $(TOOL_DIR)/partition_entry.h $(TOOL_DIR)/partition_table.h
	$(CXX) -c -o $(COMPILE_DIR)/partition_table.o $(TOOL_DIR)/partition_table.cc

$(COMPILE_DIR)/utilities.o: $(TOOL_DIR)/utilities.cc $(TOOL_DIR)/utilities.h $(TOOL_DIR)/chs_address.h $(TOOL_DIR)/typedefs.h
	$(CXX) -c -o $(COMPILE_DIR)/utilities.o $(TOOL_DIR)/utilities.cc


# header dependencies ------------------------------------------------------------------------------

$(TOOL_DIR)/chs_address.h: $(TOOL_DIR)/typedefs.h ;

$(TOOL_DIR)/partition_entry.h: $(TOOL_DIR)/typedefs.h $(TOOL_DIR)/chs_address.h ;

$(TOOL_DIR)/partition_table.h: $(TOOL_DIR)/partition_entry.h ;

$(TOOL_DIR)/utilities.h: $(TOOL_DIR)/typedefs.h $(TOOL_DIR)/chs_address.h ;

$(TOOL_DIR)/typedefs.h: ;


# bootcode binaries --------------------------------------------------------------------------------

$(OUTPUT_DIR)/bootcode_1.bin: $(BOOTCODE_DIR)/bootcode_1.asm
	$(ASM) -f bin -o $(OUTPUT_DIR)/bootcode_1.bin bootcode_1.asm


$(OUTPUT_DIR)/bootcode_2.bin: $(BOOTCODE_DIR)/bootcode_2.asm
	$(ASM) -f bin -o $(OUTPUT_DIR)/bootcode_2.bin bootcode_2.asm


# utitlities

clean:
	rm -rf $(COMPILE_DIR)/*.o $(TARGET)
