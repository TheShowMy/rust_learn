
use tokio::task::spawn_blocking;


async fn delay(task:u64,time:u64) {
    println!("任务 {} 正在开始。", task);
    //在spawn_blocking中可以执行阻塞操作
    //哪怕是单线程运行时，也会创建专门的线程池来处理这些阻塞操作
    let result = spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_secs(time as u64));
        time
    }).await;
    //如果不关心阻塞操作的结果
    // spawn_blocking(move || {
    //     std::thread::sleep(std::time::Duration::from_secs(time as u64));
    //     time
    // })
    println!("结果: {:?}", result);
    println!("任务 {} 已完成。", task);
}

#[tokio::main]
async fn main() {
    tokio::join!(
        delay(1,3),
        delay(2,2),
        delay(3,1),
    );
    println!("所有任务已完成。");
}