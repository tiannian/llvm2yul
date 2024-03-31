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

- [X] 64 / 32 bit with 256 align as primitive type. Primitive type use parameter and return value directly.
- [ ] Map LLVM IR instruction to yul
    1. [X] ExtractValue
    2. [ ] InsertValue
    3. [ ] Alloca
    4. [ ] Load
    5. [ ] Store
    6. [ ] GetElementPtr
    7. [ ] PtrToInt
    8. [ ] IntToPtr
    9. [ ] Phi
    10. [ ] Select
    11. [ ] Call
- [ ] Map LLVM IR Type to yul
    1. [ ] Void
    2. [ ] Integer
    3. [ ] Pointer
    4. [ ] Array
    5. [ ] Struct
    6. [ ] NamedStruct
- [ ] Map LLVM IR Constant to yul
    1. [ ] Int
    2. [ ] Null
    3. [ ] AggregateZero
    4. [ ] Struct
    5. [ ] Array
    6. [ ] GlobalReference
    7. [ ] Poison
- [ ] Flatten LLVM IR Struct or Array
- [ ] Map LLVM IR function call to yul.
    - [ ] Use FFI function to map yul builtin function.
    - [ ] allocate function as builtin function
- [ ] Map block termiantor to control flow.
    - [X] Ret (leave)
    - [ ] Br
    - [ ] CondBr
    - [ ] Switch (switch)
    - [X] Unreachable (invaild)
- [X] Map LLVM IR funtion to yul
    - [X] Primitive parameter type
    - [X] Primitive return type
    - [ ] Nested struct parameter and return support.
- [ ] Make unsupported display clearily

