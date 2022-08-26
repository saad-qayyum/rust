use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let days = ["first","second","third","fourth","fifth","sixth", "seventh","eighth",
        "ninth","tenth","eleventh","twelfth"];

        let mut vec = Vec::new();    

        if let Ok(lines) = read_lines("lyrics") {
        for line in lines{
            if let Ok(ip) = line {
                vec.push(ip);
            }
        }
    }

    for elem in 0..12 {
        let day = days[elem];
        println!("On the {day} day of christmas, my true love sent to me");
        let  mut j = vec.len()-elem-1;
        while j < vec.len(){
            let mut line = &vec[j];
            j+=1;
            println!("{}",line);
        }
        println!("__________________");
    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}