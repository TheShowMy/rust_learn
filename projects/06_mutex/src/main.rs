// Mutex
// 适用于频繁读写共享数据的场景
// 互斥锁：一种用于保护共享数据的互斥原语
// ·原语(primitive)：最基本、不可再分解的操作或机制
// Mutex:
// ·最常见的用于在线程间分享（可变）数据的工具
// 只允许对数据的独占(exclusive)访问
// ·临时阻塞同一时刻想要访问数据的其它线程
// ·访问数据前需要请求锁(lock)
// 当完成时需要移除锁
// ·锁定(locked)
// ·未锁定(unlocked)
// 锁中毒
// ·当某个线程在持有Mutex时发生panic，这个互斥锁就会被视为已中毒
// ，一旦Mutex中毒
// ·默认情况下，其他线程将无法访问其所保护的数据
// ·因为这些数据很可能已被污染
// ·调用lock和try_lock方法会返回一个Result，用于指示该互斥锁是否已中毒
// ·中毒的Mutex并不会阻止对底层数据的所有访问
// ·PoisonError类型提供了一个 into_inner方法，可以返回原本在成功加锁时会
// 返回的守卫（guard）
// 即使互斥锁已中毒，仍然可以访问到它所保护的数据

use std::{
    sync::{Arc, Mutex},
    thread,
};

static NUMBERS: Mutex<Vec<i32>> = Mutex::new(vec![]);

fn main() {
    let mut handles = Vec::new();
    for i in 0..20 {
        let h = thread::spawn(move || {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(i);
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    let lock = NUMBERS.lock().unwrap();
    println!("总数: {} 内容:{:?}", lock.len(), *lock);

    //中毒示例
    let m = Arc::new(Mutex::new(0));
    {
        let m1 = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m1.lock().unwrap();
            *num = 1;
            panic!("线程发生panic");
        });
        let _ = handle.join().unwrap_err();
    }
    {
        let result = Arc::clone(&m);
        thread::spawn(move || {
            match result.lock() {
                Ok(_) => println!("成功获取锁"),
                Err(poisoned) => {
                    println!("锁已中毒，尝试恢复数据");
                    let mut num = poisoned.into_inner();
                    *num = 0; // 恢复数据
                    println!("数据已恢复为: {}", *num);
                }
            }
        });
    }
}
