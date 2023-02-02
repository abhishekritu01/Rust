//condition check result
pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Jhon: what do you want to drink ?");
    } else if age < 21 && check_id {
        println!("Jhon: Sorry you have to leave");
    } else {
        println!("Jhon: I need to see your ID")
    }

    //shorthand
    let is_of_age:bool = if age >= 21 { true } else { false };
    println!("Is Of Age:{}", is_of_age);
}
