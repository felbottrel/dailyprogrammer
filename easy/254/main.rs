fn atbash(input: &str) -> String {
    let mut cipher = String::new();
    let base = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    for c in input.chars() {
        match base.find(c) {
            Some(i) => cipher.push(base.chars().nth(51-i).unwrap()),
            None => cipher.push(c),
        }
    }    
    cipher
}

fn main() {
    assert_eq!(atbash("fooBAR"), "ULLyzi");
    assert_eq!(atbash(atbash("wizard").as_str()), "wizard");
}