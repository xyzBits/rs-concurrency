use std::pin::Pin;
use tracing_subscriber::util::SubscriberInitExt;

#[test]
fn test_self_reference() {
    let s = "Hello, world!".to_string();

    /*    let _ = SelfReference {

        // - value moved here
        a: s,

        // value borrowed here after move
        // borrow of moved value: 's'
        b: &s,
    };*/
}

struct SelfReference {
    a: String,
    b: *const String,
}

impl SelfReference {
    fn new(msg: &str) -> SelfReference {
        Self {
            a: msg.to_string(),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let ptr_to_a = &self.a as *const _;
        self.b = ptr_to_a;
    }

    fn get_a(&self) -> &str {
        &self.a
    }

    fn get_b(&self) -> &str {
        unsafe { &*self.b }
    }
}

#[test]
fn test_row_pointer() {
    let mut sr_1 = SelfReference::new("Hello");
    sr_1.init();

    let mut sr_2 = SelfReference::new("World");
    sr_2.init();

    println!("sr_1: {{ a: {}, b: {}  }}", sr_1.get_a(), sr_1.get_b());
    println!("sr_2: {{ a: {}, b: {}  }}", sr_2.get_a(), sr_2.get_b());
}

///
/// Before swap
/// sr_1: { a: Hello, b: Hello  }
/// sr_2: { a: World, b: World  }
///
/// After swap
/// sr_1: { a: World, b: Hello  }
/// sr_2: { a: Hello, b: World  }
///
/// 在两者交换后，字段 a 的数据也发生了交互，但是字段 b 的数据没有改变，仍然指向之前的位置
/// 这意味着 sr_1 sr_2 将不再是自引用结构体，并且保存了一个指向其他对象的裸指针
/// sr 的 字段 b 的生命周期将不再和其结构体本身相关联，难以保证 sr.b 的指针不会变成悬垂指针
#[test]
fn test_swap_memory() {
    let mut sr_1 = SelfReference::new("Hello");
    sr_1.init();

    let mut sr_2 = SelfReference::new("World");
    sr_2.init();

    println!("Before swap");
    println!("sr_1: {{ a: {}, b: {}  }}", sr_1.get_a(), sr_1.get_b());
    println!("sr_2: {{ a: {}, b: {}  }}", sr_2.get_a(), sr_2.get_b());

    std::mem::swap(&mut sr_1, &mut sr_2);

    println!("\nAfter swap");
    println!("sr_1: {{ a: {}, b: {}  }}", sr_1.get_a(), sr_1.get_b());
    println!("sr_2: {{ a: {}, b: {}  }}", sr_2.get_a(), sr_2.get_b());
}

#[derive(Debug)]
struct Foo {
    x: i32,
    y: i32,
}

impl Foo {
    fn new() -> Foo {
        Foo { x: 0, y: 0 }
    }
}

#[test]
fn test_foo_pin() {
    let box_foo = Box::new(Foo::new());

    let pin_foo = Pin::new(box_foo);

    let foo_ref = &*pin_foo;

    println!("{:?}", foo_ref);

    let pin1 = Box::pin(Foo::new());
}
