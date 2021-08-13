use std::collections::HashMap;

fn main() {
    solve(String::from("jjjaalllkxxxklll"));
}

fn solve(s: String) {
    let mut chars: HashMap<char, i32> = HashMap::new();

    for elem in s.chars() {
        if chars.contains_key(&elem) {
            *chars.get_mut(&elem).unwrap() += 1;
        } else {
            chars.insert(elem, 1);
        }
    }

    for (key, value) in &chars {
       if *value == 1 {
          println!("{}", *key);
          break;
       }
    }

    println!("_");

    
}