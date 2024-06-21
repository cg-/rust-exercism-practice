pub fn square_of_sum(n: u32) -> u32 {
    let mut res = 0 as u32;
    for i in 1..=n {
        res += i;
    }
    res.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut res = 0 as u32;
    for i in 1..=n {
        res += (i.pow(2)) as u32;
    }
    res
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
