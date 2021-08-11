/// A generic quicksorting algorithm
/// Here term 'generic' refers to any type of objects where a partial order can
/// be established will be sorted.
use std::cmp::Ordering;

fn partition(arr: &mut Vec<i64>, low: usize, high: usize) -> usize {
    let pivot = arr[high - 1];
    let mut idx = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(idx, j);
            idx = idx + 1;
        }
    }
    idx
}

fn quicksort(arr: &mut Vec<i64>, low: usize, high: usize) -> &Vec<i64> {
    if low >= usize::MIN && high >= usize::MIN {
        if low < high {
            let mid = partition(arr, low, high);
            quicksort(arr, low, mid - 1);
            quicksort(arr, mid, high);
        }
    }
    arr
}

/// Sorts i64 elements in a vector.
pub fn sort(arr: &mut Vec<i64>) -> &Vec<i64> {
    quicksort(arr, usize::MIN, arr.len())
}

//////////////////////////////////////////////////////////////////////////////
// Generic implementation of quicksort (sort_gen)

/// Defined a partially-ordered comparator to be used to compare objects while
/// sorting.
pub trait Comparator {
    fn compare(&self, other: &Self) -> Ordering;
}

/// A cloning trait for moving objects when they are mutable.
pub trait Copier {
    fn copy(&self) -> Self;
}

fn partition_gen<T: Comparator + Copier>(arr: &mut Vec<T>, low: usize, high: usize) -> usize {
    let pivot: T = T::copy(&arr[high - 1]);
    let mut idx = low;

    for j in low..high {
        if T::compare(&arr[j], &pivot) == Ordering::Less {
            arr.swap(idx, j);
            idx = idx + 1;
        }
    }
    idx
}

fn quicksort_gen<'a, T: Comparator + Copier>(
    arr: &'a mut Vec<T>,
    low: usize,
    high: usize,
) -> &'a Vec<T> {
    if low >= usize::MIN && high >= usize::MIN {
        if low < high {
            let mid = partition_gen(arr, low, high);
            quicksort_gen(arr, low, mid - 1);
            quicksort_gen(arr, mid, high);
        }
    }
    arr
}

/// Sorts a vector of generic type, which must define a comparator and copy
/// trait.
pub fn sort_gen<'a, T: Comparator + Copier>(arr: &'a mut Vec<T>) -> &'a Vec<T> {
    quicksort_gen(arr, usize::MIN, arr.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    /// Represents a process node
    #[derive(Debug)]
    struct Node {
        pid: u64,
        name: String,
    }

    impl Node {
        fn new(p: u64, n: String) -> Self {
            Self { pid: p, name: n }
        }
    }

    impl Comparator for Node {
        fn compare(&self, other: &Self) -> Ordering {
            if self.pid > other.pid {
                return Ordering::Greater;
            }
            if self.pid == other.pid && self.name > other.name {
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
    }

    impl Copier for Node {
        fn copy(&self) -> Self {
            Node {
                pid: self.pid,
                name: self.name.clone(),
            }
        }
    }

    #[test]
    fn test_quicksort() {
        let mut rng = rand::thread_rng();

        let mut numbers: Vec<i64> = (0..25).map(|_| rng.gen_range(1..3001)).collect();
        sort(&mut numbers);

        let mut elem = numbers[0];
        for idx in 1..numbers.len() {
            assert!(elem < numbers[idx]);
            elem = numbers[idx];
        }
    }

    #[test]
    fn test_generic_quicksort() {
        let mut nodes = vec![
            Node::new(4, String::from("kobj2")),
            Node::new(2, String::from("kobj")),
            Node::new(1, String::from("systemd")),
        ];
        sort_gen(&mut nodes);

        let mut elem = Node::copy(&nodes[0]);
        for idx in 1..nodes.len() {
            println!("{:?} {:?}", elem, nodes[idx]);
            assert_eq!(Node::compare(&elem, &nodes[idx]), Ordering::Less);
            elem = Node::copy(&nodes[idx]);
        }
    }
}
