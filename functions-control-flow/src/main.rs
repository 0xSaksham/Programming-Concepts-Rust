fn main() {
    println!("Hello, world!\n");

    another_function();
    parameter_function(6);
    two_parameter_function(22,'Z');
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