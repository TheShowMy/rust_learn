///tokio 单线程运行时示例

async fn hi() {
    println!("Hi from Tokio 1.x!");
}

// fn main() {
//     let rt = runtime::Builder::new_current_thread() //当前线程运行时
//         .enable_all()
//         .build()
//         .unwrap();
//     rt.block_on(hi());
// }

// 使用属性宏简化异步主函数的编写
#[tokio::main(flavor = "current_thread")]// 指定使用当前线程运行时
async fn main() {
    hi().await;
}
