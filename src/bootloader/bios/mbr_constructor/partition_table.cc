/***************************************************************************************************
 *                                                                                                 *
 * PartitionTable: a class representing a collection of partition entries in a MBR.                *
 *                                                                                                 *
 **************************************************************************************************/


#include "partition_entry.h"
#include "partition_table.h"


/***** Constructor & Destructor *****/

// Default constructor, populates table with invalid entries
PartitionTable::PartitionTable() {
  for (int i=0; i < NUM_ENTRIES; i++) {
    entries[i] = new PartitionEntry();
    entries[i] -> setStatus(0x01);
  }
}

// Destructor, destroys each non-null pointer in entries
PartitionTable::~PartitionTable() {
  for (unsigned int i=0; i < NUM_ENTRIES; i++) {
    if (entries[i] != nullptr) {
      delete entries[i];
    }
  }
}


/***** Mutators *****/

void PartitionTable::setEntry(unsigned int pos, PartitionEntry* entry) {
  entries[pos] = entry;
}


/***** Accessors *****/

PartitionEntry* PartitionTable::getEntry(unsigned int pos) const {
  return entries[pos];
}


/***** Utilities *****/

unsigned char* PartitionTable::output() const {
  unsigned char* result = new unsigned char[NUM_ENTRIES * PartitionEntry::ENTRY_LENGTH];

  // Write each entry into result array
  for (int i=0; i < NUM_ENTRIES; i++) {
      unsigned char* entry = entries[i] -> output();
      for (int j=0; j < PartitionEntry::ENTRY_LENGTH; j++) {
	result[i * PartitionEntry::ENTRY_LENGTH + j] = entry[j];
      }
      delete[] entry;
    }
  
  return result;
}