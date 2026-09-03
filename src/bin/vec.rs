
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
fn main(){
    let mut v: Vec<i32> = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("{third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("{third}"),
        _ => println!("idk")
    }

    let row: Vec<_> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.5),
        SpreadSheetCell::Text(String::from("HELLO"))
    ];


}
