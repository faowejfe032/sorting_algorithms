mod input_arrays;

use std::time::SystemTime;
use crate::input_arrays::large_input_array;

fn main() {

    let mut input_array = large_input_array();
    let timer = SystemTime::now();
    selection_sort(&mut input_array);
    let duration = timer.elapsed().expect("error").as_millis();
    println!("selection_sort:");
    println!("\tSorted Array: {:?}", input_array);
    println!("\tTime (ms): {:?}", duration);

    let mut input_array = large_input_array();
    let timer = SystemTime::now();
    bubble_sort(&mut input_array);
    let duration = timer.elapsed().expect("error").as_millis();
    println!("bubble_sort:");
    println!("\tSorted Array: {:?}", input_array);
    println!("\tTime (ms): {:?}", duration);

    let mut input_array = large_input_array();
    let timer = SystemTime::now();
    merge_sort(&mut input_array);
    let duration = timer.elapsed().expect("error").as_millis();
    println!("merge_sort:");
    println!("\tSorted Array: {:?}", input_array);
    println!("\tTime (ms): {:?}", duration);

}

fn selection_sort(array: &mut [i32]) -> &mut [i32] {

    let mut minimum: i32;
    let mut index: usize = 0;
    let mut temp;

    for i in 0..array.len() {

        minimum = array[i];

        for j in i..array.len() {

            if array[j] < minimum {

                minimum = array[j];

                index = j;

            }

        }

        temp = array[i];

        array[i] = minimum;

        array[index] = temp;

    }

    array

}

fn bubble_sort(array: &mut [i32]) -> &mut [i32] {

    let mut i = array.len() - 1;
    let mut j = array.len() - 1;

    loop {

        if array[i-1] > array[i] {

            let temp = array[i];
            array[i] = array[i-1];
            array[i-1] = temp;

            j = array.len() - 1;

            if i > 1 {

                i -= 1;
                continue;

            }

        }

        i -= 1;
        j -= 1;

        if i == 0 {
            i = array.len() - 1;
        }

        if j == 0 {
            break;
        }

    }

    array

}

fn merge_sort(array: &mut [i32]) -> &mut [i32] {

    let middle = array.len()/2;
    let end = array.len();
    bubble_sort(&mut array[0..middle]);
    bubble_sort(&mut array[middle..end]);

    let (mut i, mut j) = (0, middle);

    let mut sorted_array: Vec<i32> = Vec::new();

    loop {

        if i < middle && j < end {

            if array[i] < array[j] {
                sorted_array.push(array[i]);
                i += 1;
                continue;
            } else {
                sorted_array.push(array[j]);
                j += 1;
                continue;
            }

        } else {



            if i < middle {
                sorted_array.push(array[i]);
                i += 1;
                continue;
            }

            if j < end {
                sorted_array.push(array[j]);
                j += 1;
                continue;
            }

            break;

        }

    }

    for i in 0..array.len() {
        array[i] = *sorted_array.get(i).expect("error");
    }

    array

}