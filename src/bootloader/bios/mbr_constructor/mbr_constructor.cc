#include <iostream>
#include <vector>
#include <cstdlib>
#include <fstream>
#include "config_parser.h"
#include "exit_codes.h"
#include "required_params.h"
#include "mbr_layout.h"

using namespace std;



void printWelcome() {
  cout << "################################################################################" << endl
       << "#                    Welcome to Eric's MBR constructor!                        #" << endl
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
  ofstream file(outputFilename, ios_base::binary);
  if (!file.is_open()) {
    cout << "FAIL" << endl;
    cout << "Could not open '" << outputFilename << "' for writing!" << endl;
    cout << "Exiting..." << endl;
    exit(EXIT_BAD_OUTPUT);
  }

  cout << "SUCCESS" << endl;
  return file;
}

void generateMBR(const map<string, unsigned long>& params, ofstream& out) {
  for (int i=0; i < BOOTCODE_1_LENGTH; i++)
    out << char(0) << endl;
  
}

void printGoodbye() {
  cout << "################################################################################" << endl
       << "#                   MBR construction was successful! Exiting...                #" << endl
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

  generateMBR(parameters, outFile);

  printGoodbye();  
}
