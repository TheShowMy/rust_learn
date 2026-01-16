use std::cell::Cell;

// Cell<T>
// ·针对实现了Copy的类型
// ·get方法可通过复制的方式获取内部值
// 针对实现了Default的类型
// ·take方法会将当前内部值替换为Default::defaulto，并返回原来的值
// ，针对所有类型
// ·Replace方法，替换当前内部值，返回原来的内部值
// ·into_inner方法，消耗(consume)掉这个Cell<T>，并返回内部值
// ·set方法，替换当前的内部值，丢弃原来的值
// ·Cell<T>一般用于简单类型（如数值），因为复制/移动不会太消耗资源
// ·可能情况下优先使用Cell<T>而不是其它的Cell类型
// ·对于较大或者不可复制（non-copy）的类型，RefCell更有优势

fn main() {
    let cell = Cell::new(5);
    assert_eq!(cell.get(), 5); //字面量5是Copy类型 可以直接通过get方法copy出来 只有实现了Copy trait的类型才能使用get方法

    assert_eq!(cell.replace(10),5); //replace方法 替换当前内部值为10 并返回原来的值5 replace方法适用于所有类型
    assert_eq!(cell.get(), 10); //当前内部值被替换为10

    let ten = cell.into_inner(); //into_inner方法 消耗掉这个Cell<T> 并返回内部值
    //取消这个注释 将会报错 因为cell已经被消耗掉了
    // cell.get(); 
    assert_eq!(ten, 10); //ten的值为10

    let cell = Cell::new(String::from("hello"));
    assert_eq!(cell.take(), "hello"); //String不是Copy类型 不能直接通过get方法获取 所以使用take方法获取 只有实现了Default trait的类型才能使用take方法
    assert_eq!(cell.take(), String::default()); //使用task后 cell内部值被替换为默认值String::new()

    cell.set(String::from("world")); //set方法 替换当前的内部值为"world" 丢弃原来的值
    assert_eq!(cell.take(), "world"); //再次使用take方法获取内部值
}
