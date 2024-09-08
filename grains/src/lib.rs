pub fn square(s: u32) -> u64 {
    if s == 1 {
        return 1;
    }
    else {
        return square(s - 1) * 2;
    }
}

pub fn total() -> u64 {
    let r : u128 = square(64).into();
    return (r * 2 - 1).try_into().unwrap();
}
