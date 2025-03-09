/*
 *****************************************************
 *
 * Logging implementations
 *
 *****************************************************
 */

#include "log.hpp"

/*
 *****************************************************
 *
 * Standard Library Include
 *
 *****************************************************
 */

#include <iostream>

/*
 *****************************************************
 *
 * Constants
 *
 *****************************************************
 */

const std::string GREEN_FG  = "\x1b[32m";
const std::string RED_FG    = "\x1b[31m";
const std::string YELLOW_FG = "\x1b[33m";
const std::string RESET     = "\x1b[0m";

Logger::Logger(std::string name, std::ostream &output) : name{name}, output{output} {}

void Logger::log_info(std::string msg) const
{
    output << "[" << GREEN_FG << "INFO" << RESET << "] " << msg << "\n";
}

void Logger::log_warning(std::string msg) const
{
    output << "[" << YELLOW_FG << "WARN" << RESET << "] " << msg << "\n";
}
void Logger::log_error(std::string msg) const
{
    output << "[" << RED_FG << "ERR" << RESET << "]  " << msg << "\n";
}
