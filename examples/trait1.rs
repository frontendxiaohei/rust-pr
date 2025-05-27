


use std::ops::{Deref, DerefMut};

// 定义一个自定义的智能指针结构体
struct SmartPointer<T> {
    data: Box<T>,
    access_count: usize,
}

impl<T> SmartPointer<T> {
    fn new(value: T) -> Self {
        SmartPointer {
            data: Box::new(value),
            access_count: 0,
        }
    }

    fn get_access_count(&self) -> usize {
        self.access_count
    }
}

// 实现 Deref trait
impl<T> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("Deref called!");
        &*self.data
    }
}

// 实现 DerefMut trait
impl<T> DerefMut for SmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("DerefMut called!");
        self.access_count += 1;
        &mut *self.data
    }
}

// 定义一个结构体用于测试
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday! {} is now {} years old!", self.name, self.age);
    }
}

fn main() {
    // 创建一个 Person 实例并包装在 SmartPointer 中
    let mut smart_person = SmartPointer::new(Person {
        name: String::from("Alice"),
        age: 25,
    });

    // 使用 Deref 来读取数据
    println!("Initial state: {:?}", *smart_person);
    println!("Access count: {}", smart_person.get_access_count());

    // 使用 DerefMut 来修改数据
    smart_person.celebrate_birthday();
    println!("After birthday: {:?}", *smart_person);
    println!("Access count: {}", smart_person.get_access_count());

    // 直接修改内部数据
    (*smart_person).name = String::from("Alice Smith");
    println!("After name change: {:?}", *smart_person);
    println!("Final access count: {}", smart_person.get_access_count());
}