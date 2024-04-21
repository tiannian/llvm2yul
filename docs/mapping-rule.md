# Mapping Rule

This document describes how to compile LLVM IR to Yul.

## Type

LLVM IR have rich type system. But Yul only have one type `u256`. So a mapping rule is needed between them.

Most modern programming languages ​​do not provide integers up to 256 bits, and 64-bit integers in Yul are treated as 256-bit integers. Therefore, u64 can be used instead of u256 in programming languages ​​to avoid complex type merging operations.

For example, use the following type to explain `U256` in Rust:

```rust
#[repr(C)]
#[repr(align(32))]
pub struct U256(u64)
```

### Basic Type

Since Yul only supports u256, all floating-point related types and vector related types are not supported. This means `llvm2yul` only supported these following basic LLVM IR type.

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

However, when the function pointer is used as a parameter of `datacopy` and `dataoffset`, it will be converted to a string literal.

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

Basic literal only have these type. They will be treated as a plain `u256`.

- Integer
- Null
- Undef
- Poison

#### Integer

Integer Constant Value in LLVM IR will direct convert into Yul number. 

But the constant integer in LLVM IR only have `u64` type. To construct `u256`, `__yul_ext_literal(u64, u64, u64, u64)`'s parameters will merge into one `u256`.

`__yul_ext_literal(u64, u64, u64, u64) -> u64` is just a fake function, it will convert a single literal.

Considering that languages ​​like C do not have complex macro support, there are two functions:

- `__yul_ext_literal(u64, u64, u64, u64)`
- `__yul_ext_literal_str(str)`

#### Null, Undef and Posion

Beacuse of the value of Yul will be initiated as `0`, so Null, Undef and Posion constant value are presented as `0`.

### Aggregate Literal

These literal of aggregate are support:

- AggregateZero
- Struct
- Array

These types will be flatten. The flatten rule is same as aggregate type.

### Constant with Instruction

Some constant in LLVM IR is an instruction. These type of constant will convert into single value directly.

### Global Reference

Global Reference will add into data section, then access these value using `datacopy` to load data.

## Instructions

### Function Call

If function is builtin function, all parameters will be treated as basic type. If function is not builtin function, parameters will be flatten.

### Operate Instructions

Only a subset of LLVM IR instructions are supported. They will be represent as Yul's statement. Not all LLVM IR instructions supported, error-handling instructions, float instructions and arithmetic instructions are not supported.

Since arithmetic instructions are not supported, all arithmetic operations need to use builtin functions.

- ExtractValue
- InsertValue
- Alloca
- Load
- Store
- GetElementPtr
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

### Supported function
