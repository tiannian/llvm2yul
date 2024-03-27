#![no_std]

extern "C" {
    pub fn __yul__ext_literal(
        x0: u32,
        x1: u32,
        x2: u32,
        x3: u32,
        x4: u32,
        x5: u32,
        x6: u32,
        x7: u32,
    ) -> u32;

    pub fn __yul_sload(p: u32) -> u32;
    pub fn __yul_sstore(p: u32, v: u32);

    pub fn __yul_caller() -> u32;
    pub fn __yul_calldataload(p: u32) -> u32;
    pub fn __yul_calldatasize() -> u32;

    pub fn __yul_datacopy(t: u32, f: u32, l: u32);
    pub fn __yul_datasize(p: extern "C" fn()) -> u32;
    pub fn __yul_dataoffset(p: extern "C" fn()) -> u32;

    pub fn __yul_return(p: u32, l: u32);

    pub fn __yul_revert(p: u32, o: u32);
}

// fn allocate(size: u32) -> u32 {
//     let stack_base = u256(0x40);
//
//     size
// }

#[inline]
fn u256(i: u32) -> u32 {
    unsafe { __yul__ext_literal(0, 0, 0, 0, 0, 0, 0, i) }
}

pub fn hello(a: [u32; 20]) {
    unsafe { __yul_sload(a[0]) };
}

#[inline(never)]
fn deploy_contract(p: extern "C" fn(), index: u32) -> (u32, u32) {
    let codeoffset = unsafe { __yul_dataoffset(p) };
    let codesize = unsafe { __yul_datasize(p) };

    unsafe { __yul_datacopy(index, codeoffset, codesize) };

    (index, codesize)
}

#[no_mangle]
pub extern "C" fn _entry() {
    let sender = unsafe { __yul_caller() };
    let idx = u256(1);

    unsafe { __yul_sstore(idx, sender) };

    // deploy contract
    let (idx0, codesize) = deploy_contract(_deployed_entry, idx);

    unsafe { __yul_return(idx0, codesize) };
}

#[no_mangle]
pub extern "C" fn _deployed_entry() {
    let v0 = u256(0);

    unsafe { __yul_revert(v0, v0) }
}
