fn main() {

    let s1 = String::from("hello");

    let len = will_not_take_ownership(&s1); //reference passed, so ownership will not be taken

    println!("The length of '{}' is {}.", s1, len);

    //change s in other function
    let mut s = String::from("hello");

    // change(&s); //will not work need to make s mutable
    change(&mut s);


    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; //cannot be two refrences of a mutable string

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3); //some refrences are mutable and some are immutable so its a problem


    //this works somehow
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{} ", r3,);
    // println!("{} {} {}", r3,r1,r2); //cannot do this still

}

fn will_not_take_ownership(str:&String)->usize{
    str.len()
}

// fn change(str: &String){
//     str.push_str(", world");
// }

fn change(str: &mut String){
    str.push_str(", world");
}

//dangling refrence
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}