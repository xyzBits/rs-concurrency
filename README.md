# Rust Concurrency

## 创建线程：thread::spawn

## 等待线程: t.join()

## 线程同步：
    - 共享内存
    - CSP: channel
    - Actor: 传递消息，完成并发任务同步，

## 共享内存：
    - atomics
    - 线程间共享只读数据 Arc<T>
    - 线程间共享可写数据 Arc<Mutex<T>>

## CSP
    - mpsc 
    - oneshot

## 并发数据处理

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, {}
```
`FnOnce() -> T` 代表一个 trait，它是一个整体，代表 F 是一个闭包类型，只能执行一次的闭包类型
`Send` 代表所有权可以从一个线程 move 到另一个线程，比如有一个 String 你可以从一个 thread move 到另一个 thread 
但有些数据，就实现了 `!Send`，比如 Arc，如果可以在线程之间共享，那么就会出现在更新 reference count 时，临界区更新共享数据时，会有问题
绝大多数的数据类型，都实现了 `Send` 

`'static` 如果这个数据不是 static 的引用，必须是拥有所有权的数据
要么是 拥有所有权的数据，要么是一个全局的引用，也就是 static 的引用
