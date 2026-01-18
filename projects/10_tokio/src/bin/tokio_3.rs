use tokio::task::JoinSet;


async fn hi() {
    println!("Hello, world!");
}

async fn hi2() {
    for i in 0..10000000 {
        let _ = i*2;
    }
    println!("Hello, world! - 2");
}

async fn run() {
    for i in 0..5 {
        println!("{i}");
        tokio::task::yield_now().await;// 主动让出当前任务的执行权
    }
}

async fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    a + b
}

#[tokio::main]
async fn main() {
    tokio::spawn(run());// 添加一个异步任务到运行时
    hi().await;
    hi2().await;

    let result = tokio::join!(add(1, 2), add(3, 4));
    println!("结果: {:?}", result);

    let mut set = JoinSet::new();
    for i in 0..5 { 
        set.spawn(add(i, i * 2));
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(value) => println!("完成任务的结果: {}", value),
            Err(e) => println!("任务失败: {}", e),
        }
    }
}