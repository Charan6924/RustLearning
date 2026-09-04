use adder::add_two;

#[test]
fn it_adds_two(){
    let result = add_two(10);
    assert_eq!(12,result)
}
