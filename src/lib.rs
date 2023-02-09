use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
//use std::result;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    let window_size: usize = k - 1;

    // Empty BinaryHeap as a max-heap, (value, index)
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    // Store first k-1 values into heap
    for (idx, val) in nums.iter().enumerate().take(window_size) {
        heap.push((*val, idx));
    }

    for (idx, val) in nums.iter().enumerate().take(n).skip(window_size) {
        // Push new element
        heap.push((*val, idx));

        while heap.peek().unwrap().1 < idx - window_size {
            // Remove max, which is outside the window
            heap.pop();
        }

        // Add heap head as a maximum of the current window
        maximums.push(heap.peek().unwrap().0);
    }
    maximums
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    let window_size: usize = k - 1;

    // Empty BST, (value, index)
    let mut tree: BinarySearchTree<(i32, usize)> = BinarySearchTree::new();

    // Insert first k-1 values into tree
    for (idx, val) in nums.iter().enumerate().take(window_size) {
        tree.insert((*val, idx));
    }

    for (idx, val) in nums.iter().enumerate().take(n).skip(window_size) {
        // Insert new element
        tree.insert((*val, idx));

        while tree.max().unwrap().1 < idx - window_size {
            // Remove max, which is outside the window
            tree.extract_max();
        }

        // Add the maximum element of the tree as a maximum of the current window
        maximums.push(tree.max().unwrap().0);
    }
    maximums
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut maximums = Vec::with_capacity(n - k + 1);
    let window_size: usize = k - 1;
    let window_size_i: isize = window_size as isize;

    println!("{}", k);

    // Empty Deque, (value, index)
    let mut deque: VecDeque<(i32, usize)> = VecDeque::new();

    for (idx, val) in nums.iter().enumerate() {
        let idx_i: isize = idx as isize;

        // Remove from head, which is outside the window
        while !deque.is_empty() && ((deque.front().unwrap().1 as isize) < (idx_i - window_size_i)) {
            deque.pop_front();
        }

        // Remove all the elements <= current one (val)
        while !deque.is_empty() && deque.back().unwrap().0 <= *val {
            deque.pop_back();
        }

        // Insert new element
        deque.push_back((*val, idx));

        // Window size reached
        if idx >= window_size {
            // Add head element of the deque as a maximum of the current window
            maximums.push(deque.front().unwrap().0);
        }
    }
    maximums
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k1_v100() {
        let k = 1;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k16_v100() {
        let k = 16;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k64_v100() {
        let k = 64;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k128_v1000() {
        let k = 128;
        let v = gen_random_vector(1000);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k256_v1000() {
        let k = 256;
        let v = gen_random_vector(1000);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k256_v10000() {
        let k = 256;
        let v = gen_random_vector(10000);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k5_v5() {
        let k = 5;
        let v = gen_random_vector(5);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k50_v50() {
        let k = 50;
        let v = gen_random_vector(50);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_heap_version_k512_v512() {
        let k = 512;
        let v = gen_random_vector(512);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k1_v100() {
        let k = 1;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k16_v100() {
        let k = 16;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k64_v100() {
        let k = 64;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k128_v1000() {
        let k = 128;
        let v = gen_random_vector(1000);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k256_v1000() {
        let k = 256;
        let v = gen_random_vector(1000);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k256_v10000() {
        let k = 256;
        let v = gen_random_vector(10000);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k5_v5() {
        let k = 5;
        let v = gen_random_vector(5);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k50_v50() {
        let k = 50;
        let v = gen_random_vector(50);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_bst_version_k512_v512() {
        let k = 512;
        let v = gen_random_vector(512);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k1_v100() {
        let k = 1;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k16_v100() {
        let k = 16;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k64_v100() {
        let k = 64;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k128_v1000() {
        let k = 128;
        let v = gen_random_vector(1000);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k256_v1000() {
        let k = 256;
        let v = gen_random_vector(1000);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k256_v10000() {
        let k = 256;
        let v = gen_random_vector(10000);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k5_v5() {
        let k = 5;
        let v = gen_random_vector(5);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k50_v50() {
        let k = 50;
        let v = gen_random_vector(50);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[test]
    fn test_linear_version_k512_v512() {
        let k = 512;
        let v = gen_random_vector(512);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
