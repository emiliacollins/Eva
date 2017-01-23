/***************************************************************************************************
 *                                                                                                 *
 * CHSAddress: a class for representing a CHS type address.                                        *
 *                                                                                                 *
 **************************************************************************************************/


#ifndef CHS_ADDRESS_H
#define CHS_ADDRESS_H


#include "typedefs.h"


class CHSAddress {
 public:
  /***** Constants *****/

  // Number of bytes CHS address takes up on disk
  static const unsigned int CHSADDRESS_SIZE = 3; 
  
  /***** Constructors *****/

  // Default Constructor
  CHSAddress();

  // Initializing Constructor
  CHSAddress(CylinderNumber cylinder, HeadNumber head, SectorNumber sector);

  
  /***** Accessors *****/

  CylinderNumber getCylinder() const;
  HeadNumber getHead() const;
  SectorNumber getSector() const;

  
  /***** Mutators *****/

  void setCylinder(CylinderNumber cylinder);
  void setHead(HeadNumber head);
  void setSector(SectorNumber sector);

  
  /***** Utilities *****/

  // Outputs the CHSAddress object into byte array as it would be represented on disk
  char* output() const;

  
 private:
  /***** Constants *****/

  // Default values
  static const CylinderNumber DEFAULT_CYLINDER = 0x000;
  static const HeadNumber DEFAULT_HEAD         =  0x00;
  static const SectorNumber DEFAULT_SECTOR     =  0x00;
  
  // Location of bytes in disk representation
  static const unsigned int H_INDEX       = 0;
  static const unsigned int CHIGH_S_INDEX = 1;
  static const unsigned int CLOW_INDEX    = 2;

  // Bit masks for creating disk representation
  static const unsigned short CHIGH_BITS_MASK = 0x300;
  static const unsigned char CLOW_BIT_MASK   = 0x0FF;

  
  /***** Internal Utilities *****/

  // Constructor helper
  void init(CylinderNumber cylinder, HeadNumber head, SectorNumber sector);

  // Make bytes for disk representation
  unsigned char makeCSbyte() const;
  unsigned char makeCbyte() const;

  
  /***** Instance Variables *****/

  CylinderNumber cylinder;
  HeadNumber head;
  SectorNumber sector;
};


#endif
