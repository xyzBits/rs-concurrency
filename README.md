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