fn main() {
    a();
    b();
}

fn a(){
    let a = 10;
    let b = String::from("Ucup");
    println!("{} {}", a,b);
}

fn b(){
    let a = 12;
    let b = String::from("Aslam");
    println!("{}, {}", a, b);
}

#[test]
fn string(){
    let name: &str = " Ucup Aslam ";
    let trim: &str = name.trim();
    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type(){
    let mut name: String = String::from("Ucup Aslam");
    name.push_str(" Gantenk");
    println!("{}", name);
    
    let budi = name.replace("Ucup", "Yusuf");
    println!("{}", budi);
}


#[test]
fn ownership_rules(){
    let a = 10;
    println!("{}", a);
    {
        let b = 10;
        println!("{}", b);
    }
}

#[test]
fn data_copy(){
    let a = 10;
    let b = a + 1;

    println!("{}, {}", a,b);
}

#[test]
fn ownership_movement(){
    let name1 = String::from("Ucup");
    let name2 = name1;

    println!("{}", name2);

}

#[test]
fn clone(){
    let name1 = String::from("UCUP");
    let name2 = name1.clone();
    println!("{}, {}", name1, name2);
}

#[test]
fn if_expression(){
    let value = 9;

    if value >= 10 {
        println!("Good!");
    }else if value == 9 {
        println!("Not bad!");
    }else{
        println!("Bad!");
    }
}

#[test]
fn let_statement(){
    let value = 9;
    
    let result = if value >= 9 {
        "Good"
    }else{
        "Bad"
    };
    
    println!("{}", result)
}

#[test]
fn loop_expression(){
    let mut counter = 0;

    loop{
        counter += 1;
        if counter >= 10{
            break
        }else if counter % 2 == 0 {
            continue
        }
        println!("{}", counter);
    }

}

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        if counter > 10 {
            break counter * 2;
        }
        counter += 1 
    };
    println!("{}", result);
}

#[test]
fn loop_label(){
    let mut number = 1;
    
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }
            
            println!("{} X {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break
            }
        }
        number += 1;
    }
}

 #[test]
fn while_loop(){
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0{
            println!("Even : {}", counter);
        }
        counter += 1
    }
}

#[test]
fn arr_iteration(){
    let arr = ["a", "b", "c", "d", "e"];
    let mut index = 0;

    while index < arr.len(){
        println!("Value :{}", arr[index]);
        index += 1
    }
}

#[test]
fn arr_iteration_for(){
    let arr = ["a", "b", "c", "d", "e"];
    for value in arr {
        println!("Value : {}", value);
    }
}

#[test]
fn range_datatype(){
    let arr = ["a", "b", "c", "d", "e"];
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);
    
    for i in 0..5{ //gini juga bis bruhh
        println!("ulala{}", arr[i]);
    }
}

#[test]
fn range_inclusive(){
    let arr = ["a", "b", "c", "d", "e"];
    let range = 0..=4; // ini maksudnya bruhh
    println!("Start: {}", range.start());
    println!("End: {}", range.end());
    
    for i in 0..=4{ //gini juga bis bruhh
        println!("ulala{}", arr[i]);
    }
}
fn say_something(word: &str, name: &str){
    println!("{}, {}", word, name);
}

fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn param_or_arg(){
   say_something("Hey", "Ucup");
    println!("{}", factorial(10));
}

fn print_text(value: String, times: i32){
    if times == 0{
        return;
    }else{
        println!("{}", value);
    }
    print_text(value, times-1);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1{
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_recursive(){
    print_text(String::from("Ucup"), 5);
    println!("{}",factorial_recursive(10));
}

fn print_number(number: i32){
    println!("number {}", number);
}

fn hi(name: String){
    println!("name {}", name);
}

#[test]
fn fn_ownership(){
    let number = 10;
    print_number(number);
    println!("{}", number);
    
    let name = String::from("Ucup");
    hi(name); 
}


fn full_name(first: String, last: String) -> String {
    format!("{} {}", first, last)
}

#[test]
fn r_value_own(){
    let first = String::from("Ucup");
    let last = String::from("Aslam");

    let name = full_name(first, last);

    println!("full {}", name);
    //println!("{}", first);
    //println!("{}", last);
}

// mengembalikan ownership
fn kembalikan(f: String, l:String) -> (String, String, String) {
    let full = format!("{} {}", f, l);

    (f, l, full)
}


#[test]
fn test_kembali_owner(){
    let f = String::from("ucup");
    let l = String::from("aslam");

    let (f, l, full) = kembalikan(f, l);

    println!("{}", f);
    println!("{}", l);
    println!("{}", full);

}

fn fname_references(f: &String, l: &String) -> String {
    format!("{} {}", f, l)
}

#[test]
fn references(){
    let f = String::from("ucup");
    let l = String::from("aslam");

    let full = fname_references(&f, &l);

    println!("{}", full);
     println!("{}", f); //gak diambil sama function bruhhh ownershipnyee
     println!("{}", l);

}

fn change_value(v: &mut String) {
   // v.push_str("Test");
    v.push_str("Test");
}


#[test]
fn test_change_value(){
    let mut v = String::from("ucp");

    // change_value(&v);
    change_value(&mut v);
    println!("{}", v);


}







