fn add_world(foo: &str) -> String {
    let mut bar = foo.to_string();
    bar.push_str(" world");
    return bar;
}

fn main() {
    let string = add_world("Hello");
    println!("{}", string);
}

#[test]
fn test_should_add_world() {
    assert_eq!("Hello world", add_world("Hello"));
}
