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

#include "column.hpp"

/*
 *****************************************************
 *
 * Struct Type Definitions
 *
 *****************************************************
 */

/* Database Table */
struct Table {
    std::string name;

    std::vector<Column> columns;
};
