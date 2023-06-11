#[allow(unused_variables)]
fn main() {
    // i32 refers to a 32 bit integer when declaring variables in rust 
    let x: i32 = 10;
    let extra_variable: i32 = 10;
    // braces here specify a certain scope where variables within the scope will only be accessible within the scope
    {
        let y: i32 = 5;
        // braces are responsible for transferring values from the variables
        println!(" The value of x is {} and value of y is {}", x, y);
    };
    assert_eq!(x, extra_variable);
    // sub function list 
    // defined_name();
    // check_variable_immutability();
    // intro_tuples();
    // calling_functions();
    // basic_iteration();
    // vector_intro();
    // let mut move_zeroes_array: Vec<i32> = vec![0,1,0,3,12];
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    let mut chars: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut chars)
}

// note even functions should be declared using the snake case system
fn defined_name(){
    let x: &str = "hello";
    println!("The variable in the second function is {}", x);
}

fn check_variable_immutability(){
    let mut x: i32 = 1;
    x = 7; // mutable variable is reinitialized
    // redeclaring and reinitializing the value here is called shadowing in rust
    let mut x: i32 = x;
    x += 7;
    print!("This is the new value of x: {}", x);
    // using underscore allows unused variables in rust
    let _new_variable: i32 = 10;
}

// introduction to tuples data structures in rust 
// notes: tuples are compound data structures that allow one to group together multiple values of different types into single entity.
fn intro_tuples(){
    // declaration syntax of tuple is used using braces
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    // declaring tuples
    let (a, b);
    (a, ..) = (3, 4);
    println!("The value of a should be 3: {}", a);
    [.., b] = [1, 2];
    assert_eq!([a, b], [3, 2]); 

    println!("The above code did not fuck up")

}

// calling function within a function

fn calling_functions(){
    let (tuple_one, tuple_two) = (1, 3);
    let result = sub_called_function(tuple_one, tuple_two);
    assert_eq!(result, 4);
    println!("Checking the result now: {}", result);
}
// remmeber when return values in rust ommit the semi color in order to return the value based on teh exact specified type
fn sub_called_function(one: i32, two:i32) -> i32{
     return one + two
}


// iterating through an array... remember in rust arrays are fixed sized hence u cannot change the length or update the values once declared
fn basic_iteration(){
    // here array is declared based on its type and length
    let array: [i32; 4] = [1,2,3,4];
    for index in 0..array.len(){
        println!("Print out the index: {}, and the value here: {}", index, array[index]);

    }
}

// interchangable and array equivalent to other languages would be vectors in rust
fn vector_intro(){
    let new_vector: Vec<i32> = vec![1,3,3,43,43,34];
    let mut empty_vector: Vec<i32> = Vec::new();
    let mut index = 0;
    // while loop structure in rust
    while index < new_vector.len(){
        println!("print out the values here:{}", new_vector[index]);
        if new_vector[index] > 5{
            empty_vector.push(new_vector[index])
        }
        // note: in rust there is no post fix incremeent arguement system
        index = index + 1;
    }
    // :? is the format specifier used to display rust vectors in a debug format
    println!("{:?}",empty_vector);
}

// the match expression is the equivalent of switch case in rust
fn match_expression(){

}

// basic leetcode problem using rust
// remember when the vector here in rust is passed using &mut, its not the copy but a reference to the o
//riginal array from the function above hnce it needs to be dereferenced in order to use it
fn move_zeroes(nums: &mut Vec<i32>){
    println!("{:?}", nums);
    let mut non_zero_vector: Vec<i32> = Vec::new();
    let mut zero_count: usize = 0;
    // once u iterate over nums directly changes the ownership therefore u cannot iterate over nums directly in this pattern
    // there nums which is a borrowed value needs to be iterated using iter_mut in order to prevent ownership issues
    for number in nums.iter_mut(){
        let number: i32 = *number; // star before number variable here is for dereferencing number
        if number != 0 {
            non_zero_vector.push(number)
        }else{
            zero_count = zero_count + 1
        }
    };
    let mut index: usize = 0;
    while index < zero_count{
        non_zero_vector.push(0);
        index = index + 1;
    }
    for (index, num_value) in non_zero_vector.iter().enumerate(){
        nums[index] = *num_value;
    }
    println!("{:?}", nums);
    nums;
    
    

    // below are the ways that violate the borrowed rules of rust but are applicable for general iteration
    // for index in 0..non_zero_vector.len(){
    //     nums[index] = non_zero_vector[index]
    // }
    // recopying the array into nums
    // remember clearing and extending from borrowed vectors will violate the rules of rust
    // nums.clear();
    // nums.extend(non_zero_vector);

    }


    // reverse string rust conversion by receiving s mut vector char
    // in rust there is no need to return variable if the original passed on value is changing... returning becomes optional
    fn reverse_string(s:&mut Vec<char>){
        let mut left_index: usize = 0;
        let mut right_index: usize = s.len() - 1;
        while left_index < right_index{
            let local_char:char = s[left_index];
            s[left_index] = s[right_index];
            s[right_index] = local_char;
            left_index = left_index +  1;
            right_index = right_index - 1;
        }
    }

