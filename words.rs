pub fn not_case_sensitive(line: &String, raw_word: &String) {
    let word = raw_word.to_lowercase();
    let lowercase_line = line.to_lowercase();
    let words = lowercase_line
        .split(|c: char| !(c.is_alphabetic() || c == '\''))
        .filter(|s| !s.is_empty());
    for term in words {
        if term == word {
            println!("{}", line);
            break;
        }
    }
}

pub fn case_sensitive(line: &String, raw_word: &String) {
    let words = line
        .split(|c: char| !(c.is_alphabetic() || c == '\''))
        .filter(|s| !s.is_empty());
    for term in words {
        if term == raw_word {
            println!("{}", line);
            break;
        }
    }
}
