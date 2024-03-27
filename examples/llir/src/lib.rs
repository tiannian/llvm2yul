#![no_std]

extern "C" {
    fn test(u: i64);

    fn build_int() -> i64;
}

struct Test {
    a: i64,
    b: i64,
}

impl Test {
    pub fn new(i: i64) -> Self {
        Self {
            a: i,
            b: unsafe { build_int() },
        }
    }
}

#[no_mangle]
pub fn _entry(l: i64) {
    for i in 1..l {
        let t = Test::new(i);

        unsafe {
            test(t.a);
            test(t.b)
        }
    }
}
