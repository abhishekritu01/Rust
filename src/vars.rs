

//variables hold permitive data or reference to data
//variables are immutable by default
//Rust is block-scope language 
//mu ---> mutable

pub fn run(){
    let name ="jhon";
    let mut age=27;
    println!("My name is {} and I am {}" ,name ,age);
     age=29;  //cannot assign twice to immutable variable
    println!("My name is {} and I am {}" ,name ,age);


    //Define constant 
    const ID:i32 =001;
    println!("ID:{}",ID);

    //Assign multiple variable
    let(my_name,my_age)=("Abhishek",27);
    println!("{} is {}",my_name,my_age);

}
