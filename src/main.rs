fn add_world(foo: &str) -> String {
    let mut bar = foo.to_string();
    bar.push_str(" world");
    return bar;
}

#[test]
fn test_should_add_world() {
    assert_eq!("Hello world", add_world("Hello"));
}
