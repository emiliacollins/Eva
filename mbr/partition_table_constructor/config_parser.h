#ifndef CONFIG_PARSER_H
#define CONFIG_PARSER_H


#include <string>
#include <map>
#include <vector>


class ConfigParser {
  typedef std::map<std::string, unsigned long> ParamMap;
  typedef std::pair<std::string, unsigned long> ConfigEntry;

 public:
  ConfigParser();
  bool parse(const std::string& filename);
  bool verifyParams() const;
  void setParameterList(const std::vector<std::string>& params);
  const ParamMap& getParams() const;
  ParamMap::const_iterator getIterator() const;
 private:
  static void trim(std::string& s);
  static bool parseValue(std::string value, unsigned long& result);
  static bool parseLine(std::string line, ConfigEntry& entry);

  ParamMap entries;
  std::vector<std::string> parameterList;
};

#endif
