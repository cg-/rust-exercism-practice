pub fn is_armstrong_number(num: u32) -> bool {
    let str_rep = num.to_string();
    let mut sum = 0 as u64;

    for i in str_rep.chars(){
        let cur_num = i.to_digit(10).expect("trouble converting from char to u32") as u64;
        let exponent = str_rep.len() as u32;
        sum += (cur_num).pow(exponent);
    }

    num as u64 == sum
}
