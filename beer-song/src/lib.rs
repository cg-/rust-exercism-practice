pub fn verse(n: u32) -> String {
    match n{
        0 => {
            String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
        },
        1 => {
            String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
        },
        2 => {
            String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
        },
        _ => {
            format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
        },
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut to_ret = String::new();
    let mut count = start;
    while count >= end{
        to_ret.push_str(&verse(count));
        if count != end{
            to_ret.push_str(&String::from("\n"));
        }
        if count == 0{
            break;
        }
        count -= 1;
    }
    to_ret
}
