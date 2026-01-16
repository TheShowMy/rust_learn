use std::{thread, time::Duration};
// 使用 scoped 线程确保线程在作用域结束前完成
// 可以直接使用外部变量
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
    //'scope 生命周期 代表线程作用域的生命周期 这个生命周期结束后 这个作用域的所有线程就会被回收 它的生命周期比 里面线程的生命周期更长
    thread::scope(|s| {
        // 生命周期为 non-'static 的线程
        for _i in 0..5 {
            let _b = "world".to_string();
            s.spawn(|| {
                thread::sleep(Duration::from_secs(1)); //延迟i秒
                println!("{a} 执行线程");
            });
        }
    });

    //'env 生命周期 代表被作用域线程引用的变量的生命周期 必须比 线程作用域更长 比如上面的 a 变量

    //更复杂的例子

    let chunck_size: usize = 10; // 每个线程处理的数据量
    let numbers: Vec<u32> = (0..10000).collect(); // 待处理的数据
    let chunks = numbers.chunks(chunck_size); // 将数据分块
    let sum = thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in chunks {
            let h = s.spawn(move || chunk.iter().sum::<u32>());
            handles.push(h);
        }

        handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    });
    println!("总和: {}", sum);
}
