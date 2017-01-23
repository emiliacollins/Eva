#include <fstream>
#include <sstream>
#include <map>
#include "config_parser.h"


ConfigParser::ConfigParser() {}

bool ConfigParser::parse(const std::string& filename) {
  
  std::ifstream inputFile(filename);
  if (!inputFile.is_open())
    return false;
  
  std::string line;
  while(getline(inputFile, line)) {
    ConfigEntry e;
    if (parseLine(line, e)) {
      entries[e.first] = e.second;
    }
  }

  return true;
}


bool ConfigParser::verifyParams() const {
  for (std::vector<std::string>::const_iterator it=parameterList.begin(); it != parameterList.end(); it++) {
    if (entries.find(*it) == entries.end()) {
      return false;
    }
  }
  return true;
}


void ConfigParser::setParameterList(const std::vector<std::string>& params) {
  parameterList = params;
}

const ConfigParser::ParamMap& ConfigParser::getParams() const  {
  return entries;
}


void ConfigParser::trim(std::string& s) {

  s = s.substr(0, s.find("#"));

  for (int i=0; i < s.length() && s[i] == ' '; i++) {
    s = s.substr(1);
  }

  for (int i=s.length() - 1; i >= 0 && s[i] == ' '; i--) {
    s = s.substr(0, i);
  }
}


bool ConfigParser::parseValue(std::string value, unsigned long& result) {
  std::stringstream ss;

  {
    unsigned long hexDenoterIndex = value.find("x");
  
    if (hexDenoterIndex != std::string::npos) {
      value = value.substr(hexDenoterIndex+1);
      ss << std::hex;
    }
  }

  ss << value;
  
  return (bool)(ss >> result);
}


bool ConfigParser::parseLine(std::string line, ConfigEntry& entry) {
 
  trim(line);

  
  unsigned int target = line.find('=');

  
  if (target == std::string::npos || target == line.length()-1) {
    return false;
  }
  entry.first = line.substr(0, target);
  

  return parseValue(line.substr(target+1), entry.second);
}




