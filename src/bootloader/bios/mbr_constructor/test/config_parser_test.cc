#include "../config_parser.h"
#include <iostream>
#include <map>

using namespace std;

int main() {
  ConfigParser parser;
  parser.parse("example.conf");
  const map<string, unsigned long> params = parser.getParams();

  vector<string> paramList;
  paramList.push_back("boot_signature");
  paramList.push_back("disk_signature");
  paramList.push_back("drive_number");
  paramList.push_back("partition_1_size");
  paramList.push_back("partition_1_start");
  paramList.push_back("partition_1_status");
  paramList.push_back("partition_1_type");
  paramList.push_back("partition_2_size");
  paramList.push_back("partition_2_start");
  paramList.push_back("partition_2_status");
  paramList.push_back("partition_2_type");
  paramList.push_back("partition_3_size");
  paramList.push_back("partition_3_start");
  paramList.push_back("partition_3_status");
  paramList.push_back("partition_3_type");
  paramList.push_back("partition_4_size");
  paramList.push_back("partition_4_start");
  paramList.push_back("partition_4_status");
  paramList.push_back("partition_4_type");
  paramList.push_back("timestamp_hours");
  paramList.push_back("timestamp_minutes");
  paramList.push_back("timestamp_seconds");
  paramList.push_back("use_partition_1");
  paramList.push_back("use_partition_2");
  paramList.push_back("use_partition_3");
  paramList.push_back("use_partition_4");


  
  for (auto it=params.begin(); it != params.end(); it++) {
    cout << it -> first << "=" << it -> second << endl;
  }


  parser.setParameterList(paramList);
  if (parser.verifyParams())
    cout << "verified" << endl;
  else
    cout << "failed" << endl;

}
