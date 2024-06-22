struct FindFacResults{
    val: u64,
    remain: u64,
    done: bool,
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut to_ret: Vec<u64> = Vec::new();

    let mut cur = n;
    loop{
        let result = find_fac(cur);
        if result.val != 1{
            to_ret.push(result.val);
        }
        if result.done{
            break;
        }
        cur = result.remain;
    }

    to_ret
}

pub fn find_fac(n: u64) -> FindFacResults {
    // find any factors
    for i in 2..n{
        if n % i == 0{
            return FindFacResults{
                val: i,
                remain: n/i,
                done: false,
            }
        }
    }

    // no other factors exist
    return FindFacResults{
        val: n,
        remain: 0,
        done: true,
    }
}