fn main() {
    // let s = "hello";
    // println!("{}", s)

    // 一个String由3部分组成：指针、长度、容量
    // 三个部份存放stack，字符串内容存放heap
    let mut s = String::from("hello"); // 在heap上分配内存
    s.push_str(", world");
    let s1 = s;  // 在stack上复制一份指针、长度、容量，并没有复制heap上的数据
    // println!("{}", s);  // s moved to s1
    println!("{}", s1); // 如果离开作用域后，自动调用drop函数，并将s1使用的heap的内存释放

    // shallow copy (也许会将指针、长度、容量的复制可以看做是浅拷贝，但rust让s失效，s move到s1了)
    // deep copy (rust不会自动穿件数据的深拷贝，就运行时性能而言，任何自动赋值的操作都是廉价的)

    // Clone
    // 如果想对heap上的数据进行深度拷贝，可以使用clone方法
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
    // stack的数据
    // Copy trait, 可以用于完全放在stack上的数据类型
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // 总结：
    // 如果一个类型实现了copy trait，那么旧的变量在赋值后仍然可使用
    // 一些copy trait的类型：整数类型、bool、char、浮点、元组（字段都是copy类型的）
    // 如果一个数据类型或者该类型的一部分实现了drop trait，那么就不允许实现copy trait了。

    let s = String::from("Hello World");
    take_ownership(s);
    // println!("s: {}", s); // value borrowed here after move

    let x = 5;
    make_copy(x);
    println!("x: {}", x);

}


fn take_ownership(some_string: String) {
    println!("{}", some_string)
}

fn make_copy(some_number: i32) {
    println!("{}", some_number)
}