/***************************************************************************************************
 *                                                                                                 *
 * A collection of shared utilities.                                                               *
 *                                                                                                 *
 **************************************************************************************************/


#include "typedefs.h"
#include "chs_address.h" 
#include "utilities.h"


/***** Utilities *****/

// Converts a given CHSAddress to LBA format
LBAddress chs2lba(const CHSAddress& address) {
  // (C * HPC + H) * SPT + (S - 1)
  return (address.getCylinder() * HEADS_PER_CYLINDER + address.getHead()) * MAX_SECTORS_PER_TRACK +
         (address.getSector() - 1);
}


// Converts a given LBAddress to CHS format
CHSAddress lba2chs(const LBAddress address) {
  CHSAddress chsAddr;

  // LBA / (HPC * SPT) 
  chsAddr.setCylinder(address / (HEADS_PER_CYLINDER * MAX_SECTORS_PER_TRACK));

  // (LBA / SPT) % HPC
  chsAddr.setHead((address / MAX_SECTORS_PER_TRACK) % HEADS_PER_CYLINDER);

  // (LBA % SPT) + 1
  chsAddr.setSector((address % MAX_SECTORS_PER_TRACK) + 1);

  return chsAddr;
}


// Converts the first <length> bytes of a given number to an array of bytes (little endian)
char* convertToByteArray(unsigned long long num, unsigned int length) {
  char * result = new char[length];

  for (int i=0; i < length; i++) {
    // Retrieve the lowest 8 bits
    result[i] = num & LOWEST_BYTE_MASK;
    
    // Move to next byte
    num = num >> 8;
  }
  
  return result;
}
