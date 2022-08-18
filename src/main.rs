use std::collections::{ HashMap, HashSet };
use std::time::{ SystemTime, UNIX_EPOCH };

fn main() {
    let test_1 = vec![2, 3, 5];
    let test_2 = vec![6, 1, 2, 4];
    let test_3 = vec![20, 70, 40, 100, 10];
    let test_4 = vec![1, 1, 2, 100, 23, 38, 1, 22, 18, 17, 3, 2, 10, 15, 1, 1, 1];
    let test_5 = vec![1, 2, 3, 4, 5, 6, 7];
    let test_6: Vec<i32> = (1..=100).collect();
    println!("test 1: {:?}", find_equal_sums(&test_1));
    println!("test 2: {:?}", find_equal_sums(&test_2));
    println!("test 3: {:?}", find_equal_sums(&test_3));
    timer(&|| println!("test 4: {:?}", find_equal_sums(&test_4)));
    timer(&|| println!("test 5: {:?}", find_equal_sums(&test_5)));
    timer(&|| println!("test 6: {:?}", find_equal_sums(&test_6)));
    timer(&|| println!("test 6 (bool): {:?}", contains_equal_sums(&test_6)));
}

fn find_equal_sums(numbers: &Vec<i32>) -> Option<Vec<i32>> {
    let sum: i32 = numbers.iter().sum();
    if sum % 2 == 1 { return None }
    let target_sum = sum / 2;
    let mut sums_map: HashMap<i32, Vec<i32>> = HashMap::new();
    sums_map.insert(0, vec![]);
    let mut sums_set: HashSet<i32> = HashSet::new();
    sums_set.insert(0);
    for n in numbers {
        let mut added_keys: Vec<i32> = Vec::new(); 
        for k in &sums_set {
            let new_key = *k + *n;
            added_keys.push(new_key);
            if !sums_map.contains_key(&new_key) {
                let mut sum_list = sums_map[&k].clone();
                sum_list.push(*n);
                sums_map.insert(new_key, sum_list);
            }
            if new_key == target_sum {
                return Some(sums_map[&new_key].clone());
            }
        }
        for k in added_keys {
            sums_set.insert(k);
        }
    }
    None
}

fn contains_equal_sums(numbers: &Vec<i32>) -> bool {
    let sum: i32 = numbers.iter().sum();
    if sum % 2 == 1 { return false }
    let target_sum = sum / 2;
    let mut sums_set: HashSet<i32> = HashSet::new();
    sums_set.insert(0);
    for n in numbers {
        let mut added_keys: Vec<i32> = Vec::new(); 
        for k in &sums_set {
            let new_key = *k + *n;
            added_keys.push(new_key);
            if new_key == target_sum {
                return true;
            }
        }
        for k in added_keys {
            sums_set.insert(k);
        }
        // println!("{} {:?}", n, &sums_set);
    }
    false
}

fn timer(f: &dyn Fn()) {
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    f();
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    println!("timer: {} s", (end_time - start_time) as f64 / 1_000_000_000.0);
}