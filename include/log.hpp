#pragma once

/*
 *****************************************************
 *
 * Standard Library Includes
 *
 *****************************************************
 */

#include <ostream>
#include <string>

/*
 *****************************************************
 *
 * Squeef Library Includes
 *
 *****************************************************
 */

#include "column.hpp"

/*
 *****************************************************
 *
 * Struct Type Definitions
 *
 *****************************************************
 */

/* Database Table */
struct Logger {
    std::string   name;
    std::ostream &output;

    Logger(std::string name, std::ostream &output);

    void log_info(std::string) const;
    void log_warning(std::string) const;
    void log_error(std::string) const;
};
