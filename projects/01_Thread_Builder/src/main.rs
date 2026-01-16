use std::thread;
//线程建造者模式 构建线程
//优点：可以自定义线程名称和栈大小
//方便调试和报错的时候定位问题
fn main() {
    let handle = thread::Builder::new()
        .name("线程 1".into())//设置线程名称
        .stack_size(4 * 1024 * 1024)//设置栈大小 4MB
        .spawn(another_thread)
        .unwrap();

    handle.join().unwrap();
}

fn another_thread() {
    println!("当前线程的名字是 {:?}", thread::current().name());
}
