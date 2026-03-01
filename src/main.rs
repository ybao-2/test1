fn main() {
    println!("Hello, world!");

    // mut - can be changed
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);

    /*
    let a = 10;
    let b = 3.0;
    let c = a as f64 / b;
    println!("c is {0:08.3}\na is {1}\nonce again, a is {1}", c, a);

    let d = 66;
    println!("d is {d}");
    */

    let a = 13;
    let b = 2.3;
    let c:f32 = 120.0;

    let average = (a as f64 + b + c as f64)/3.0;

    assert_eq!(average,45.1);
    println!("Test passed!");


    let letters = ['a', 'b', 'c'];
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5]; //creates an array to hold i32 integers and has length of 5
    numbers = [0; 5]; //initializes array with 5 copies of value 0
    //let index:usize = numbers.len();
    println!("last number is {}", numbers[4]);

    let parking_lot = [[1,2,3],
                                      [4,5,6]];
    let num = parking_lot[0][1];
    println!("num is {num}");
    //let garage: [[[i32;100]; 20]; 5]; // 100 elements, 20 rows, 5 floors

    // tuple - group multiple items of mixed data types
    let mut stuff: (i32, f64, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first item is {}", first_item);

    //let(x,y,z) = stuff;
    //println!("y is {}", y);


    let result = square(13);
    println!("result is {:?}", result);

    
    let celcius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celcius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed for temperature conversion");


    let mut count = 0;
    let num2 = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("result is {}", num2);

    // for loop example
    let message = ['h','e','l','l','o'];
    for(index, &item) in message.iter().enumerate(){
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    let mut matrix = [[1,2,3],[4,5,6],[7,8,9]];
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }

    //max, min, mean
    let nums = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = nums[0];
    min = nums[0];
    mean = 0.0;
    for  &i in nums.iter(){
        if i > max {
            max = i;
        }else if i < min {
            min = i;
        }
        mean += i as f64;
    }
    mean = mean / nums.len() as f64;

    assert_eq!(max,56);
    assert_eq!(min,-18);
    assert_eq!(mean,12.5);
    println!("maxminmean passed");

}

fn celsius_to_fahrenheit(c:f64) -> f64{
    (1.8 * c) + 32.0
}

// this function will return i32
fn square(x:i32) -> (i32, i32){
    println!("squaring {}", x);
    return (x , x * x);
}
