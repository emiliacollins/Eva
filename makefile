include config/disk.conf
include config/partitions.conf

OUTPUT_DIR=build
TEST_DIR=$(TOOL_DIR)/test
TEST_OUTPUT_DIR=$(OUTPUT_DIR)/test
COMPILE_DIR=$(OUTPUT_DIR)/.compiled
MBR_DIR=mbr
CONFIG_DIR=config
PARTITION_TABLE_CONFIG=$(CONFIG_DIR)/partitions.conf
DISK_CONFIG=$(CONFIG_DIR)/disk.conf
COMPILER_FLAGS=-std=c++11

ASM=nasm
CXX=g++

#---------------------------------------------------------------------------------------------------

all: $(OUTPUT_DIR)/boot_disk.img

$(OUTPUT_DIR)/boot_disk.img: $(OUTPUT_DIR)/mbr.bin partition1_image partition2_image partition3_image partition4_image disk_skeleton


$(OUTPUT_DIR)/mbr.img: mbr_binary partition_table_binary mbr_skeleton
	dd if=$(OUTPUT_DIR)/mbr.bin of=$(OUTPUT_DIR)/mbr.img conv=notrunc
	dd if=$(OUTPUT_DIR)/partition_table.bin of=$(OUTPUT_DIR)/mbr.img skip=446 bs=1 conv=notrunc


partition_table_binary: partition_table_tool $(PARTITION_TABLE_CONFIG)
	echo -ne "$(PARTITION_TABLE_CONFIG)\n$(OUTPUT_DIR)/partition_table.bin" | ./$(OUTPUT_DIR)/partition_table_constructor 

$(OUTPUT_DIR)/mbr.bin:
	(cd $(MBR_DIR); make)
	cp $(MBR_DIR)/$(OUTPUT_DIR)/mbr_bootstrap.bin $(OUTPUT_DIR)/mbr_bootstrap.bin

partition_table_tool:
	(cd $(MBR_DIR); make)
	cp $(MBR_DIR)/$(OUTPUT_DIR)/Partition_table_constructor $(OUTPUT_DIR)/partition_table_constructor


disk_skeleton: $(DISK_CONFIG)
	dd if=/dev/zero of=$(OUTPUT_DIR)/disk.img bs=$(DISK_SECTOR_SIZE) count=$(DISK_SECTOR_COUNT)

mbr_skeleton:
	dd if=/dev/zero of=$(OUTPUT_DIR)/mbr.img bs=512 count=1

clean:
	rm -rf $(OUTPUT_DIR)
	mkdir -p $(OUTPUT_DIR)
	mkdir -p $(TEST_OUTPUT_DIR)
	mkdir -p $(COMPILE_DIR)
