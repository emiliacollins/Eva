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
  /***** Constructors & Destructors *****/
  
  // Constructor
  PartitionTable();

  // Destructor
  ~PartitionTable();

  
  /***** Mutators *****/

  // Places given entry at specified position
  void setEntry(unsigned int pos, PartitionEntry* entry);


  /***** Accessors *****/

  // Retrieves entry at position
  PartitionEntry* getEntry(unsigned int pos) const;


  /***** Utilities *****/
  
  unsigned char* output() const;

  
 private:
  /***** Constants *****/
  
  static const unsigned int NUM_ENTRIES = 4;


  /***** Instance Variables *****/
  
  PartitionEntry* entries[NUM_ENTRIES];
};

#endif
