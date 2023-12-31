//Program to concatenate two strings

fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from(s1);
    result.push_str(s2);
    result
}


fn main() {
    let string1 = "Hello, ";
    let string2 = "World!";

    let concatenate_strings = concatenate_strings(string1, string2);
    println!("{}", concatenate_strings);
}
