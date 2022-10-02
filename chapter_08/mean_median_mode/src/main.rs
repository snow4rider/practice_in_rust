fn main() {

    //let v: i32 = vec![];
    let v = vec![15,26,38,49,53,60,75,82,90];
    let m = mean(&v);
    let med = median(&v);

    println!("The mean is {}. The median is {}.", m, med);
}

// fn sort
//fn sort(array: &mut Vec<i32>) {
    //for i in 0..array.len() {
        //for j in 0..array.len() - i - 1 {
            //if array[j + 1] < array[j] {
                // let tmp = array[j];
                // array[j] = array[j + 1];
                // array[j + 1] = tmp;
                //array.swap(j,j + 1);
            //}
        //}
    //}
//}

// fn mean 
fn mean(array: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(array.iter());
    f64::from(sum) / (array.len() as f64)
}

// fn median
fn median(array: &[i32]) -> f64 {
    let len = array.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&array[(mid - 1)..(mid + 1)])
    } else {
        f64::from(array[mid])
    }
}
// fn mode
fn mode(array: &[i32]) -> f64 {
    let len = array.len();
    

}