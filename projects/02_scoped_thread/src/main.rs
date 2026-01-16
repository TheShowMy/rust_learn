use std::{thread, time::Duration};

fn main() {
    // 使用 scoped 线程确保线程在作用域结束前完成
    // let mut handles = vec![];

    // for i in 0..5 {
    //     生命周期为 'static 的线程
    //     let handle = std::thread::spawn(move || {
    //         thread::sleep(Duration::from_secs(i)); //延迟i秒
    //         println!("执行线程{}", i);
    //     });
    //     handles.push(handle);
    // }

    // handles.into_iter().for_each(|h| h.join().unwrap());

    // 使用 scoped 线程确保线程在作用域结束前完成
    // 可以直接使用外部变量

    let a = "hello".to_string();
    // 生命周期为 non-'static 的线程
    thread::scope(|s| {
        for _i in 0..5 {
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1)); //延迟i秒
                println!("{a} 执行线程");
            });
        }
    });
}
