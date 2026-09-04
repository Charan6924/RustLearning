struct Point<T>{
    x : T,
    y : T
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}
fn main(){
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let wont_work = Point{x:0,y:1};
}

fn largest<T:PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for i in list{
        if i > largest{
            largest = i;
        }
    }

    largest
}
