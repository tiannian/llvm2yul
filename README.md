# LLVM IR to Yul

> Convertor from LLVM to Yul

## Passes Design

### Compile Object

1. Iterate object's all function.
2. Compile function
    1. Compile Header
    2. Compile Block.
    3. Compile Termiantor
    4. Compile Body

### Compile Function Header

In this step, llvm2yul will flat struct.

### Compile Block

- Compile phi instruction
- Compile call instruction. struct will be flatted
- Compile load
- Compile store
- Compile select

### Compile Termiantor

- Compile Ret
- Compile Br
- Compile CondBr
- Compile Switch
- Compile Unreachable

### Compile builtin call

Replace function call into builtin

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

