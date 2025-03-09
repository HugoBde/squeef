#pragma once

/*
 *****************************************************
 *
 * Standard Library Includes
 *
 *****************************************************
 */

#include <string>
#include <vector>

/*
 *****************************************************
 *
 * Squeef Library Includes
 *
 *****************************************************
 */

#include "table.hpp"

/*
 *****************************************************
 *
 * Struct Type Definitions
 *
 *****************************************************
 */

/* Database */
struct Database {
    std::string name;

    std::vector<Table> tables;
};
