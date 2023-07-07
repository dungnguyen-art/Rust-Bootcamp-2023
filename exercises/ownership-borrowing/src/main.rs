// Exercise 1
// Make it compile
fn main() {
    println!("=======Hi VBI===========");
    exercise1();
    exercise2();
    exercise3();
    exercise4(1);
    exercise5();
    exercise6();
    exercise7();
    exercise8();
}

// Approache 1 - Dùng reference
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world"); // dạng string, lưu ở bộ nhớ heap. 
    let y = &x; // cả hai đều borrow giá trị, x vẫn là ownership của giá trị
    let z = &x; // cả hai đều borrow giá trị, x vẫn là ownership của giá trị
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}

// fn exercise1() {
//         // Use as many approaches as you can to make it work
//         let x = String::from("hello, world"); // dạng string, lưu ở bộ nhớ heap. 
//         let y = &x;
//         let z = x; // Lưu ý: Khi mượn rồi thì không được move nữa. 
//         // println!("x = {}", x);
//         println!("y = {}", y);
//         println!("z = {}", z);
// }
// // Approaches 2 - Use clone()
// fn exercise1() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("hello, world"); // dạng string, lưu ở bộ nhớ heap. 
//     let y = x.clone();
//     let z = &x; // Lưu ý: Khi mượn rồi thì không được move nữa. 
//     println!("x = {}", x);
//     println!("y = {}", y);
//     println!("z = {}", z);
// }

// // Approaches 3 - mutable reference
// fn exercise1() {
//     // Use as many approaches as you can to make it work
//     let mut x = String::from("hello, world"); // dạng string, lưu ở bộ nhớ heap. 
//     let y = &x;
//     let z = &mut x; // Lưu ý:  s
//     z.push_str(" - ntd");
//     // println!("x = {}", x);
//     // println!("y = {}", y);
//     println!("z = {}", z);
// }

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    //println!("{}", s);
    s
}
// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        for &element_index in &additions {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }
        // println!("In while")
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> String {
    let str_value = value.to_string(); // Convert u32 to String
    let str_ref: &str = &str_value; // Obtain a reference to the String
    str_ref.to_string()// Return the reference to the String
}

// Exercise 5
// Make it compile
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res = match my_map.get(&key) {
        Some(child) => child.clone(),
        None => {
            let value = "3.0".to_string();
            my_map.insert(key, value.clone());
            value // HERE IT FAILS
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    let mut prev_key = String::new();

    for line in io::stdin().lines() {
        let s = line.unwrap();

        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0].to_string();
        }
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    let mut v = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s = String::from_utf8(chars.to_vec()).unwrap();
        v.push(s);
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile

fn exercise8() {
    let mut accounting = vec!["Alice", "Ben"];

    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<String> = add_input.trim().split_whitespace().map(String::from).collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = &add_vec[0];
        accounting.push(person);

        break; // Break the loop after pushing the person
    }
}
