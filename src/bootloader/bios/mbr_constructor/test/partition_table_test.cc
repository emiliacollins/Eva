#include "../partition_table.h"
#include "../partition_entry.h"
#include <iostream>

using namespace std;

int main() {
  PartitionTable table;
  
  PartitionEntry entry1;

  entry1.setStatus(0x80);
  entry1.setType(0x90);
  entry1.setStart(0x7C00);
  entry1.setSectorCount(1024);

  cout << entry1.getStatus() << endl;
  cout << entry1.getType() << endl;
  cout << entry1.getStartLBA() << endl;
  cout << entry1.getSectorCount() << endl;

  table.setEntry(0, entry1);

  PartitionEntry entry2 = table.getEntry(0);

  cout << entry2.getStatus() << endl;
  cout << entry2.getType() << endl;
  cout << entry2.getStartLBA() << endl;
  cout << entry2.getSectorCount() << endl;

  entry2.setStart(entry2.getStartCHS());
  entry2.setEnd(entry2.getEndCHS());

  cout << entry2.getStatus() << endl;
  cout << entry2.getType() << endl;
  cout << entry2.getStartLBA() << endl;
  cout << entry2.getSectorCount() << endl;

}
