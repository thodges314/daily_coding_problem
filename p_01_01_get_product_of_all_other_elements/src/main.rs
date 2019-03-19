fn main() {
    println!("With division!");
    //define an array
    let mut array = [1, 2, 3, 4, 5];
    // pass a slice to the function, not an array.  This way we don't have to specify size.
    // The &mut designation beans that we are borrowing with mutation - the receiving function can mutate the
    // values that our reference is pointing to
    prod_array(&mut array);
    // Because our values were mutated, printing the array will show off changed values.
    println!("{:?}", array);
    let mut array_2 = [3, 2, 1];
    prod_array(&mut array_2);
    println!("{:?}", array_2);

    println!("Without division!");

    array = [1, 2, 3, 4, 5];
    prod_array_no_div(&mut array);
    println!("{:?}", array);
    array_2 = [3, 2, 1];
    prod_array_no_div(&mut array_2);
    println!("{:?}", array_2);
}

// WITH DIVISION //

// receives a mutable array reference (slice)
fn prod_array(array: &mut [i32]) {
    let mut prod = 1;
    // the iterator multiplies each value by the preceeding values, so we get the prdocut of all of the values
    for e in array.iter() {
        prod *= e;
    }
    // iter_mut differs from iter in that we can mutate the values iterated over
    for item in array.iter_mut() {
        // notice that we use * to dereference the values.  Otherwise we just get pointers.
        *item = prod / *item;
    }
}

// WITHOUT DIVISION //

// function to find product of all elements in an array excluding the element at an index
fn find_product(array: &[i32], index: usize) -> i32 {
    let mut prod = 1;
    for (i, e) in array.iter().enumerate() {
        if i != index {
            prod *= *e;
        }
    }
    prod
}

fn prod_array_no_div(array: &mut [i32]) {
    let mut elements: Vec<i32> = vec![0; array.len()];
    // .. operator stops one short of final value
    for i in 0..array.len() {
        elements[i] = find_product(&array, i);
    }
    for i in 0..array.len() {
        array[i] = elements[i];
    }
}
