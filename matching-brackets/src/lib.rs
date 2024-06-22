pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars(){
        match c {
            '[' | '{' | '(' => {
                stack.push(c);
            },
            ')' => if stack.pop() != Some('('){
                return false
            },
            '}' => if stack.pop() != Some('{'){
                return false
            },
            ']' => if stack.pop() != Some('['){
                return false
            },
            _ => (),
        }
    }

    if stack.len() != 0{
        return false;
    }

    return true;

}
