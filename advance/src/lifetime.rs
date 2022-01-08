
// 隐式生命周期
fn foo(x: &str) -> &str {
    x
}
// 合上面的同作用
fn foo1<'a>(x: &'a str) -> &'a str {
    x
}

#[test]
fn test_foo() {
    let res = foo("123");
    println!("{}", res);
}