
pub fn run(){
    println!("print hello from print.rs file");
    
    // basic formatting
    println!("{} is from {}", "Brad","Mass");
    
    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}","brad","mass","code");
    

    //named argument
    println!("{name} likes to play {activity}",name="Abhishek",activity="Cricket");

    //placeholder traits
    println!("Binary:{:b}  Hex:{:x} octal:{:o}",10,10,10);

    //placeholder for debug trait---------***
    println!("{:?}",(12,true,"hello"));

    //basic maths
    println!("10 + 10 ={}",10+10);
}
