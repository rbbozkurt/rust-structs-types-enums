fn wrap_with(mut elements:  Vec<i32>, wrapper: i32) -> Vec<i32> {
    elements.insert(0, wrapper);
    elements.push(wrapper);
    elements
}

fn merge_vectors ( mut first : Vec<i32>, second : Vec<i32>) -> Vec<i32> {
    first.extend(second);
    first 
}

fn main() {
    let mut v = vec![1, 2, 3];
    //v.push(4);
    v = wrap_with(v, 0);
    println!("{:?}",v);
    println!("{:?}", merge_vectors(v, vec![4, 5, 6]));
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
   // let more_numbers = vec![5, 6];
    //v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    //let mut other_numbers = vec![7, 8];
    //v.append(&mut other_numbers);
    //println!("{:?}", v);

    // insert items at a given index
    //v.insert(0, 0);
    //println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 
}
