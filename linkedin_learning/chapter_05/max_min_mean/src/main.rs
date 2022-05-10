fn main() {
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mean: f32;
    
    for num in numbers {
        if num > max {
            max = num;
        } else {
            continue;
        }
    }

    for num in numbers {
        if num < min {
            min = num;
        } else {
            continue;
        }
    }

    let mut total: f32 = 0.0;
    for num in numbers {
        
        total += num as f32;
 
    }
    mean = total / numbers.len() as f32;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}