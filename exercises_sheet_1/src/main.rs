
// Exercise 1: String Reversal
/*
Write a function that takes a string as input and returns the string reversed. For example,
 if the input is "hello", the output should be "olleh".
*/

// fn --> function definition
// string_reverse --> function name
// s: &str --> function parameter, a string slice reference
// -> String --> return type, the function will return a String

// s.chars() --> converts the string slice into an iterator of characters
// .rev() --> reverses the order of the characters in the iterator
// .collect() --> collects the characters from the iterator into a new String


fn string_reverse(s: &str) -> String {
    s.chars().rev().collect() // last experession automatically returned, ";" is not required
    // same as 
    // return s.chars().rev().collect(); // in this case ";" is requied
}

// Exercise 2: bigger Number
/*
Write a function bigger that takes two i32 and returns the bigger number ( i32 ) without using
another function call and additional variables;
*/
fn bigger(a:i32, b:i32)-> i32 {
    // if a > b { return a; } b //last expression
    if a > b { 
        a                       //last expression
    } else { 
        b                       //last expression   
    }
}

// Exercise 3: Multiplication of Different Types
/*
Write a function multiply that takes an i32, a f32 and a f64 and returns the multiplication of the
three of them as a f64 value
*/
fn multiply(a: i32, b: f32, c: f64, option: u8) -> f64 {
    match option { // as the swithch statement in other languages
        0 => {
            // Option 1: Convert everything to f64 (best for precision)
            (a as f64) * (b as f64) * c
        }
        1 => {
            // Option 2: Convert everything to f32 (less precision, smaller memory)
            ((a as f32) * b * (c as f32)) as f64
        }
        _ => {
            // Option 3: Convert everything to i32 (loses decimals!)
            (a * (b as i32) * (c as i32)) as f64
        }
    }
}

// Exercise 4: Energy Equivalent of Mass
/*
Write a function e_equals_mc_squared that takes as input a f32 representing the mass, and that
uses a globally-defined constant containing the value of the speed of light in a vacuum (expressed in
m/s). The function outputs the energy equivalent to the mass input;
*/

// Exercise 5: Max and Min in a Vector
/*
Given a vector of i32 , create a function max_min that returns the maximum and the minimum value
inside that vector;
*/

// Exercise 6: Lord Farquaad
/*
Write a function lord_farquaad that takes a String and outputs another String in which every
character 'e' is substituted by the character '💥';
*/

// Exercise 7: Furniture Price Lookup
/*
In the main function initialize a HashMap<String, f32> called furniture that stores the pair
String as key and f32 as value, where the String is the name of the furniture and the f32 is its
price. Then write a function that borrows the HashMap , takes a furniture: String as input and
returns the corresponding f32 . If there is no such furniture in the HashMap , return -1.0 ;
*/


// Exercise 8: String Append
/*
Write a function append that takes a String , appends the word "foobar" to it and returns it.
Then write a main function in which you:
- Declare a String initialized with some text.;
- Pass the String to the function append ;
- Print the original String and the one returned by append ;
(do it in this order!)
*/

//Exercise 9: Armstrong Number
/*
An Armstrong number is a number that is the sum of its own digits each raised to the power of the
number of digits.
For example:
9 is an Armstrong number, because 9 = 91 = 9
10 is not an Armstrong number, because 10 ≠ 12 + 02 = 1
153 is an Armstrong number, because: 153 = 13 + 53 + 33 = 1 + 125 + 27 = 153
154 is not an Armstrong number, because: 154 ≠ 13 + 53 + 43 = 1 + 125 + 64 = 190
Write the function is_armstrong that determines whether a number is an Armstrong number;
*/

//Exercise 10: Transpose of a Matrix
/*
Write a function that takes a 2x2 i32 "matrix" (2x2 tuple) as input, transposes and returns it.
*/

fn main() {
    //ES 1: String Reversal
    println!("Exercise 1: String Reversal");
    let test: &str = "hello";
    let reversed: String = string_reverse(test);
    println!("Original: {}", test);
    println!("Reversed: {}", reversed);

    // ES 2: Bigger Number
    println!("\nExercise 2: Bigger Number");
    let num1: i32 = 10;
    let num2: i32 = 20;
    let bigger_num: i32 = bigger(num1, num2);
    println!("Bigger number between {} and {} is {}", num1, num2, bigger_num);

    // ES 3: Multiplication of Different Types
    println!("\nExercise 3: Multiplication of Different Types");
    let a: i32 = 5;
    let b: f32 = 2.5;
    let c: f64 = 1.0;
    let result: f64 = multiply(a, b, c, 0);
    println!("Multiplication between {} and {} and {} is: {}", a, b, c, result);
}
