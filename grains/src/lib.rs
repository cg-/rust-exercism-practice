pub fn square(s: u32) -> u64 {
    match s{
        0 => panic!("Square must be between 1 and 64")
        >64 => panic!("Square must be between 1 and 64")
        _ => 2_u64.pow(s-1)
    }
}

pub fn total() -> u64 {
    (1)
}
