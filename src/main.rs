fn main() {
    let x = (-1.0_f32).sqrt();
    assert_ne!(x, x); // NaN != NaN

    if x.is_nan() {
        println!("未定义的数学行为");
    }
}
