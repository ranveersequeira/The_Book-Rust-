fn main() {
    // Mutable, Shadowing
    // variable with mut keyword would be mutable
    let un_mutable: usize = 19;
    // un_mutable = 13; // this would be error
    let mut mutable_var: usize = 18;
    mutable_var = 19;

    //There is const which never changed once assigned data type (P.S - Not like JS)
    //we cannot assign any value of const variables at runtime.
    const FIXED_VARIABLE: usize = 100_1000;

    //Shadowing allow you to create new variable using existng name, can be used to change
    // data type of variable
    let num_variable = 10;
    let num_variable = "Ranveer"; //notice we're re-declaring the value instead of assigning new
                                  //value

    //Data Types
    let num1: u32 = 100;
    let num2: u32 = 0xff;

    let opieeessss: u8 = 255; // as u8 contains till 255 anything greator would be re-assign to new
                              // value Ex - 256 would be 0 , 257 would be 1 , Also some warning/error by LSP

    let float_1: f64 = 2.0;
    let float_2: f32 = 3.0;

    let learned_rust: bool = true;

    let character_word: char = 'z';

    //Compound Types
    let tup: (&str, i32) = ("Learn Rust Now", 100);
    let (task, value) = tup;
    let task = tup.0;

    let error_codes: [i32; 3] = [200, 400, 500];
    let not_found = error_codes[1];
    //we can also generate array with fixed value using
    let generate_ten_zeros = [0; 10];
}
