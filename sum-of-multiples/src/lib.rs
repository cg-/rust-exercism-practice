pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut vals: Vec<u32> = Vec::new();
    for fac in factors{
        if *fac == 0{
            continue;
        }
        let mut mult = 1;
        while fac * mult < limit{
            vals.push(fac * mult);
            mult += 1;
        }
    }
    vals.sort();
    vals.dedup();
    vals.into_iter().sum()
}
