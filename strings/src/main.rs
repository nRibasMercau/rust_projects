fn main() {
    let mut s1 = String::from("hello"); 
    let mut s2 = s1.clone(); 

    println!("{}", s1);
    println!("{}", s2);

    s1 = String::from("Bye");
    println!("{}", s1);
    println!("{}", s2);


}
