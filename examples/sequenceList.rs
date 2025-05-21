// 实现一个顺序表的数据结构

// 顺序表是一个线性表的实现，使用数组来存储元素
// 顺序表的特点是：
// 1. 支持随机访问，可以通过索引直接访问元素
// 2. 插入和删除操作的时间复杂度为O(n)，因为需要移动元素
// 3. 顺序表的大小是固定的，不能动态扩展
// 4. 顺序表的内存空间是连续的，适合存储大量数据

trait SequenceListTrait<T> {
    fn new(size: usize) -> Self;
    fn insert(&mut self, index: usize, value: T);
    fn delete(&mut self, index: usize);
    fn get(&self, index: usize) -> Option<&T>;
    fn size(&self) -> usize;
}


struct SequenceList<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> SequenceListTrait<T> for SequenceList<T> {
    fn new(size: usize) -> Self {
        SequenceList {
            data: Vec::with_capacity(size),
            size,
        }
    }

    fn insert(&mut self, index: usize, value: T) {
        if index > self.size {
            panic!("Index out of bounds");
        }
        if self.size == self.data.capacity() {
            panic!("SequenceList is full");
        }
        self.data.insert(index, value);
        self.size += 1;
    }

    fn delete(&mut self, index: usize) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        self.data.remove(index);
        self.size -= 1;
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            None
        } else {
            Some(&self.data[index])
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}


fn main() {
    let mut list = SequenceList::new(10);
    list.insert(0, 1);
    list.insert(1, 2);
    list.insert(2, 3);
    list.insert(3, 4);
    list.insert(4, 5);

    println!("Size: {}", list.size());
    for i in 0..list.size() {
        println!("Element at index {}: {:?}", i, list.get(i));
    }

    list.delete(2);
    println!("After deletion:");
    for i in 0..list.size() {
        println!("Element at index {}: {:?}", i, list.get(i));
    }
}

