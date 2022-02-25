fn main() {
    println!("Hello, world!");

    another_function();
    another_function_para(12, 34);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为: {}",x);
    println!("y 的值为: {}",y);

    fn five()->i32 {5}

    println!("five() 的值为: {}", five());
}

fn another_function() {
    println!("Hello, world!");
}

fn another_function_para(x: i32, y: i32) {
    println!("x 的值为: {}", x);
    println!("y 的值为: {}", y);
}