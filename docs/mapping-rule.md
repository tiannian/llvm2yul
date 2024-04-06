# Mapping Rule

This document describes how to compile LLVM IR to Yul.

## Type

LLVM IR have rich type system. But Yul only have one type `u256`. So a mapping rule is needed between them.

### Basic Type

For now, only a subset of LLVM IT first class type are supported. Since Yul only supports u256, all floating-point related types and vector related types are not supported. This means `llvm2yul` only supported these following basic LLVM IR type.

- Void
- Integer
- Pointer
- Function

#### Void Type

The Void Type is only allowed to appear in the return value alone. It is not mapped to any Yul variable and is only used to indicate that the function has no return value.

#### Integer Type

Integer types have various lengths, but no matter what the length is, it will be treated as an integer with a length of 256 bits.

#### Pointer Type

There are also many types of pointer types in LLVM IR. However, no matter how complex the pointer type is, it will only be mapped to a `u256`.

#### Function Type

Since Yul does not support function pointers, all function pointers used as parameters and return values ​​will result in errors.

However, when the function pointer is used as a parameter of `datacopy` and `dataoffse`t, it will be converted to a string literal.

### Aggregate Type

LLVM IR has support for complex structures, but Yul does not, so these complex types need to be flattened into a set of basic types.

- Array
- Structure
- Named Structure

#### Structure

Each field in the structure is recursively expanded into a sequence of Yul variables.

For example, we have this structure in LLVM IR:

```llvm
%struct.RT = type { i8, i8 }
%struct.ST = type { i32, i32, %struct.RT, i32 }
```

It will be flatten to:

```
ST1_0, ST_1, RT_0, RT_1, ST_2
```

#### Named Structure Type

Named Structure need follow same rule of structure. But if a `Named Structure Type`'s name is marked by basic type, it will treat as a `u256`.

#### Array

Array also will expand to a sequence of Yul variables. But if the element of array's type is `i8`, it will be treated as bytes or string. This rule will work on literal of array.

## Literal

### Basic Literal

### Aggregate Literal

## Instructions

### Function Call

### Operate Instructions

Only a subset of LLVM IR instructions are supported. They will be represent as Yul's statement. Not all LLVM IR instructions supported, each error-handling instructions, float instructions and arithmetic instructions are not supported.

Since arithmetic instructions are not supported, all arithmetic operations need to use builtin functions.

- ExtractValue
- InsertValue
- Alloca
- Load
- Store
- PtrToInt
- IntToPtr
- Phi
- Select

## Code Structure

### Function Declaration

### Block

### Terminator and Flow Control

## Builtin Function

Builtin function will map to yul EVM dialect function. But them will be add a prefix `__yul_`. For example, `add` function will be `__yul_add`.

These builtin functions will add or remove following the changes of yul compiler.
