fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        // let string3 = String::from("xyz");
        // `string3` does not live long enough
        // result = longest(string1.as_str(), string3.as_str());
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}


fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}