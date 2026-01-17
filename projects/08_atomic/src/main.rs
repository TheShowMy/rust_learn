// 原子类型
// Atomic Types
// 位于std::sync:atomic，以Atomic 开头
// 例如:AtomicBool, Atomiclsize, AtomicUsize, Atomicl8, AtomicU16...
// 内部可变性
// 允许通过共享引用进行修改(例如 &AtomicUsize)
// 相同的接口：
// 加载与存储(load/store)
// 获取并修改(fetch-modify)
// 比较并交换(compare-exchange)

use std::{sync::atomic::{AtomicUsize, Ordering}, thread::{self, Thread}, time::Duration};

fn main() {
    let done = AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..5 {
            s.spawn(|| {
                for i in 0..100 {
                    thread::sleep(Duration::from_millis(20));
                    // 这里分成了两步实现 所有不是原子操作
                    // let current = done.load(Ordering::Relaxed);// 读取当前值 不关心排序
                    // done.store(current + 1, Ordering::Relaxed);// 存储新值 不关心排序

                    done.fetch_add(1, Ordering::Relaxed);// 原子操作 读取并增加1 不关心排序
                }
            });
        }

        loop {
            let n = done.load(Ordering::Relaxed);
            if n == 500 {
                break;
            } else {
                println!("当前进度: {}/500", n);
                thread::sleep(Duration::from_millis(1000));
            }
        }
    });

    print!("完成！\n");
}
