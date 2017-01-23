#ifndef MBR_LAYOUT_H
#define MBR_LAYOUT_H

const unsigned int BYTE_SIZE = 1;
const unsigned int WORD_SIZE = 2;
const unsigned int DWORD_SIZE = 4;


const unsigned int BOOTCODE_1_LENGTH = 218;
const unsigned int BOOTCODE_2_LENGTH = 216;
const unsigned int DISK_SIGNATURE_LENGTH = DWORD_SIZE;
const unsigned int PARTITION_ENTRY_COUNT = 4;
const unsigned int PARTITION_ENTRY_LENGTH = 16;
const unsigned int PARTITION_TABLE_LENGTH = PARTITION_ENTRY_COUNT * PARTITION_ENTRY_LENGTH;
const unsigned int BOOT_SIGNATURE_LENGTH = 2;

const char EMPTY_WORD[WORD_SIZE] = {0x00,0x00};
const char BOOT_SIGNATURE[BOOT_SIGNATURE_LENGTH] = {0x55, (char)0xAA};

#endif
