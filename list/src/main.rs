#![feature(linked_list_remove, linked_list_cursors)]

use csv::Writer;
use rand::Rng;
use std::collections::linked_list::LinkedList;
use std::time::Instant;

fn generate_indices(length: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut len = length;
    let mut removes = Vec::new();
    for _ in 0..length {
        let num = rng.gen_range(0..len);
        removes.push(num);
        len -= 1;
    }
    removes
}
fn generate_values(length: usize) -> Vec<usize> {
    // generate vector of random values
    let mut rng = rand::thread_rng();
    let mut values = Vec::new();
    for _ in 0..length {
        let num: usize = rng.gen_range(0..length);
        values.push(num);
    }
    values
}

fn fill_data_structures(length: usize) -> (Vec<usize>, LinkedList<usize>, u128, u128) {
    // initialize data structures
    let mut vec = Vec::new();
    let mut list = LinkedList::new();
    // generate random values
    let rng = generate_values(length);
    // create two copies of rng in because of borrow checker
    let rng2 = rng.clone();

    // start the timer for vector insertion
    let start_vec = Instant::now();
    for i in rng {
        // search the position to insert the value
        match vec.binary_search(&i) {
            Ok(pos) | Err(pos) => vec.insert(pos, i),
        }
    }
    // stop the timer
    let vec_insert_time = start_vec.elapsed().as_millis();

    // start the timer for list insertion
    let start_list = Instant::now();
    for i in rng2 {
        // get cursor to the front of the list
        let mut cursor = list.cursor_front_mut();
        // iterate trough the list until the value is greater than the current value
        loop {
            match cursor.current() {
                Some(val) if *val < i => {
                    cursor.move_next();
                }
                _ => break,
            }
        }
        // insert the value into the list
        cursor.insert_before(i);
    }
    // stop the timer
    let list_insert_time = start_list.elapsed().as_millis();
    // return the data structures and the times
    (vec, list, vec_insert_time, list_insert_time)
}

fn remove_elements(
    mut vec: Vec<usize>,
    mut list: LinkedList<usize>,
    removes: Vec<usize>,
) -> (u128, u128) {
    // start the timer for vector removal
    let start_vec = Instant::now();
    for i in &removes {
        // remove the value in the index of i
        vec.remove(*i);
    }
    // stop the timer
    let duration_vec = start_vec.elapsed().as_millis();

    // start the timer for list removal
    let start_list = Instant::now();
    for i in &removes {
        // remove the value in the index of i
        list.remove(*i);
    }
    // stop the timer
    let duration_list = start_list.elapsed().as_millis();
    // return the times
    (duration_vec, duration_list)
}

fn main() {
    // the starting length of the data structures
    let mut length = 10;
    // create the csv file
    let mut wtr = Writer::from_path("times.csv").unwrap();
    wtr.write_record([
        "length",
        "vec_insert_time",
        "list_insert_time",
        "vec_remove_time",
        "list_remove_time",
    ])
    .unwrap();

    for _ in 0..7 {
        // get data structures filled with sorted random values and times it took to fill them
        let (vec, list, vec_insert_time, list_insert_time) = fill_data_structures(length);
        // generate indices to remove
        let removes = generate_indices(length);
        // get times it took to remove elements
        let (vec_remove_time, list_remove_time) = remove_elements(vec, list, removes);
        // write the times to csv
        wtr.write_record(&[
            length.to_string(),
            vec_insert_time.to_string(),
            list_insert_time.to_string(),
            vec_remove_time.to_string(),
            list_remove_time.to_string(),
        ])
        .unwrap();
        // increase the length of the data structures and repeat
        length *= 10;
    }
    // flush the csv file
    wtr.flush().unwrap();
}