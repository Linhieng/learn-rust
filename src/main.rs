fn main() {
    assert_eq!(9, u8::MAX.wrapping_add(10));
    assert_eq!(None, u8::MAX.checked_add(10));
    assert_eq!((9, true), u8::MAX.overflowing_add(10));
    assert_eq!(255, u8::MAX.saturating_add(10));

    // println!("{}", 10_u8.checked_add(10));
    println!("{:?}", 10_u8.checked_add(10)); // Option<u8>

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2); // 不报错
    assert!(xyz.0 + xyz.1 == xyz.2); // 报错
}
