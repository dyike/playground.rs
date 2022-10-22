fn main() {
    tuple_fun();
}



/// 元组类型
fn move_card(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}
fn tuple_fun() {
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    let card = (0, 1);
    let res = move_card(card);
    assert_eq!(res, (1, 2));
}


// 结构体
