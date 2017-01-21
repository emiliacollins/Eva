#ifndef UTILITIES_H
#define UTILITIES_H

#include "typedefs.h"
#include "chs_address.h"

LBAddress chs2lba(CHSAddress address);
CHSAddress lba2chs(LBAddress address);
unsigned char* convertToByteArray(unsigned long l);

#endif
