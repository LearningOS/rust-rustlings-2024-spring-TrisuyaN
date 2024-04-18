// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz"); // 编译器得知三者生命周期相同（为最短的string2），从而导致认为result在line26已经似掉了（25行就似了），从而拒绝编译
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
    
}
