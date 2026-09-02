fn main(){
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
}

fn change(string: &mut String){
    string.push_str(", world");
}
