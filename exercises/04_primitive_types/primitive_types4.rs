fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);

        // TRY SOMETHING
        // let mut a_cloned = a.clone();
        // dbg!(a_cloned);

        // let edit_slice = &mut a_cloned[1..4];

        // dbg!(&edit_slice);

        // edit_slice[0] = 0;
        // edit_slice[1] = 0;
        // edit_slice[2] = 0;

        // dbg!(&edit_slice);
    }
}
