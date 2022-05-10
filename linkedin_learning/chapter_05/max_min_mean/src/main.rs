fn main() {
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let max: i32;
    let min: i32;
    let mean: f32;

    min = minimum(numbers);
    max = maximum(numbers);
    mean = average(numbers);
   
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}

fn average(numbers: [i32; 14]) -> f32 {
    let mut total: f32 = 0.0;
    for num in numbers {
        total += num as f32;
    }
    let mean = total / numbers.len() as f32;
    return mean;
}

fn maximum(numbers: [i32; 14]) -> i32{
    let mut max: i32 = 0;
    for num in numbers {
        if num > max{
            max = num;
        } else {
            continue;
        }
    }
    return max;
}

fn minimum(numbers: [i32; 14]) -> i32{
    let mut min: i32 = 0;
    for num in numbers {
        if num < min {
            min = num;
        } else {
            continue;
        }
    }
    return min;
}