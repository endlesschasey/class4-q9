# 第四课作业

## Question 9: 

实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None

```rust
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
```

```output
Sum of [1, 2, 3, 4, 5] is 15
Overflow occurred while summing [4294967295, 1]
Sum of [] is 0
```
