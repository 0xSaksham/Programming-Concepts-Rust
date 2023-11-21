// Arrow function with paranthesis

fn plus_one(x: i32) -> i32{
    x+1
}

fn main() {
    println!("Hello, world!\n");

    another_function();
    parameter_function(6);
    two_parameter_function(22,'Z');

    // Statements and Expressions
    
    let y = { //Everything inside this is expression.
        let x = 3; 
        x + 10
    };
    println!("The value of y is: {y}\n");


// Using that arrow function
    let x = plus_one(4);
    println!("x:{x}\n");
}

fn another_function(){
    println!("Another Function\n");
}

fn parameter_function(x:i32){
    println!("The value of x is {}\n", x);
}

fn two_parameter_function(value:i32, unit_label:char){
    println!("Value:{value} and Unit:{unit_label}\n");
}






