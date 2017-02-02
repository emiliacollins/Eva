/***************************************************************************************************
 *                                                                                                 *
 * PartitionTable: a class representing a collection of partition entries in a MBR.                *
 *                                                                                                 *
 **************************************************************************************************/


#include "partition_entry.h"
#include "partition_table.h"


/***** Constructor *****/

// Default constructor, populates table with invalid entries
PartitionTable::PartitionTable() {
  for (int i=0; i < NUM_ENTRIES; i++) {
    entries[i] = PartitionEntry();
    entries[i].setStatus(0x01);
  }
}


/***** Mutators *****/

void PartitionTable::setEntry(unsigned int pos, const PartitionEntry& entry) {
  entries[pos] = entry;
}


/***** Accessors *****/

const PartitionEntry& PartitionTable::getEntry(unsigned int pos) const {
  return entries[pos];
}


/***** Utilities *****/

char* PartitionTable::output() const {
  char* result = new char[NUM_ENTRIES * PartitionEntry::ENTRY_LENGTH];
  // Write each entry into result array
  for (int i=0; i < NUM_ENTRIES; i++) {
      char* entry = entries[i].output();
      for (int j=0; j < PartitionEntry::ENTRY_LENGTH; j++) {
	result[i * PartitionEntry::ENTRY_LENGTH + j] = entry[j];
      }
      delete[] entry;
    }
  
  return result;
}
