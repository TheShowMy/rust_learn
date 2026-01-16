use std::sync::Arc;

///线程数据共享

//静态变量
// static DATA: [i32; 5] = [1, 2, 3, 4, 5];
// static mut COUNTER: i32 = 0;
fn main() {
    //多个线程共享静态变量
    // let mut handles = Vec::new();
    // for _ in 0..100 {
    //     let h = std::thread::spawn(|| {
    //         println!("静态变量 DATA: {:#?}", DATA);
    //     });
    //     handles.push(h);
    // }
    // for h in handles {
    //     h.join().unwrap();
    // }

    //多个线程修改静态变量 不安全
    // let mut handles2 = Vec::new();
    // for _ in 0..10000 {
    //     let h = std::thread::spawn(|| unsafe {
    //         COUNTER += 1;
    //     });
    //     handles2.push(h);
    // }
    // handles2.into_iter().for_each(|h| h.join().unwrap());
    // println!("静态变量 COUNTER: {}", unsafe { COUNTER });

    // 使用Box::leak 创建 'static 生命周期的变量
    // let data: &'static [i32; 6] = Box::leak(Box::new([0, 1, 2, 3, 4, 5]));
    // let mut handles3 = Vec::new();
    // for _ in 0..10000 {
    //     let h = std::thread::spawn(move || {
    //         println!("静态变量 data: {:#?}", data);
    //     });
    //     handles3.push(h);
    // }
    // handles3.into_iter().for_each(|h| h.join().unwrap());

    // 使用 Arc 创建 原子引用计数的共享数据
    let data2 = Arc::new([0, 1, 2, 3, 4, 5]);
    let mut handles3 = Vec::new();
    for _ in 0..10000 {
        let data = Arc::clone(&data2);
        let h = std::thread::spawn(move || {
            println!("静态变量 data: {:?}", data);
        });
        handles3.push(h);
    }
    handles3.into_iter().for_each(|h| h.join().unwrap());
}
