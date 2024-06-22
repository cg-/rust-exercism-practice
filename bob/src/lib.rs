/*
- **"Sure."**
  This is his response if you ask him a question, such as "How are you?"
  The convention used for questions is that it ends with a question mark.
- **"Whoa, chill out!"**
  This is his answer if you YELL AT HIM.
  The convention used for yelling is ALL CAPITAL LETTERS.
- **"Calm down, I know what I'm doing!"**
  This is what he says if you yell a question at him.
- **"Fine. Be that way!"**
  This is how he responds to silence.
  The convention used for silence is nothing, or various combinations of whitespace characters.
- **"Whatever."**
  This is what he answers to anything else.
*/

pub fn reply(message: &str) -> &str {
    if is_silence(message){
        return "Fine. Be that way!"
    }

    if is_yelling(message) && is_question(message){
        return "Calm down, I know what I'm doing!";
    }

    if is_question(message){
        return "Sure.";
    }

    if is_yelling(message){
        return "Whoa, chill out!";
    }

    "Whatever."
}

pub fn is_silence(message: &str) -> bool{
    if message.trim() == ""{
        return true;
    }
    return false;
}

pub fn is_question(message: &str) -> bool{
    if message.trim_end().ends_with("?"){
        return true;
    }
    return false;
}

pub fn is_yelling(message: &str) -> bool{
    let mut has_letters = false;
    let mut has_lowercase = false;
    for i in message.chars(){
        if i.is_alphabetic(){
            has_letters = true;
        }
        if i.is_ascii_lowercase(){
            has_lowercase = true;
        }
    }
    if !has_letters || has_lowercase{
        return false;
    }
    return true;
}