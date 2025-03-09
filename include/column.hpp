#pragma once

/*
 *****************************************************
 *
 * Standard Library Includes
 *
 *****************************************************
 */

#include <string>

/*
 *****************************************************
 *
 * Enum Type Definitions
 *
 *****************************************************
 */

enum Type {
    TYPE_CHAR,
    TYPE_BOOLEAN,
    TYPE_UINT8,
    TYPE_SINT8,
    TYPE_UINT16,
    TYPE_SINT16,
    TYPE_UINT32,
    TYPE_SINT32,
    TYPE_UINT64,
    TYPE_SINT64,
    TYPE_FLOAT32,
    TYPE_FLOAT64,

    // Should STRING be its own type or should it be an array of chars ???
    TYPE_STRING
};

/*
 *****************************************************
 *
 * Struct Type Definitions
 *
 *****************************************************
 */

/* Table Column */
struct Column {
    std::string name;

    Type type;

    // Flags
    bool is_optional;
    bool is_array;
    bool is_primary_key;
    bool is_foreign_key;
};
