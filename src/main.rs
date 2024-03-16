struct Struct {
    e: i32,
}

fn main() {
    let mut x = 5;
    let _y = 10;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // .. 表示忽略 0 或多个，_ 表示忽略一个
    [c, .., d, _] = [1, 2, 3, 4, 5, 6, 7];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 6, 5], [a, b, c, d, e]);

    // shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

}
