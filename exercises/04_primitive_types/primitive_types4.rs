fn main() {
    let test: Vec<i32> = (1..=4).collect();
    println!("Test array {:?}", test);
        let test: [i32; 4] = (1..=4).collect::<Vec<i32>>().try_into().unwrap();
    println!("Test array {:?}", test);
}

#[cfg(test)]
mod tests {

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
