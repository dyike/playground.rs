fn main() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;  // 原生指针
    let y = Box::new(20);  // 堆内存上的数字20，
    let ptr_y = &*y as *const i32;  // 转换成原生指针
    
    // 操作原生指针需要使用unsafe
    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);
}
