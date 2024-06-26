# LLVM IR to Yul

> Convertor from LLVM to Yul

## Supported Language

If you want to build using LLVM frontend, please use corresponding language standard library.

- Rust: [patine](https://github.com/tiannian/patine)

## Use `yullc`

`yullc` is a standalone compiler from llvm-ir to EVM Bytecode.

## Build

### Install llvm

Many platfrom can't get prebuilt binary library for LLVM.
So we suggested compile LLVM manually. Please refer [llvm-sys](https://crates.io/crates/llvm-sys#compiling-llvm)
documentation to build it.

### Compile

```bash
export LLVM_SYS_170_PREFIX=<path-to-llvm-directory>
cargo build
```

## Usage and Test

> Use rust example. Follow these instructions to build testable LLVM IR file.

1. Clone patine repo

```bash
git clone https://github.com/tiannian/patine
```

2. Build store example

```bash
RUSTFLAGS="--emit=llvm-ir" cargo build --release --example=store
```

3. Link llvm

```bash
llvm-link target/release/deps/patine_core-*.ll \
          target/release/deps/patine_std-*.ll \
          target/release/examples/store-*.ll \
          -o store.dc
llvm-dis store.dc
```

4. Copy `store.dc.ll` to `llvm2yul` directory.

5. Compile this repo.

```bash
export LLVM_SYS_170_PREFIX=<path-to-llvm-directory>
RUST_LOG=llvm2yul=debug cargo r --example=compiler
```

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
- Compile call instruction. struct will be flatten
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
    3. [X] Alloca
    4. [ ] Load
    5. [ ] Store
    6. [ ] GetElementPtr
    7. [X] PtrToInt
    8. [X] IntToPtr
    9. [ ] Phi
    10. [X] Select
    11. [X] Call
- [X] Map LLVM IR Type to yul
    1. [X] Void
    2. [X] Integer
    3. [X] Pointer
    4. [X] Array
    5. [X] Struct
    6. [X] NamedStruct
- [ ] Map LLVM IR Constant to yul
    1. [X] Int
    2. [X] Null
    3. [X] AggregateZero
    4. [X] Struct
    5. [X] Array
    6. [X] GlobalReference
    7. [X] Poison
    8. [X] Ptr2Int
    9. [X] Int2Ptr
- [X] Flatten LLVM IR Struct or Array
- [ ] Map LLVM IR function call to yul.
    - [X] Use FFI function to map yul builtin function.
    - [X] allocate function as builtin function
- [ ] Map block termiantor to control flow.
    - [ ] Ret (leave)
    - [ ] Br
    - [ ] CondBr
    - [ ] Switch (switch)
    - [ ] Unreachable (invaild)
- [X] Map LLVM IR funtion to yul
    - [X] Primitive parameter type
    - [X] Primitive return type
    - [X] Nested struct parameter and return support.
- [ ] Make unsupported display clearily

