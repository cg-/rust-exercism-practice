/*
Take any positive integer n.
If n is even, divide n by 2 to get n / 2.
If n is odd, multiply n by 3 and add 1 to get 3n + 1.
Repeat the process indefinitely.
The conjecture states that no matter which number 
you start with, you will always reach 1 eventually.

Given a number n, return the number of steps required 
to reach 1.
*/

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0{
        return None;
    }
    let mut val = n;
    let mut steps = 0;
    while val != 1{
        // even
        if val % 2 == 0{
            steps += 1;
            val /= 2;
        // odd
        }else{
            steps += 1;
            val *= 3;
            val += 1;
        }
    }
    return Some(steps)
}
