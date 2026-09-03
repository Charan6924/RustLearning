fn main(){
    let mut s1: String = String::from("hi");
    let s2 : String = String::from(", there");

    s1.push_str(&s2);
    println!("{},{}",s1,s2)
}
