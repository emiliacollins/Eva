/***************************************************************************************************
 *                                                                                                 *
 * PartitionTable: a class representing a collection of partition entries in a MBR.                *
 *                                                                                                 *
 **************************************************************************************************/


#ifndef PARTITION_TABLE_H
#define PARTITION_TABLE_H


#include "partition_entry.h"


class PartitionTable {
 public:
  /***** Constants *****/
  
  static const unsigned int NUM_ENTRIES = 4;
  static const unsigned int PARTITION_TABLE_LENGTH = 64;
  
  
  /***** Constructor *****/
  
  // Constructor
  PartitionTable();

  
  /***** Mutators *****/

  // Places given entry at specified position
  void setEntry(unsigned int pos, const PartitionEntry& entry);


  /***** Accessors *****/

  // Retrieves entry at position
  const PartitionEntry& getEntry(unsigned int pos) const;


  /***** Utilities *****/
  
  char* output() const;

  
 private:
  /***** Instance Variables *****/
  
  PartitionEntry entries[NUM_ENTRIES];
};

#endif
