pub fn build_proverb(list: &[&str]) -> String {
    let mut to_ret = String::new();

    for i in 0..list.len() {
        if i != list.len() - 1{
            to_ret = format!("{}For want of a {} the {} was lost.\n", to_ret, list[i], list[i+1]);
        }else{
            to_ret = format!("{}And all for the want of a {}.", to_ret, list[0]);
        }
    }

    to_ret
}
