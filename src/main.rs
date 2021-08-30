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

    let mut input_array = large_input_array();
    let timer = SystemTime::now();
    quick_sort(&mut input_array);
    let duration = timer.elapsed().expect("error").as_millis();
    println!("quick_sort:");
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

fn merge_sort(array: &mut [i32]) {

    let len = array.len();

    if len <= 1 {
        return
    }

    merge_sort(&mut array[0..len/2]);
    merge_sort(&mut array[(len/2)..len]);
    merge(array);

}

fn merge(array: &mut [i32]) {

    let len = array.len();
    let mid = len/2;

    let mut vector: Vec<i32> = Vec::new();

    let (mut i, mut j) = (0, mid);

    loop {

        if i < mid && j < len {

            if array[i] < array[j] {

                vector.push(array[i]);
                i += 1;

            } else {

                vector.push(array[j]);
                j += 1;

            }

            continue;

        }

        if !(i < mid) {

            for j in j..len {
                vector.push(array[j]);
            }

            for k in 0..array.len() {
                array[k] = *vector.get(k).expect("error");
            }

        }

        if !(j < len) {

            for i in i..mid {
                vector.push(array[i]);
            }

            for k in 0..array.len() {
                array[k] = *vector.get(k).expect("error");
            }

        }

        break;

    }

}

fn quick_sort(array: &mut [i32]) -> &mut [i32] {

    let len = array.len();

    if len <= 1 { return array; }

    let middle = partition(array);
    quick_sort(&mut array[0..middle]);
    quick_sort(&mut array[(middle+1)..len]);

    array

}

fn partition(array: &mut [i32]) -> usize {

    let mut lesser: Vec<i32> = Vec::new();
    let mut greater: Vec<i32> = Vec::new();

    let partition_point = array[0];

    for i in 1..array.len() {

        if array[i] < partition_point {

            lesser.push(array[i]);

        } else {

            greater.push(array[i]);

        }

    }

    for i in 0..lesser.len() {

        array[i] = *lesser.get(i).expect("error");

    }

    array[lesser.len()] = partition_point;

    for i in 0..greater.len() {

        array[i+lesser.len()+1] = *greater.get(i).expect("error");

    }

    lesser.len()

}