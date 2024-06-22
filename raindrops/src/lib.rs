/*
- is divisible by 3, add "Pling" to the result.
- is divisible by 5, add "Plang" to the result.
- is divisible by 7, add "Plong" to the result.
- **is not** divisible by 3, 5, or 7, the result should be the number as a string.
*/

pub fn raindrops(n: u32) -> String {
    let mut to_ret = String::new();
    let mut found = false;
    if n % 3 == 0{
        found = true;
        to_ret = format!("{}{}",to_ret, "Pling");
    }
    if n % 5 == 0{
        found = true;
        to_ret = format!("{}{}",to_ret, "Plang");
    }
    if n % 7 == 0{
        found = true;
        to_ret = format!("{}{}",to_ret, "Plong");
    }
    if !found {
        to_ret = format!("{}",n);
    }
    to_ret
}
