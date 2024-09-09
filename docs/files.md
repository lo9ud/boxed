# File Format

This document describes the file format used by the application to save and load workspaces.

## Overview

The file format is a custom binary format based (very loosely) on the Parquet format. The format is designed to be simple and easy to read and write, while still being efficient and compact.

I wanted to challeneg myself to design a reasonably efficient binary format, that could support random-access in near-linear time, while still being simple to implement and understand.

By having some obvious metadata at the start of the file, much of the ui can start loading up well before the actual column values are read, and the column values can be read in parallel, as they are stored in a column-major format.

This allows for a more responsive ui, as the user can start interacting with the workspace before the entire file is loaded, and only the cells/columns in view need to be read/evaluated to be displayed.

# Format

The file format is a simple binary format. The file is divided into three sections: the header, the data and a footer.

## Header

The header is composed of metadata about the columns, the workspace and some general file metadata.

### Columns

The columns section is a list of column names, types and file offsets. The file offset describes the starting position of the column data in the file. This allows for easy indexing into the file to read the data for a specific column.

(TBC) Parity per column?

### Workspace

The workspace section is a list of key-value pairs. Each key-value pair is represented by a key and a value. The key is a string, and the value is a string, a number, a boolean, a reference to a column or an expression. This is also where the expresssions for expression-based columns are stored.

(TBC) Store binary expression object, or string representation?

### File Metadata

The file metadata section is a list of metadata about the file. This includes the file format version, the application version, the date the file was created, and any other relevant information.

## Data

The data section is a list of columns. Each column is represented by a list of values. The values are stored in a column-major format, meaning that all the values for a single column are stored together, allowing for easy indexing inot each column from just an starting offset and a length.

After the columns comes a string table. The string table is a list of strings that are referenced by the columns and the workspace. This allows for easy storage of strings without having to store them multiple times in the file, and allows for arbitrary-length strings to be stored.

This was in an effort to reduce the amount of data that needs to be stored in the file, and to make the file more compact, as most strings tend to be repeated many times in a typical workspace.

(TBC) Compression?
(TBC) Error-detection/correction? (Hamming/parity bits?)

## Footer

The footer is a simple checksum of the file. This is used to verify the integrity of the file when loading it. The checksum is calculated over the entire file, excluding the checksum itself.

# Example

(TBC) Not confirmed yet

```
+----------------------------------------------+
| Header (version, date etc.)                  |
| Columns                                      |
| +-------------------------+                  |
| | 1. (name, type, offset) |                  |
| | 2. (name, type, offset) |                  |
| | ...                     |                  |
| +-------------------------+                  |
| Workspace                                    |
| +---------------------------------------+    |
| | 1. (key, key_type, value, value_type) |    |
| | 2. (key, key_type, value, value_type) |    |
| | ...                                   |    |
| | a. (col, col_type, expr, expr_type)   |    |
| +---------------------------------------+    |
+----------------------------------------------+
| Data                                         |
| +-------------------+                        |
| | Column 1 (values) |                        |
| | Column 2 (values) |                        |
| | ...               |                        |
| +-------------------+                        |
| String Table                                 |
| +-------------------------+                  |
| | String 1 (length, data) |                  |
| | String 2 (length, data) |                  |
| | ...                     |                  |
| +-------------------------+                  |
+----------------------------------------------+
| Footer (checksum)                            |
+----------------------------------------------+
```
