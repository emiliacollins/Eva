/***************************************************************************************************
 *                                                                                                 *
 * PartitionEntry: a class for representing a partition in an MBR parition table.                  *
 *                                                                                                 *
 **************************************************************************************************/

#include <cassert>
#include "typedefs.h"
#include "utilities.h"
#include "partition_entry.h"

/***** Constructors *****/

// Default constructor
PartitionEntry::PartitionEntry() {
  init(DEFAULT_STATUS, DEFAULT_TYPE, DEFAULT_START, DEFAULT_LENGTH);
}

// Constructor for LBA partition size specification
PartitionEntry::PartitionEntry(PartitionStatus status, PartitionType type, LBAddress start, unsigned long sectorCount) {
  init(status, type, start, sectorCount);
}

// Constructor for CHS partition size specification
PartitionEntry::PartitionEntry(PartitionStatus status, CHSAddress start, PartitionType type, CHSAddress end) {
  init(status, type, chs2lba(start), chs2lba(end) - chs2lba(start) + 1);
}

// Constructor helper
void PartitionEntry::init(PartitionStatus status, PartitionType type, LBAddress start, unsigned long sectorCount) {
  setStatus(status);
  setType(type);
  setStart(start);
  setSectorCount(sectorCount);
}

// Copy Constructor
void PartitionEntry::operator=(const PartitionEntry& entry) {
  setStatus(entry.getStatus());
  setStart(entry.getStartLBA());
  setType(entry.getType());
  setSectorCount(entry.getSectorCount());
}



/***** Accessors *****/

PartitionStatus PartitionEntry::getStatus() const {
  return status;
}

CHSAddress PartitionEntry::getStartCHS() const {
  return lba2chs(start);
}

PartitionType PartitionEntry::getType() const {
  return type;
}

CHSAddress PartitionEntry::getEndCHS() const {
  return lba2chs(start + sectorCount - 1);
}

LBAddress PartitionEntry::getStartLBA() const {
  return start;
}

unsigned long PartitionEntry::getSectorCount() const {
  return sectorCount;
}


/***** Mutators *****/
void PartitionEntry::setStatus(PartitionStatus status) {
  this -> status = status;
}

void PartitionEntry::setStart(CHSAddress start) {
  setStart(chs2lba(start));
  
}

void PartitionEntry::setType(PartitionType type) {
  this -> type = type;
}

void PartitionEntry::setEnd(CHSAddress end) {
  LBAddress endLBA = chs2lba(end);

  // Ensure end address is valid
  assert(endLBA > start);
  
  setSectorCount(endLBA - start + 1);
}

void PartitionEntry::setStart(LBAddress start) {
  this -> start = start;
}

void PartitionEntry::setSectorCount(unsigned long sectorCount) {
  this -> sectorCount = sectorCount;
}


/***** Utilities *****/

// Outputs the PartitionEntry object into byte array as it would be represented on disk
unsigned char* PartitionEntry::output() const {
  unsigned char* result = new unsigned char[ENTRY_LENGTH];

  // Output partition status
  result[STATUS_INDEX] = type;

  // Output CHS starting address
  {
    unsigned char* chsStart = lba2chs(start).output();
    for (unsigned int i= 0; i < CHSAddress::CHSADDRESS_SIZE; i++) {
      result[CHS_START_INDEX + i] = chsStart[i]; 
    }
    delete[] chsStart;
  }

  // Output partition type
  result[TYPE_INDEX] = type;

  // Output CHS ending address
  {
    unsigned char* chsEnd = lba2chs(start + sectorCount - 1).output();
    for (unsigned int i= 0; i < CHSAddress::CHSADDRESS_SIZE; i++) {
      result[CHS_END_INDEX + i] = chsEnd[i]; 
    }
    delete[] chsEnd;
  }

  // Output LBA starting address
  {
    unsigned char* startArray = convertToByteArray(start, LBA_START_LENGTH); 
    for (int i=0; i < LBA_START_LENGTH; i++) {
      result[LBA_START_INDEX + i] = startArray[i];
    }
    delete[] startArray;
  }

  // Output sector count
  {
    unsigned char* sectorCountArray = convertToByteArray(sectorCount, SECTOR_COUNT_LENGTH); 
    for (unsigned int i=0; i < SECTOR_COUNT_LENGTH; i++) {
      result[SECTOR_COUNT_INDEX + i] = sectorCountArray[i];
    }
    delete[] sectorCountArray;
  }
  
  return result;
}
