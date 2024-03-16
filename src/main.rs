fn main() {
    let x =define_x();
    let y = define_y();
    println!("{}, world", x);
    println!("{}, world", y);

    let mut z = 1;
    println!("{}", z);
    z = 2;
    let z = z;
    println!("{}", z);

    test_unused_variables();


    let (mut i, j) = (1, 2);
    assert_eq!(i, 1);
    i = 2;
    assert_eq!(i, 2);
    assert_eq!(j, 2);

}

fn define_x() -> String {
    let x = "hello".to_string();
    return x
}

fn define_y() -> &'static str {
    let x = "hello";
    x
}

#[allow(unused_variables)]
fn test_unused_variables() {
    let x = 123;
}
