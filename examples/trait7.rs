trait Animal {
    fn name(&self) -> &str;
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> &str {
        "Dog"
    }
}

struct Cat;

impl Animal for Cat {
    fn name(&self) -> &str {
        "Cat"
    }
}

// 修改函数接受 &dyn Animal
fn print_animal_name(animal: &dyn Animal) {
    println!("{}", animal.name());
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals {
        // 通过解引用并取引用传递
        print_animal_name(&*animal);
    }
}