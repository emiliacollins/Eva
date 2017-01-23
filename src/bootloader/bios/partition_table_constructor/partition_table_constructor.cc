#include <iostream>
#include <vector>
#include <cstdlib>
#include <fstream>
#include "config_parser.h"
#include "exit_codes.h"
#include "required_params.h"
#include "utilities.h"
#include "partition_table.h"

using namespace std;



void printWelcome() {
  cout << "################################################################################" << endl
       << "#              Welcome to Eric's MBR Partition Table constructor!              #" << endl
       << "################################################################################" << endl
       << endl;
}

string getConfigFilename() {
  string configFile = "";
  while (configFile == "") {
    cout << "Configuration file: ";
    cin >> configFile;
  }
  return configFile;
}

map<string, unsigned long> parseConfigFile(string configFilename) {
  ConfigParser parser;
  cout << "Parsing config file...";
  if (!parser.parse(configFilename)) {
    cout << "FAIL" << endl;
    cout << "Could not parse config file '" << configFilename << "'!" << endl;
    cout << "Exiting..." << endl;
    exit(EXIT_BAD_CONFIG);
  }
  cout << "SUCCESS" << endl;
  
  parser.setParameterList(REQUIRED_PARAMS);

  cout << "Verifying configuration parameters...";
  if (!parser.verifyParams()) {
    cout << "FAIL" << endl;
    cout << "Config file missing param or param misconfigured!" << endl;
    cout << "Exiting..." << endl;
    exit(EXIT_BAD_OR_MISSING_PARAM);
  }
  cout << "SUCCESS" << endl;

  return parser.getParams();
}


string getOutputFilename() {
  string outputFilename = "";
  while(outputFilename == "") {
    cout << "Output file: ";
    cin >> outputFilename;
  }
  return outputFilename;
}

ofstream getOutputHandle(string outputFilename) {
  cout << "Opening '" << outputFilename << "' for writing...";
  ofstream file(outputFilename, ios::out | ios::binary);
  if (!file.is_open()) {
    cout << "FAIL" << endl;
    cout << "Could not open '" << outputFilename << "' for writing!" << endl;
    cout << "Exiting..." << endl;
    exit(EXIT_BAD_OUTPUT);
  }

  cout << "SUCCESS" << endl;
  return file;
}


PartitionTable generatePartitionTable(const map<string, unsigned long>& params, ofstream& out) {
  PartitionTable table;

  cout << "Generating partition table...";
  cout.flush();

  
  // Partition 1
  if (params.at("use_partition_1")) {
    PartitionEntry entry;
    entry.setStatus(params.at("partition_1_status"));
    entry.setType(params.at("partition_1_type"));
    entry.setStart(params.at("partition_1_start"));
    entry.setSectorCount(params.at("partition_1_size"));
    table.setEntry(0, entry);
  }

  // Partition 2
  if (params.at("use_partition_2")) {
    PartitionEntry entry;
    entry.setStatus(params.at("partition_2_status"));
    entry.setType(params.at("partition_2_type"));
    entry.setStart(params.at("partition_2_start"));
    entry.setSectorCount(params.at("partition_2_size"));
    table.setEntry(1, entry);
  }

  // Partition 3
  if (params.at("use_partition_1")) {
    PartitionEntry entry;
    entry.setStatus(params.at("partition_3_status"));
    entry.setType(params.at("partition_3_type"));
    entry.setStart(params.at("partition_3_start"));
    entry.setSectorCount(params.at("partition_3_size"));
    table.setEntry(2, entry);
  }

  // Partition 4
  if (params.at("use_partition_1")) {
    PartitionEntry entry;
    entry.setStatus(params.at("partition_4_status"));
    entry.setType(params.at("partition_4_type"));
    entry.setStart(params.at("partition_4_start"));
    entry.setSectorCount(params.at("partition_4_size"));
    table.setEntry(3, entry);
  }

  cout << "done" << endl;
  
  // Write Partition table
  cout << "\t" << "Writing partition table...";
  cout.flush();

  char* partitionTableBytes = (char*)table.output();
  out.write(partitionTableBytes, PartitionTable::PARTITION_TABLE_LENGTH);
  delete[] partitionTableBytes;

  cout << "done" << endl;
}


void printGoodbye() {
  cout << endl
       << "################################################################################" << endl
       << "#               PartitionTable construction was successful! Exiting...         #" << endl
       << "################################################################################" << endl
       << endl;
  exit(EXIT_SUCCESSFUL);
}

int main() {
  printWelcome();
  
  string configFilename = getConfigFilename();

  map<string, unsigned long> parameters = parseConfigFile(configFilename);

  string outputFilename = getOutputFilename();

  ofstream outFile = getOutputHandle(outputFilename);

  generatePartitionTable(parameters, outFile);

  outFile.close();

  printGoodbye();  
}
