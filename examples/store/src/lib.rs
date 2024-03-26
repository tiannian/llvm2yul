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

#[inline]
fn u256(i: u32) -> u32 {
    unsafe { __yul__ext_literal(0, 0, 0, 0, 0, 0, 0, i) }
}

#[no_mangle]
pub extern "C" fn _entry() {
    let sender = unsafe { __yul_caller() };
    let idx = u256(0);
    unsafe { __yul_sstore(idx, sender) };

    // deploy contract
    let codeoffset = unsafe { __yul_dataoffset(_deployed_entry) };
    let codesize = unsafe { __yul_datasize(_deployed_entry) };
    let idx0 = u256(0);
    unsafe { __yul_datacopy(idx0, codeoffset, codesize) };

    unsafe { __yul_return(idx0, codesize) };
}

#[no_mangle]
pub extern "C" fn _deployed_entry() {
    let v0 = u256(0);

    unsafe { __yul_revert(v0, v0) }
}
