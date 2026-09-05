enum List{
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons,Nil};
use std::rc::Rc;

fn main(){
    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b: List = Cons(3, Rc::clone(&a));
    let c: List = Cons(4, Rc::clone(&a));
}

