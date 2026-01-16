
// RefCell<T>
// ·与Cell<T>不同，RefCell<T>允许直接借用它的内部值
// ·有一点运行时开销
// ·RefCell<T>不仅持有T的值，还持有一个计数器，用来追踪有多少个借用
// ·借用是在运行时被追踪的
// ·Rust原生的引用类型则完全是在编译时静态检查的
// 方法
// ·borrow，可以获取对RefCell内部值的不可变引用(&T) 当存在可变引用时调用会 panic
// ·borrow_mut，可以获取对RefCell内部值的可变引用(&mut T) 当存在任何引用时调用会 panic
// 其他
// try_borrow, try_borrow_mut 与 borrow 和 borrow_mut 类似 但不会 panic 而是返回 Result
// into_inner
// replace
// take

use std::cell::RefCell;

fn main() {
    let rc = RefCell::new(5);
    {
        let b1 = rc.borrow(); //获取不可变引用
        let b2 = rc.borrow(); //可以多次获取不可变引用
        assert_eq!(*b1, 5);
        assert_eq!(*b2, 5);
        // let mut b3 = rc.borrow_mut(); //尝试获取可变引用 会panic 因为已经存在不可变引用
    } //b1 b2 超出作用域 被释放
    println!("修改前: {}", rc.borrow());
    {
        let mut b3 = rc.borrow_mut(); //获取可变引用
        *b3 += 1; //修改内部值
        assert_eq!(*b3, 6);
        //let b4 = rc.borrow(); //尝试获取不可变引用 会panic 因为已经存在可变引用
    } //b3 超出作用域 被释放
    println!("修改后: {}", rc.borrow());
    *rc.borrow_mut() += 1; //特殊写法 直接修改内部值 生成一个临时的可变引用 修改完成后 临时引用被释放
    println!("再次修改后: {}", rc.borrow());
    {
        let mut b3 = rc.borrow_mut();
        *b3 += 10;
        let v = rc.try_borrow(); //尝试获取不可变引用 不会panic 会返回Err 因为已经存在可变引用
        assert!(v.is_err());
        let v = rc.try_borrow_mut(); //尝试获取可变引用 不会
        assert!(v.is_err());
    }
}
