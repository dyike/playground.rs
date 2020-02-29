fn main() {
    // test_match_fun();

    test_if_let_fun();
}


fn test_match_fun() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1...3 => println!("All"),
        | 5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),   // @可以将模式中的值绑定给一个变量，供右侧代码使用
        _ => println!("Common"),  // _ 为通配符，处理剩余情况
    }
}


fn test_if_let_fun() {
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }

    assert_eq!(binary, 1);
}