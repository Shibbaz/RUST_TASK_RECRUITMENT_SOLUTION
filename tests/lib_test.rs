fn build_message(name: String) -> String{
    let message = "You are welcome ".to_owned() + &name;
    return message;
}

#[test]
fn test_build_message(){
    let name = String::from("Fikayo");

    assert_eq!(build_message(name), "You are welcome Fikayo")
}