pub fn reverse(input: &str) -> String {
    let mut to_ret = "".to_string();
    for a in input.chars().rev(){
        to_ret.push(a);
    }
    to_ret
}
