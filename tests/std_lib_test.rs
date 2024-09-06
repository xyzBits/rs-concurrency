use std::collections::HashMap;
use std::fs::File;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
#[test]
fn test_deref() {
    let x = DerefExample { value: 'a' };
    let result = *x;

    assert_eq!('a', result);

    let result = &x;
}

/// pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File>
#[test]
fn test_as_ref() {
    let str_ = "Cargo.toml";
    let string = String::from(str_);
    let path = Path::new(str_);
    let path_buf = PathBuf::from(".").join("Cargo.toml");

    File::open(str_).unwrap();
    File::open(string).unwrap();
    File::open(path).unwrap();
    File::open(path_buf).unwrap();
}

#[test]
fn test_deref_1() {
    let foo = Box::new(5i32);

    // 如何把一个类型为 &Box<i32> 的变量赋给 &i32 呢，因为 deref 发挥作用
    // 编译器注意到了 foo 的类型 Box<i32> 和 i32 不符合，但是 Box<i32> 实现了 Deref<i32>
    // 于是它尝试在 foo 上插入了 Deref
    let bar: &i32 = &foo;
    let bar = &(*(foo.deref()));

    // foo 被执行一次解引用后，类型由 Box<i32> 变为 &i32
    let step1: &i32 = foo.deref();
    let step2: i32 = *step1;
    let step3: &i32 = &step2;
    println!("step3: {}", step3);
    println!("{}", bar);
}

/// 如果一个类型 T，实现了 Deref<Target=U>
/// 那么对于类型为 T 的变量 value 来说
/// 在不可变的上下文中，*value 相当于 *Deref::deref(&value)
/// 类型为 &T 的值会被转换为 &U
/// T 隐式的实现了 U 中所有的方法，以 &self 为接收者
#[test]
fn test_deref_2() {
    let foo = Box::new(5i32);
    let step1 = *foo;
    let step2 = foo.deref();
    let step3 = *foo.deref();

    // split 方法实现于 str 而不是 String，但是我们仍然可以对 String split
    let foo = String::from("hello world");

    foo.split(" ").for_each(|item| println!("{item}"));

    let foo = vec![10, 20, 30];
    println!("foo.first: {:?}", foo.first());

    // 在使用 Mutex<T> 的时候，调用 lock 方法之后，返回的明明是一个类型为 MutexGuard<T> 的变量
    // 我们却可以像使用 T 本身一样使用它
    let foo = Mutex::new("hello world");
    let foo_guard = foo.lock().unwrap();
    foo_guard.split(" ").for_each(|item| println!("{item}"));

    // Deref 转换也会连续进行，直到无法再继续 Deref 或者匹配到正确的类型
    let foo = Box::pin(String::from("Hello world"));
    foo.split(" ").for_each(|s| println!("{s}"));
    // 如果没有 Deref 就需要写下面的这样的代码
    &(*foo)[..].split(" ").for_each(|s| println!("{s}"));
}

#[test]
fn test_map_or_insert() {
    let mut map = HashMap::new();
    let value = map.entry("hello".to_string()).or_insert(0);
    *value += 3;

    println!("{:?}", map);
}

fn stringify(x: u32) -> String {
    format!("error code: {}", x)
}

#[test]
fn test_result_map_err() {
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));

    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}
