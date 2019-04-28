//Reference pointers - Point to a resource in memory
pub fn run() {
    //primitive Array
    let arr1 = [1,2,3,4,5];
    let arr2 = arr1;

    println!("Vals {:?}", (arr1, arr2));

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Vals {:?}", (&vec1, vec2));

}