fn main(){
    let v = vec![1,2,3];

    let v_iter = v.iter();

    // assert_eq!(v_iter.next(),Some(&1));
    // assert_eq!(v_iter.next(),Some(&2));
    // assert_eq!(v_iter.next(),Some(&3));
    // assert_eq!(v_iter.next(),None);

    let total : i32 = v_iter.sum();
    println!("{total}");
    assert_eq!(total,6);

    let v1: Vec<i32> = vec![1,2,3];
    let v2 : Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2,[2,3,4]);
}
