/***************************************************************************************************
 *                                                                                                 *
 * CHSAddress: a class for representing a CHS type address.                                        *
 *                                                                                                 *
 **************************************************************************************************/


#include "typedefs.h"
#include "chs_address.h"


/***** Constructors *****/

// Default constructor
CHSAddress::CHSAddress() {
  init(DEFAULT_CYLINDER, DEFAULT_HEAD, DEFAULT_SECTOR);
}

// Construct CHSAddress object with initialized value
CHSAddress::CHSAddress(CylinderNumber cylinder, HeadNumber head, SectorNumber sector) {
  init(cylinder, head, sector);
}

// Constructor helper
void CHSAddress::init(CylinderNumber cylinder, HeadNumber head, SectorNumber sector) {
  setCylinder(cylinder);
  setHead(head);
  setSector(sector);
}


/**** Accessors *****/

CylinderNumber CHSAddress::getCylinder() const {
  return cylinder;
}

HeadNumber CHSAddress::getHead() const {
  return head;
}

SectorNumber CHSAddress::getSector() const {
  return sector;
}


/***** Mutators *****/

void CHSAddress::setCylinder(CylinderNumber cylinder) {
  this -> cylinder = cylinder;
}

void CHSAddress::setHead(HeadNumber head) {
  this -> head = head;
}

void CHSAddress::setSector(SectorNumber sector) {
  this -> sector = sector;
}


/***** Utilities *****/

// Ouptut CHSAddress object into byte array as it would be represented on disk
char* CHSAddress::output() const {
  char* result = new char[CHSADDRESS_SIZE];
  result[H_INDEX] = head;
  result[CHIGH_S_INDEX] = makeCSbyte();
  result[CLOW_INDEX] = makeCbyte();
  return result;
}

// Create byte where high two bits are high bits for cylinder number and lower five bites are the
// sector number
unsigned char CHSAddress::makeCSbyte() const {
  return (cylinder & CHIGH_BITS_MASK) | sector;
}

// Create byte of the lower eight bits of the cylinder number
unsigned char CHSAddress::makeCbyte() const {
  return cylinder & CLOW_BIT_MASK;
}
