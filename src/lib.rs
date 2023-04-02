pub struct Heap<T>
where
    T: PartialOrd,
{
    arr: Vec<T>,
    typ: HeapType,
}

#[derive(Clone, Copy)]
pub enum HeapType {
    Min,
    Max,
}

impl<T> Heap<T>
where
    T: PartialOrd,
{
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            arr: Vec::new(),
            typ: heap_type,
        }
    }

    pub fn heap_type(&self) -> HeapType {
        self.typ
    }

    pub fn size(&self) -> usize {
        self.arr.len()
    }

    pub fn peak(&self) -> Option<&T> {
        self.arr.first()
    }

    pub fn insert(&mut self, item: T) {
        self.arr.push(item);
        self.heapify_up();
    }

    pub fn remove(&mut self) -> Option<T> {
        let length = self.arr.len();
        if length < 1 {
            return None;
        }

        self.arr.swap(0, length - 1);
        let removed_item = self.arr.pop();
        self.heapify_down();
        removed_item
    }

    fn heapify_up(&mut self) {
        let mut idx = self.arr.len() - 1;
        while let Some(parent_idx) = self.heapify_up_parent_idx(idx) {
            self.arr.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn heapify_down(&mut self) {
        let mut idx = 0;
        while let Some(child_idx) = self.heapify_down_child_idx(idx) {
            self.arr.swap(idx, child_idx);
            idx = child_idx;
        }
    }

    fn heapify_up_parent_idx(&self, child_idx: usize) -> Option<usize> {
        let parent_idx = child_idx.checked_sub(1)? / 2;
        let child = self.arr.get(child_idx)?;
        let parent = self.arr.get(parent_idx)?;
        match &self.typ {
            HeapType::Min if child < parent => Some(parent_idx),
            HeapType::Max if child > parent => Some(parent_idx),
            _ => None,
        }
    }

    fn heapify_down_child_idx(&self, parent_idx: usize) -> Option<usize> {
        let left_child_idx = parent_idx * 2 + 1;
        let right_child_idx = left_child_idx + 1;
        let parent = self.arr.get(parent_idx)?;
        let left_child = self.arr.get(left_child_idx)?;
        let right_child = self.arr.get(right_child_idx)?;
        match &self.typ {
            HeapType::Min if parent > left_child && left_child <= right_child => {
                Some(left_child_idx)
            }
            HeapType::Min if parent > right_child && right_child <= left_child => {
                Some(right_child_idx)
            }
            HeapType::Max if parent < left_child && left_child >= right_child => {
                Some(left_child_idx)
            }
            HeapType::Max if parent < right_child && right_child >= left_child => {
                Some(right_child_idx)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Heap, HeapType};

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::<usize>::new(HeapType::Min);
        let nums = vec![3, 6, 2, 4, 6, 1, 2, 2, 0, 9, 5, 2, 3, 3, 9, 7, 2, 6];
        for i in nums {
            heap.insert(i);
        }

        let mut nums = vec![];
        while let Some(i) = heap.remove() {
            nums.push(i);
        }
        assert_eq!(
            nums,
            vec![0, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 5, 6, 6, 6, 7, 9, 9]
        );
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::<usize>::new(HeapType::Max);
        let nums = vec![3, 6, 2, 4, 6, 1, 2, 2, 0, 9, 5, 2, 3, 3, 9, 7, 2, 6];
        for i in nums {
            heap.insert(i);
        }

        let mut nums = vec![];
        while let Some(i) = heap.remove() {
            nums.push(i);
        }
        assert_eq!(
            nums,
            vec![9, 9, 7, 6, 6, 6, 5, 4, 3, 3, 3, 2, 2, 2, 2, 2, 1, 0]
        );
    }
}
