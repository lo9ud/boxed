# Expressions

This document describes the expression evaluation system used in this project.

## Grammar

The grammar for expressions is as follows:

```ebnf

start
    = "=" or_expr;

or_expr
    = and_expr { "|" and_expr };

and_expr
    = not_expr { "&" not_expr };

not_expr
    = "!" not_expr
    | cmp_expr;

cmp_expr
    = add_expr { ( "=="
    | "!="
    | ">="
    | "<="
    | "<"
    | ">" ) add_expr };

add_expr
    = mul_expr { ( "+"
    | "-" ) mul_expr };

mul_expr
    = pow_expr { ( "*"
    | "/"
    | "%" ) pow_expr };

pow_expr
    = unary_expr { ("^") unary_expr };

unary_expr
    = { "-" } primary_expr;

primary_expr
    = number
    | string
    | boolean
    | variable
    | function
    | "(" or_expr ")";

function
    = identifier arg_list;

variable
    = identifier
    | col_spec;

col_spec
    = ":" ( number
    | string ) [ colfilter ];

colfilter
    = ( "rand"
    | "min"
    | "max" ) [ "(" number ")" ];

arg_list
    = "(" [ or_expr { "," or_expr } ] ")";

number =
    [ "-" ] digit { digit } [ ("e" | "E") [ "-" ] digit { digit } ];

string =
    "'" { letter } "'"
    | "\"" { letter } "\"";

boolean =
    "true"
    | "false";

identifier =
    letter { letter | digit | "_" };

letter =
    "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z";

digit =
    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
```

## Parsing

A recursive descent parser is used to parse expressions. This is a simple and efficient method for parsing expressions, and allows for easy typechecking during the parsing stage.

Addtionally, the hierarchial nature of the parser lends itself to the tree structure of the resulting AST and Expression objects.

## Evaluation

Evaluation of expressions is done using a simple recursive, bottom-up strategy. This is simple to implement and understand, and is sufficient for the simple expressions used in this project.

The evaluation system is capable of evaluating arithmetic expressions, functions, and column references.

## Type Checking

Type checking is done during the parsing stage. This is done by checking the types of the operands of each operator, and ensuring that they are compatible.
