use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

///
/// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
/// where
/// F: FnOnce() -> T,
///  F: Send + 'static,
///   T: Send + 'static,
///
///  F: FnOnce() -> T
/// 这是一个 trait ，他是一个整体，证明 F 是一个闭包，只能运行一次的闭包
///
/// Send 可以从一个线程移到另一个线程，指类型，是指所有权
/// 比如一个 String 你可以从一个线程中，move 到另一个线程里面
/// 但是有些
fn main() -> Result<()> {
    // 在不同的线程之间，可以使用 channel 来传送 message
    // 多生产者，单消费者的 channel
    let (sender, receiver) = mpsc::channel::<Msg>();

    // 创建 producers
    for i in 0..NUM_PRODUCERS {
        let sender = sender.clone();
        thread::spawn(move || producer(i, sender));
    }

    drop(sender); // 释放 tx，否则 rx 无法结束

    // 创建 consumer
    let consumer = thread::spawn(move || {
        for msg in receiver {
            println!("consumer: {:?}", msg);
        }

        println!("consumer exist");
        42
    });

    let secret = consumer
        // 主线程和子线程在这里进行汇聚
        // 不确定什么时候结束，因此需要 join 等待
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("secret: {:?}", secret);

    Ok(())
}

fn producer(idx: usize, sender: Sender<Msg>) -> Result<()> {
    loop {
        // 生成随机数
        let value = rand::random::<usize>();
        sender.send(Msg::new(idx, value))?;

        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }
    Ok(())
}
