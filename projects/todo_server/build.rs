use std::process::Command;
use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let my_app_dir = "my-app";

    println!("cargo:rerun-if-changed={}/src", my_app_dir);
    println!("cargo:rerun-if-changed={}/package.json", my_app_dir);
    println!("cargo:rerun-if-changed={}/vite.config.ts", my_app_dir);

    let status = if profile == "release" {
        println!("cargo:warning=Building my-app in release mode...");
        Command::new("pnpm")
            .args(["build"])
            .current_dir(my_app_dir)
            .status()
    } else {
        println!("cargo:warning=Building my-app in debug mode...");
        Command::new("pnpm")
            .args(["build"]) // Vite 通常只有 build 命令来生成 dist，dev 是启动开发服务器
            .current_dir(my_app_dir)
            .status()
    };

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=my-app built successfully.");
        }
        Ok(s) => {
            panic!("my-app build failed with status: {}", s);
        }
        Err(e) => {
            panic!("Failed to execute pnpm build: {}", e);
        }
    }
}
