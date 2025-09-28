/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.count >= self.items.len() {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }

        // 上浮新添加的元素
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if right_idx > self.count {
            // 右子节点不存在，只有左子节点
            left_idx
        } else {
            // 比较左右子节点，返回优先级更高的那个
            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                left_idx
            } else {
                right_idx
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 将根节点与最后一个元素交换
        self.items.swap(1, self.count);
        self.count -= 1;

        // 弹出（并返回）现在位于末尾的旧根节点
        // 我们需要处理 T::default() 占用的 items[0]
        // 所以实际的 pop 逻辑是取走 self.items[self.count + 1]
        let root = self.items.pop();

        // 如果堆在移除后不为空，则执行下沉操作
        if !self.is_empty() {
            let mut idx = 1;
            while self.children_present(idx) {
                let smallest_child_idx = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                    self.items.swap(idx, smallest_child_idx);
                    idx = smallest_child_idx;
                } else {
                    break;
                }
            }
        }

        root
    }

    // next 方法的另一种实现
    // fn next(&mut self) -> Option<T> {
    //     if self.is_empty() {
    //         return None;
    //     }

    //     self.items.swap(1, self.count);
    //     self.count -= 1;
    //     let root = self.items.pop().unwrap_or_default();

    //     // 下沉操作... (与上面相同)

    //     Some(root)
    // }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}