/***************************************************************************************************
 *                                                                                                 *
 * PartitionEntry: a class for representing a partition in an MBR parition table.                  *
 *                                                                                                 *
 **************************************************************************************************/


#ifndef PARTITION_ENTRY_H
#define PARTITION_ENTRY_H


#include "typedefs.h"
#include "chs_address.h"


class PartitionEntry {
 public:

  /***** Constants *****/
  
  static const unsigned int ENTRY_LENGTH = 16;

  
  /***** Constructors *****/

  // Default constructor
  PartitionEntry();

  // Constructor for LBA partition size specification
  PartitionEntry(PartitionStatus status, PartitionType type, LBAddress start, unsigned long sectorCount);

  // Constructor for CHS partition size specification
  PartitionEntry(PartitionStatus status, CHSAddress start, PartitionType type, CHSAddress end);

  //Copy Constructor
  void operator=(const PartitionEntry& entry);

  
  /***** Accessors *****/
  
  PartitionStatus getStatus() const;
  CHSAddress getStartCHS() const;
  PartitionType getType() const;
  CHSAddress getEndCHS() const;
  LBAddress getStartLBA() const;
  unsigned long getSectorCount() const;

  
  /***** Mutators *****/
  
  void setStatus(PartitionStatus status);
  void setStart(CHSAddress start);
  void setType(PartitionType type);
  void setEnd(CHSAddress end);
  void setStart(LBAddress start);
  void setSectorCount(unsigned long sectorCount);

  
  /***** Utilities *****/

  // Outputs the PartitionEntry object into byte array as it would be represented on disk
  char* output() const;

  
 private:
  /***** Constants *****/

  // Default values
  static const PartitionStatus DEFAULT_STATUS =       0x00;
  static const LBAddress DEFAULT_START        = 0x00000000;
  static const PartitionType DEFAULT_TYPE     =       0x00;
  static const unsigned long DEFAULT_LENGTH   = 0x00000000;

  // Positions and sizes for aligning entry components in output byte array
  static const unsigned int STATUS_INDEX        =  0;
  static const unsigned int STATUS_LENGTH       =  1;
  static const unsigned int CHS_START_INDEX     =  1;
  static const unsigned int TYPE_INDEX          =  4;
  static const unsigned int TYPE_LENGTH         =  1;
  static const unsigned int CHS_END_INDEX       =  5;
  static const unsigned int LBA_START_INDEX     =  8;
  static const unsigned int LBA_START_LENGTH    =  4;
  static const unsigned int SECTOR_COUNT_INDEX  = 12;
  static const unsigned int SECTOR_COUNT_LENGTH =  4;

  
  /***** Internal Utilities *****/

  // Constructor helper
  void init(PartitionStatus status, PartitionType type, LBAddress start, unsigned long numSectors);

  
  /***** Instance Variables *****/
  
  PartitionStatus status;
  LBAddress start;
  PartitionType type;
  unsigned long sectorCount;
};


#endif
