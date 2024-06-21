/*
    Implementing Sieve of Eratosthenes

    Create a list of consecutive integers from 2 
    through n: (2, 3, 4, ..., n).

    Initially, let p equal 2, the smallest prime 
    number.

    Enumerate the multiples of p by counting in 
    increments of p from 2p to n, and mark them 
    in the list (these will be 2p, 3p, 4p, ...; 
    the p itself should not be marked).

    Find the smallest number in the list greater 
    than p that is not marked. If there was no 
    such number, stop. Otherwise, let p now equal 
    this new number (which is the next prime), 
    and repeat from step 3.

    When the algorithm terminates, the numbers 
    remaining not marked in the list are all the 
    primes below n.
*/
const MAX_NUMBER: u32 = 500_000;

struct SieveEntry{
    num: u32,
    prime: bool,
}

pub fn nth(n: u32) -> u32 {
    // handle edge case
    if n == 0{
        return 2;
    }

    // initialize sieve
    let mut sieve: Vec<SieveEntry> = Vec::new();
    (0..MAX_NUMBER).for_each(|x|{
        sieve.push(SieveEntry{
            num: x,
            prime: true,
        });
    });

    let mut p = 2;
    let mut running = true;
    while running{
        running = false;
        let mut mult = 2;
        let mut val = p * mult;
        while val < MAX_NUMBER{
            sieve[val as usize].prime = false;
            mult += 1;
            val = p * mult;
        }
        for i in p+1..MAX_NUMBER{
            if sieve[i as usize].prime{
                p = sieve[i as usize].num;
                running = true;
                break
            }
        }
    }

    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);

    (3..MAX_NUMBER).for_each(|x|{
        if sieve[x as usize].prime{
            primes.push(sieve[x as usize].num);
        }
    });

    return primes[n as usize];
}
