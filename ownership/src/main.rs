fn main() {
    //Ownership Rules
    //
    //1. Each value in rust have variable that is its owner
    //2. There can be only one owner at a time
    //3. When owner goes out of scope, value would be dropped.

    {
        let name: &str = "Ranveer";
        //You can do stuff with name here.
    }
    // Variable and value inside this block {} would be dropped.

    //This copy would work because rust allow simple types stored on stack to copy.
    let x: u32 = 1000;
    let y: u32 = x;

    //String in rust are stored on heap(dynamic) so when copy it would just pass ownership to next
    //varibale, try uncommenting the print statement to understand better
    let name: String = String::from("Ranveer");
    let my_name: String = name;
    // println!("My name is {}", name);

    //When you need to copy dynamic stored values, Ex - string. You can use clone method,
    let new_name: String = String::from("Sequeira");
    let copy_name: String = new_name.clone();
    println!("My copied name is {}", new_name);
}
