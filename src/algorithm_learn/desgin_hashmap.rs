use std::ptr;
use std::ptr::null_mut;

struct Node {
    key: i32,
    value: i32,
    next: *mut Node
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            next: ptr::null_mut(),
        }
    }
}

struct MyHashMap {
    buckets: Vec<*mut Node>,
    capacity: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        let capacity = 1000;
        let mut buckets = Vec::with_capacity(capacity);

        // 初始化所有桶为空指针
        for _ in 0..capacity {
            buckets.push(ptr::null_mut());
        }
        MyHashMap { buckets, capacity }
    }

    fn hash(&self, key:i32) -> usize {
        (key.abs() as usize) % self.capacity
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = self.hash(key);
        let mut current = self.buckets[index];
        // 链地址法（Separate Chaining）
        while !current.is_null() {
            unsafe {
                if (*current).key == key {
                    (*current).value = value;
                    return;
                }
                current = (*current).next;
            }
        }
        // 键不存在，创建新节点并插入到链表头部
        let new_node = Box::into_raw(Box::new(Node::new(key, value)));
        unsafe {
            (*new_node).next = self.buckets[index];
        }
        self.buckets[index] = new_node;
    }

    fn get(&self, key: i32) -> i32 {
        let index = self.hash(key);
        let mut current = self.buckets[index];

        while !current.is_null() {
            unsafe {
                if (*current).key == key {
                    return (*current).value;
                }
                current = (*current).next;
            }
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let index = self.hash(key);
        let mut prev: *mut Node = ptr::null_mut();
        let mut current = self.buckets[index];

        while !current.is_null() {
            unsafe {
                if (*current).key == key {
                    if prev.is_null() {
                        self.buckets[index] = (*current).next;
                    } else {
                        (*prev).next = (*current).next;
                    }

                    let _ = Box::from_raw(current);
                    return;
                }

                prev = current;
                current = (*current).next;
            }
        }
    }
}

impl Drop for MyHashMap {
    fn drop(&mut self) {
        for i in 0..self.capacity {
            let mut current = self.buckets[i];
            while !current.is_null() {
                unsafe {
                    let next = (*current).next;
                    let _ = Box::from_raw(current);
                    current = next;
                }
            }
            self.buckets[i] = ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_map_operations() {
        let mut map = MyHashMap::new();

        // 测试插入和获取
        map.put(1, 10);
        map.put(2, 20);
        assert_eq!(map.get(1), 10);
        assert_eq!(map.get(2), 20);

        // 测试更新
        map.put(1, 100);
        assert_eq!(map.get(1), 100);

        // 测试获取不存在的键
        assert_eq!(map.get(3), -1);

        // 测试删除
        map.remove(1);
        assert_eq!(map.get(1), -1);
        assert_eq!(map.get(2), 20);

        // 测试删除不存在的键
        map.remove(3);
    }
}
