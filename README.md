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

`join` 等待子线程执行完成

`Actor` 所有人都可以给邮箱中发送消息，其他人可以收到消息，处理消息

线程是否一直运行取决于里面是否一直有死循环

主线程将矩阵乘法的 (idx, row, col) 发送给子线程，子线程计算完后，将 (idx, value)返回给主线程

## 矩阵乘法：从线性处理到并发处理
- 线性处理版本的矩阵乘法
- 单元测试
- 重构：改进核心算法来更加适用于并发处理
- 重构：构建线程间通讯的输入输出数据结构 
- 重构：使用多线程改进矩阵乘法
- 总结：从线性处理到并发处理的一般方法
- 总结：多线程处理的 Send/Sync/'static 
- 总结：泛型的约束

1. 将工作量大的，任务重的内容 抽取出来
2. 抽取时，要选择合适的接口，让他在多线程的环境下去使用
3. 将重逻辑放在线程中处理

oneshot: 是一种特殊类型的 channel，只允许发送一次消息，发送者发送一个消息后，就能再发送了，接收者接收到这个消息后，channel 就会关闭

## 使用并发 HashMap 来实时收集统计信息
- 需求：多线程下访问共享的 metrics table 
- Arc<T> Mutex<T> RwLock<T> 
- 构建 Arc<RwLock<HashMap<K, V>>> 版本
- 介绍 DashMap 
- 构建 DashMap<K, V> 版本
- 对于原子类型，可以用 AtomicXxx
- 构建 AtomicXxx 版本
- 小结：对于共享内存的一般处理方式
- 小结：进一步理解  Send / Sync 
- 作业：阅读 fearless concurrency 


## 异步处理的基本概念
- 什么是 Promise/Future
- 为什么需要异步处理
- Rust 为什么不直接提供运行时？
- Tokio 做了什么
- 探索异步处理的内部机制

分布式系统的一个节点 ---> 进程    ---> 线程   ---> 协程 
协程：运行在用户态下的

Output: Future 结束的时候，返回什么

```rust
#[tokio::main]
async fn main() {
    
}
```

执行步骤:
1. 当你使用 .await 时，去运行 future，放入 run queue 
2. 执行后，得到一个 pending，因为是系统  IO
3. 将其放入 wait queue 
4. 让 reactor 监听相关数据 
5. reactor 拿到数据后，从 wait queue 唤醒，放入 run queue 
6. executor 去 poll 

为什么需要 Pin ，被 Pin 在当前的地址下，不能被移动 

比如 堆上的一块内存，被栈上的一个变量 所 own，
一般来说，你可以将这个所有权 move 到另一个栈上的变量，
Pin 的意思就是说，你不可以移动

每个线程下都有一个 scheduler
如果自己的下面没有 任务，会去其他线程下面偷