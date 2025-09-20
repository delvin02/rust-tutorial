fn take(s: String) -> String {
    println!("{}", s);
    s
}

fn main() {
    let mut s = String::from("hi");
    s = take(s);
    println!("{}", s);
}
