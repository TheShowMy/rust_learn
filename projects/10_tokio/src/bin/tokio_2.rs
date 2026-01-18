///tokio 多线程运行时示例

use tokio::runtime;

async fn hi() {
    println!("Hi from Tokio 1.x!");
}

fn main() {
    let rt = runtime::Builder::new_multi_thread() //当前线程运行时
        .worker_threads(10)// 指定工作线程数
        .thread_stack_size(5*1024*1024)// 指定线程栈大小为5MB
        .event_interval(20)// 每20个事件轮询一次定时器
        .max_blocking_threads(256)// 指定最大阻塞线程数
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(hi());
}