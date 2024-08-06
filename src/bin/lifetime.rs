fn main() {
    
}

fn invalid_output<'a>(s: &'a String) -> &'a String {
    s
}
fn invalid_output1(s: &String) -> &String {
    s
}
fn invalid_output2() -> &'static str {
    "foo"
}
fn invalid_output3() -> String {
    String::from("foo")
}