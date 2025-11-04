// --- Simple Rust file to generate MIR/CFG for testing ---
// Function with an if/else branch
pub fn decide(x: i32) -> i32 {
    if x > 0 {
        x + 1
    } else {
        x - 1
    }
}

// Function with a loop and match — creates more basic blocks
pub fn process_numbers(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for n in nums {
        match n {
            0 => continue,           // branch 1
            1..=5 => sum += n,       // branch 2
            _ => sum += n * 2,       // branch 3
        }
    }
    if sum > 10 {
        sum * 2
    } else {
        sum
    }
}

fn main() {
    let result1 = decide(3);
    let result2 = process_numbers(&[1, 2, 3, 10]);

    println!("decide(3) = {}", result1);
    println!("process_numbers([1,2,3,10]) = {}", result2);
}
