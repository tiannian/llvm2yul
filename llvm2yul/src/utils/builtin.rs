pub fn builtin_args_num(name: &str) -> Option<usize> {
    if name.len() < 5 {
        return None;
    }

    let (prefix, name) = name.split_at(6);

    if prefix != "__yul_" {
        return None;
    }

    let r = match name {
        "stop" => 0,
        "add" => 2,
        "sub" => 2,
        "mul" => 2,
        "div" => 2,
        "sdiv" => 2,
        "mod" => 2,
        "smod" => 2,
        "exp" => 2,
        "not" => 1,
        "lt" => 2,
        "gt" => 2,
        "slt" => 2,
        "sgt" => 2,
        "eq" => 2,
        "iszero" => 1,
        "and" => 2,
        "or" => 2,
        "xor" => 2,
        "byte" => 2,
        "shl" => 2,
        "shr" => 2,
        "sar" => 2,
        "addmod" => 3,
        "mulmod" => 3,
        "signextend" => 2,
        "keccak256" => 2,
        "pc" => 0,
        "pop" => 2,
        "mload" => 1,
        "mstore" => 2,
        "mstore8" => 2,
        "sload" => 1,
        "sstore" => 2,
        "tload" => 1,
        "tstore" => 2,
        "msize" => 0,
        "gas" => 0,
        "address" => 0,
        "balance" => 1,
        "selfbalance" => 0,
        "caller" => 0,
        "callvalue" => 0,
        "calldataload" => 1,
        "calldatasize" => 0,
        "calldatacopy" => 3,
        "codesize" => 0,
        "codecopy" => 3,
        "extcodesize" => 1,
        "extcodecopy" => 4,
        "returndatasize" => 0,
        "returndatacopy" => 3,
        "mcopy" => 3,
        "extcodehash" => 1,
        "create" => 3,
        "create2" => 4,
        "call" => 7,
        "callcode" => 7,
        "delegatecall" => 6,
        "staticcall" => 6,
        "return" => 2,
        "revert" => 2,
        "selfdestruct" => 1,
        "invalid" => 0,
        "log0" => 2,
        "log1" => 3,
        "log2" => 4,
        "log3" => 5,
        "log4" => 6,
        "chainid" => 0,
        "basefee" => 0,
        "blobbasefee" => 0,
        "origin" => 0,
        "gasprice" => 0,
        "blockhash" => 1,
        "blobhash" => 1,
        "coinbase" => 0,
        "timestamp" => 0,
        "number" => 0,
        "difficulty" => 0,
        "prevrandao" => 0,
        "gaslimit" => 0,
        _ => return None,
    };

    Some(r)
}

pub fn is_builtin(name: &str) -> bool {
    builtin_args_num(name).is_some()
}