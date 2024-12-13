#![allow(dead_code)]

/// Sift up for Max heap
///
/// Pre: heap before i is valid
fn shift_up<T: PartialOrd>(heap: &mut Vec<T>, mut i: usize) {
    while i > 0 {
        let parent = (i - 1) / 2;
        if heap[i] > heap[parent] {
            heap.swap(i, parent);
            i = parent;
        } else {
            break;
        }
    }
}

/// Sift down for Max heap
///
/// Pre: left and right subtree of i are valid heaps
fn shift_down<T: PartialOrd>(heap: &mut Vec<T>, i: usize) {
    let left = i * 2 + 1;
    let right = i * 2 + 2;
    let mut max = i;

    if left < heap.len() && heap[left] > heap[max] {
        max = left;
    }
    if right < heap.len() && heap[right] > heap[max] {
        max = right;
    }

    if i != max {
        heap.swap(i, max);
        shift_down(heap, max);
    }
}

// [10 9 8 7 6 5]
//      10
//    9    8
//  7  6  5

/// Build Max Heap
fn build<T: PartialOrd>(v: &mut Vec<T>) {
    for i in (0..v.len() / 2).rev() {
        shift_down(v, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- SIFT UP
    #[test]
    fn test_sift_up_basic() {
        let mut heap = vec![10, 20, 5];
        shift_up(&mut heap, 2); // Index 2 contains 5
        assert_eq!(heap, vec![10, 20, 5]); // No change; heap is still valid
    }

    #[test]
    fn test_sift_up_promote_element() {
        let mut heap = vec![10, 20, 30];
        shift_up(&mut heap, 2); // Index 2 contains 30
        assert_eq!(heap, vec![30, 20, 10]); // 30 should move up
    }

    #[test]
    fn test_sift_up_multiple_swaps() {
        let mut heap = vec![5, 10, 15, 20];
        shift_up(&mut heap, 3); // Index 3 contains 20
        assert_eq!(heap, vec![20, 5, 15, 10]); // 20 should move up
    }

    // ---- SIFT DOWN
    #[test]
    fn test_sift_down_basic() {
        let mut heap = vec![20, 15, 10, 5, 12];
        shift_down(&mut heap, 0); // Root element is 20
        assert_eq!(heap, vec![20, 15, 10, 5, 12]); // No change; heap remains valid
    }

    #[test]
    fn test_sift_down_promote_child() {
        let mut heap = vec![10, 15, 12, 5, 6];
        shift_down(&mut heap, 0); // Root element is 10
        assert_eq!(heap, vec![15, 10, 12, 5, 6]); // 15 should move up
    }

    #[test]
    fn test_sift_down_multiple_swaps() {
        let mut heap = vec![5, 10, 15, 20, 25];
        shift_down(&mut heap, 0); // Root element is 5
        assert_eq!(heap, vec![15, 10, 5, 20, 25]); // 25 should move up
    }

    // ---- BUILD
    #[test]
    fn test_build_empty() {
        let mut v: Vec<i32> = vec![];
        build(&mut v);
        assert_eq!(v, vec![]); // Empty vector remains empty
    }

    #[test]
    fn test_build_single_element() {
        let mut v = vec![42];
        build(&mut v);
        assert_eq!(v, vec![42]); // Single-element vector is a valid heap
    }

    #[test]
    fn test_build_unsorted_array() {
        let mut v = vec![3, 2, 1, 5, 4];
        build(&mut v);
        assert_eq!(v, vec![5, 4, 1, 2, 3]); // Valid max heap
    }

    #[test]
    fn test_build_sorted_array() {
        let mut v = vec![1, 2, 3, 4, 5];
        build(&mut v);
        assert_eq!(v, vec![5, 4, 3, 1, 2]); // Valid max heap
    }
}
