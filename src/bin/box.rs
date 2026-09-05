use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x : T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer{
    data : String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main(){
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let c: CustomSmartPointer = CustomSmartPointer{
        data : String::from("my stuff")
    };

    let d = CustomSmartPointer{
        data : String::from("my other stuff")
    };
    drop(c);
    println!("CustomSmartPointers created");
}
