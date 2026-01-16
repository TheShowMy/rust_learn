// RwLock
// 读写锁
// ·RwLock允许在同一时刻可有多个读取者或最多一个写入者
// ·适用于经常被多线程读取，偶尔更新的场景
// 三种状态：
// ·未锁定
// ·由独占的写入者锁定
// ·由任意数量的读取者锁定

// std::sync::RwLock<T> T必须实现 Send 和 Sync 
// 锁定的方法
// read() --> RwLockReadGuard (Deref) 读取守卫(可以同时存在多个) 实现了 Deref 可以像引用一样使用
// write() --> RwLockWriteGuard (Deref & DerefMut) 写入守卫(独占) 实现了 DerefMut 可以像可变引用一样使用
// try_read() --> Result<RwLockReadGuard, TryLockError> 非阻塞式

// ·读写锁(RwLock)在发生panic时也可能进入中毒状态
// ·但只有当panic发生在独占模式（写入模式）下时，读写锁才会被中毒

use std::sync::{Arc, RwLock};

fn main() {
    let counter = Arc::new(RwLock::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            // 读取锁
            let read_guard = counter.read().unwrap();
            println!("当前计数值: {}", *read_guard);
        });
        handles.push(handle);      
    }

    {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            // 写入锁
            let mut write_guard = counter.write().unwrap();
            *write_guard += 1;
            println!("计数值已更新: {}", *write_guard);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("最终计数值: {}", *counter.read().unwrap());
}
