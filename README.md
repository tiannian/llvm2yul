# LLVM IR to Yul

> Convertor from LLVM to Yul

## Passes Design

1. 

## Features and TODOs

- [X] 64 / 32 bit as primitive type. Primitive type use parameter and return value directly.
- [ ] Map LLVM IR instruction to yul
    - [ ] Alloca
    - [ ] Load
    - [ ] Store
    - [ ] Phi
    - [ ] Call
- [ ] Map LLVM IR function call to yul.
    - [ ] Use FFI function to map yul builtin function.
    - [ ] allocate function as builtin function
- [ ] Map block termiantor to control flow.
    - [X] Ret to leave
    - [ ] Br
    - [ ] CondBr
    - [ ] Switch
    - [X] Unreachable to invaild
- [X] Map LLVM IR funtion to yul
    - [X] Primitive parameter type
    - [X] Primitive return type
    - [ ] Add struct parameter and return support.
- [ ] Make unsupported display clearily

