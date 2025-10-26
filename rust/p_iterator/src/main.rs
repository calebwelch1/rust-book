// iterate over sequence of elements no matter how they are stored
// array, graph, hashmap whatever


fn main() {
    let v1 = vec![1, 2, 3];

    // iterates over vector
    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("got: {}", value);
    }
}
