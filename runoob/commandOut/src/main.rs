fn main() {
    let a = -12;
    let b;
    if a > 0 {
        b = 1;
    }else {
        b = -1
    }

    let number = if a > 0 {10} else {-10};
    println!("b的值为: {}", b);
    println!("number的值为: {}", number);
}