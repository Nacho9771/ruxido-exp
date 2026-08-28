fn main() {
    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    for number in (1..11).rev() {
        println!("{number}!");
    };
    println!("LIFTOFF!!!");

    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}