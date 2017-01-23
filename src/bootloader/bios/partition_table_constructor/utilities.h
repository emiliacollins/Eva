/***************************************************************************************************
 *                                                                                                 *
 * A collection of shared utilities.                                                               *
 *                                                                                                 *
 **************************************************************************************************/


#ifndef UTILITIES_H
#define UTILITIES_H


#include "typedefs.h"
#include "chs_address.h"


/***** Constants *****/

const unsigned int HEADS_PER_CYLINDER = 16;
const unsigned int MAX_SECTORS_PER_TRACK = 64;
const unsigned int LOWEST_BYTE_MASK = 0xFF;


/***** Utilities *****/

// Converts a given CHSAddress to LBA format
LBAddress chs2lba(const CHSAddress& address);

// Converts a given LBAddress to CHS format
CHSAddress lba2chs(LBAddress address);

// Converts the first <length> bytes of a given number to an array of bytes
char* convertToByteArray(unsigned long long num, unsigned int length);


#endif
