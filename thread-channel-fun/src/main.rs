use std::thread;
use std::sync::mpsc;
// 一个通道可以有多个发送端，职能有一个接收端

use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 为了新线程使用来自于主线程的数据，新线程的必报获取所需要的值，闭包职能是move修饰
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // tx move，rx还在主线程
        // println!("val is {}", val); // 这是不可以的，val已经转移了
    });

    let received = rx.recv().unwrap(); // 可以处理Result<T, E>
    // recv()阻塞，等待消息
    // try_recv()不阻塞，立即返回
    // thread::sleep(Duration::from_millis(2));
    // let received = rx.try_recv().unwrap();
    println!("Got: {}", received);


    // 接收多个消息
    // main1();


    // 多发单收
    // main2();
}

/// 接受多个消息
fn main1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}


/// 多发单收
fn main2() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); // 同一通道，增加一个发送者

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 一个通道一个接收着，接受两个线程的发送者
    for received in rx {
        println!("Got: {}", received);
    }

}
