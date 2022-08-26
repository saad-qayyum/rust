fn main() {

    //takes ownership
    let mut str = String::from("Ownership will be passed to function");
    takes_ownership(str);
    // println!("{}",str); //this will not compile due to ownership shifted.
    {
        let  str1 = String::from("Ownership transfered to other variable");
        let  str2 = str1;
        // println!("{}",str1); //not valid
        println!("{}",str2); //valid
        
    }
    // println!("{}",str1); //not available here str1
    //copies
    let x = 5;
    let y = x;
    
    //does not takes ownership
    let mut str = String::from("Will not takes its ownership");
    does_not_takes_ownership(str.clone());
    println!("{}",str);


    //gives ownership
    let s1 = gives_ownership();
    println!("{}",s1);
    //takes and then returns

    let s2 = takes_and_gives_back_ownership(s1);
    println!("{}",s2);
    // println!("{}",s1); //ownership of s1 is taken
    

}



fn takes_ownership(str:String){

}

fn takes_and_gives_back_ownership(str:String)->String{
    str
}

fn gives_ownership()->String{
    String::from("Giving owner ship to caller")
}

fn does_not_takes_ownership(str:String){

}