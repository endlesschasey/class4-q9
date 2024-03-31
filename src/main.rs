fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    
    for &num in numbers {
        match sum.checked_add(num) {
            Some(result) => sum = result,
            None => return None,
        }
    }
    
    Some(sum)
}

fn main() {
    // 测试用例 1: 正常求和
    let numbers1 = [1, 2, 3, 4, 5];
    match sum_u32(&numbers1) {
        Some(result) => println!("Sum of {:?} is {}", numbers1, result),
        None => println!("Overflow occurred while summing {:?}", numbers1),
    }
    
    // 测试用例 2: 溢出情况
    let numbers2 = [u32::MAX, 1];
    match sum_u32(&numbers2) {
        Some(result) => println!("Sum of {:?} is {}", numbers2, result),
        None => println!("Overflow occurred while summing {:?}", numbers2),
    }
    
    // 测试用例 3: 空切片
    let numbers3: [u32; 0] = [];
    match sum_u32(&numbers3) {
        Some(result) => println!("Sum of {:?} is {}", numbers3, result),
        None => println!("Overflow occurred while summing {:?}", numbers3),
    }
}