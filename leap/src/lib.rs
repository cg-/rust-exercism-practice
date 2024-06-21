/*
- In every year that is evenly divisible by 4.
- Unless the year is evenly divisible by 100, 
in which case it's only a leap year if the year 
is also evenly divisible by 400.
*/
pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0{
        if year % 400 == 0{
            return true;
        }
        return false;
    }else if year % 4 == 0{
        return true;
    }
    return false;
}
