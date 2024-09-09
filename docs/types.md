# Type System

This document describes the type system used in this project.

## Overview

The type system is a simple system with no implicit type conversions. This is to avoid the ambiguity that can arise from implicit type conversions, and to make the system easier to reason about.

## Types

There are several basic types, and one generic/compound type.

- **`Number`**: A 64-bit floating-point number.
- **`String`**: A UTF-8 encoded string.
- **`Boolean`**: A boolean value.
- **`Column[T]`**: A reference to a column in the workspace holding values of type `T`.

### Number

The `Number` type is a 64-bit floating-point number. It is used to represent numeric values in the system. I chose this type as the vast majority of integers can be represented exactly as floating-point numbers, and it is easier to work with a single numeric type.

### String

The `String` type is a UTF-8 encoded string. It is used to represent text values in the system. I chose this type as it is the most common string encoding, and is widely supported, and doesnt require any special handling for different encodings or wide characters.

### Boolean

The `Boolean` type is a boolean value. It is used to represent truth values in the system. I chose this type as it is the simplest way to represent truth values, and is widely supported.

### Column[T]

The `Column[T]` type is a reference to a column in the workspace holding values of type `T`. This is used to represent columns in the workspace, and is used in expressions to reference the values in the column.

## Typechecking

Typechecking is done at the expression parsing stage. Each expression is checked for type correctness, and the type of the expression is stored in the expression object. This allows for easy typechecking of expressions before evaluating them, so errors can be caught early.
