fn main() {
    a();
    b();
}

fn a() {
    let a = 10;
    let b = String::from("Ucup");
    println!("{} {}", a, b);
}

fn b() {
    let a = 12;
    let b = String::from("Aslam");
    println!("{}, {}", a, b);
}

#[test]
fn string() {
    let name: &str = " Ucup Aslam ";
    let trim: &str = name.trim();
    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Ucup Aslam");
    name.push_str(" Gantenk");
    println!("{}", name);

    let budi = name.replace("Ucup", "Yusuf");
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    let a = 10;
    println!("{}", a);
    {
        let b = 10;
        println!("{}", b);
    }
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a + 1;

    println!("{}, {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Ucup");
    let name2 = name1;

    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("UCUP");
    let name2 = name1.clone();
    println!("{}, {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 9;

    if value >= 10 {
        println!("Good!");
    } else if value == 9 {
        println!("Not bad!");
    } else {
        println!("Bad!");
    }
}

#[test]
fn let_statement() {
    let value = 9;

    let result = if value >= 9 { "Good" } else { "Bad" };

    println!("{}", result)
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter >= 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("{}", counter);
    }
}

#[test]
fn loop_return_value() {
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
fn loop_label() {
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
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Even : {}", counter);
        }
        counter += 1
    }
}

#[test]
fn arr_iteration() {
    let arr = ["a", "b", "c", "d", "e"];
    let mut index = 0;

    while index < arr.len() {
        println!("Value :{}", arr[index]);
        index += 1
    }
}

#[test]
fn arr_iteration_for() {
    let arr = ["a", "b", "c", "d", "e"];
    for value in arr {
        println!("Value : {}", value);
    }
}

#[test]
fn range_datatype() {
    let arr = ["a", "b", "c", "d", "e"];
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in 0..5 {
        //gini juga bis bruhh
        println!("ulala{}", arr[i]);
    }
}

#[test]
fn range_inclusive() {
    let arr = ["a", "b", "c", "d", "e"];
    let range = 0..=4; // ini maksudnya bruhh
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in 0..=4 {
        //gini juga bis bruhh
        println!("ulala{}", arr[i]);
    }
}
fn say_something(word: &str, name: &str) {
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
fn param_or_arg() {
    say_something("Hey", "Ucup");
    println!("{}", factorial(10));
}

fn print_text(value: String, times: i32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }
    print_text(value, times - 1);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_recursive() {
    print_text(String::from("Ucup"), 5);
    println!("{}", factorial_recursive(10));
}

fn print_number(number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("name {}", name);
}

#[test]
fn fn_ownership() {
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
fn r_value_own() {
    let first = String::from("Ucup");
    let last = String::from("Aslam");

    let name = full_name(first, last);

    println!("full {}", name);
    //println!("{}", first);
    //println!("{}", last);
}

// mengembalikan ownership
fn kembalikan(f: String, l: String) -> (String, String, String) {
    let full = format!("{} {}", f, l);

    (f, l, full)
}

#[test]
fn test_kembali_owner() {
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
fn references() {
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
fn test_change_value() {
    let mut v = String::from("ucp");

    let value_borrow = &mut v;

    change_value(value_borrow);
    println!("{}", value_borrow);
}

#[test]
fn slice_reference() {
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &arr[..];
    println!("{:?}", slice1);
    let slice2: &[i32] = &arr[0..5];
    println!("{:?}", slice2);
    let slice3: &[i32] = &arr[5..];
    println!("{:?}", slice3);
}

#[test]

fn str_slice() {
    let name = String::from("Ucup Aslam");
    let first = &name[..4];
    println!("{}", first);
}

// struct
struct Person {
    name: String,
    age: u32,
    alive: bool,
}

#[test]
fn test_struct() {
    let name = String::from("Aslam");

    let person = Person {
        name,
        age: 10,
        alive: true,
    };
    let person2 = Person {
        name: String::from("Yusuf"),
        ..person
    };

    print_person(&person);
    print_person(&person2);
}

fn print_person(person: &Person) {
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Alive: {}", if person.alive { "Alive" } else { "Dead" });
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn tuple_struct() {
    let p = GeoPoint(10.0, 20.0);
    println!("p: {}, {}", p.0, p.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing: Nothing = Nothing {};
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.name);
    }
}

#[test]
fn test_method() {
    let person = Person {
        name: String::from("Ucup"),
        age: 17,
        alive: true,
    };

    person.say_hello("ulala")
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(20.3, 68.4);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let _level: Level = Level::Premium;
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount
                );
            }
            Payment::EWallet(wallet, number) => {
                println!(
                    "Paying with ewallet {} {} amount {}",
                    wallet, number, amount
                );
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("35235234523"));
    _payment1.pay(234)
}

#[test]
fn test_enum_matching() {
    let level: Level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

#[test]
fn test_match_value() {
    let name = "Ucup";

    match name {
        "Ucup" | "Radit" => {
            println!("Hello bos {}", name);
        }
        other => {
            println!("siape luh {}", other);
        }
    }
}

#[test]
fn test_range_pattern() {
    let v = 100;
    match v {
        35..=50 => {
            println!("ullala")
        }
        51..=70 => {
            println!("not bad laa")
        }
        71..=100 => {
            println!("NAISEEE")
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}

#[test]
fn test_struct_pattern() {
    let p = GeoPoint::new(0.0, 1.0);
    match p {
        GeoPoint(long, 0.0) => {
            println!("long : {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long : {} lat: {}", long, lat);
        }
    }

    let person = Person {
        name: String::from("Ucup"),
        age: 17,
        alive: true,
    };

    match person {
        Person { name, .. } => {
            println!("Name: {}", name);
        }
    }
}

#[test]
fn test_ignoring() {
    let p = GeoPoint::new(0.0, 1.0);
    match p {
        GeoPoint(long, _) => {
            println!("long : {}", long);
        }
        GeoPoint(_, lat) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long : {} lat: {}", long, lat);
        }
    }

    let v = 100;
    match v {
        35..=50 => {
            println!("ullala")
        }
        51..=70 => {
            println!("not bad laa")
        }
        71..=100 => {
            println!("NAISEEE")
        }
        _ => {
            println!("Invalid value ");
        }
    }
}

#[test]
fn test_match_expression() {
    let v = 25;
    let r = match v {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        _ => "Other",
    };
    println!("{}", r);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

type Pelanggan = Customer;

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("123234123412"),
        name: String::from("Ucup"),
        age: 17,
    };
    println!(
        "id: {}, name: {}, age: {}",
        customer.id, customer.name, customer.age
    );
}

mod first;
mod model;
mod second;
mod third;

use model::*;
// use model::{User, say_hello};
use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    say_hello("Ucup");
    say_hello_second("Yusuf");
    first::second::third::say_hello("Yusuf");
}

#[test]
fn test_module() {
    let u = model::User {
        name: String::from("Ucup"),
        age: 17,
        alive: true,
    };
    u.say_hello("Yusuf");
}

trait CanSayHello {
    fn hellos(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}

fn say_hello_trait(val: &impl CanSayHello) {
    println!("{}", val.say_hello())
}

trait CanSayGoodbye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodbye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn hello_goodbye(val: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", val.say_hello());
    println!("{}", val.good_bye());
}

#[test]
fn test_trait() {
    let p = Person {
        name: String::from("Ucup"),
        age: 10,
        alive: true,
    };

    /*say_hello_trait(&p);
    hello_goodbye(&p);
    let result = p.say_hello_to("Budi");
    println!("{}", result);
    let result1 = p.hellos();
    println!("{}", result1);
    println!("{}", p.good_bye_to("Sarah"));*/
    CanSayHello::say_hello(&p);
    Person::say_hello(&p, "Budi")
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodbye {
    SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let p = create_person(String::from("Ucup"));
    println!("{}", p.good_bye());
    println!("{}", p.good_bye_to("Sarah"));
}

trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self) {
        print!("{} {}", self.say_hello(), self.good_bye());
    }
}

// struct SimpleMan {
//     name: String
// }
// impl CanSay for SimpleMan{

// }

struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struct() {
    let int = Point { x: 10, y: 20 };
    println!("x: {} y: {}", int.x, int.y);
    let f: Point<f64> = Point::<f64> { x: 10.2, y: 20.32 };
    println!("x: {} y: {}", f.x, f.y);
}

enum Value<T> {
    NONE,
    SOME(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::SOME(10);
    match value {
        Value::NONE => println!("NONE"),
        Value::SOME(value) => println!("Value: {}", value),
    }
}

struct Hi<T: CanSayGoodbye> {
    value: T,
}

#[test]
fn test_generic_struct_with_trait() {
    let hi = Hi {
        value: SimplePerson {
            name: String::from("Ucup"),
        },
    };
    println!("{}", hi.value.good_bye());
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min(10, 20);
    println!("{}", result);
    let result = min(1.30, 2.20);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let p = Point { x: 10, y: 20 };
    println!("x: {}", p.get_x());
    println!("y: {}", p.get_y());
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        &self.x
    }
}

use core::ops::Add;
use core::option::Option;

struct Apple {
    quantity: u32,
}

impl Add for Apple {
    type Output = Apple;
    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 20 };
    let apple3 = apple1 + apple2;
    println!("apple3 quantity {}", apple3.quantity);
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(value) => Some(value * 2),
    }
}

#[test]
fn test_option() {
    let result = double(None);
    println!("{:?}", result);
    let result = double(Some(10));
    println!("{:?}", result);
}

use core::cmp::PartialEq;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result as ResultFormatter};

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}
#[test]
fn test_comparing() {
    let app1 = Apple { quantity: 10 };
    let app2 = Apple { quantity: 20 };
    println!("app1 == app2 {}", app1 == app2);
    println!("app1 > app2 {}", app1 > app2);
    println!("app1 < app2 {}", app1 < app2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Raditya Ucup Aslam");
    println!("{}", s.to_lowercase());
    println!("{}", s.to_uppercase());
    println!("{}", s.len());
    println!("{}", s.replace("Ucup", "Yusuf"));
    println!("{}", s.contains("Aslam"));
    println!("{}", s.starts_with("Raditya"));
    println!("{}", s.ends_with("Aslam"));
    println!("{}", s.trim());
    println!("{}", &s[0..7]);
    println!("{:?}", s.get(0..7));
}

struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> ResultFormatter {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category {
        id: String::from("ADSF12314E"),
        name: String::from("Laptop"),
    };
    println!("{:?}", category)
}

#[test]
fn test_closure() {
    let sum = |val1: i32, val2: i32| -> i32 { val1 + val2 };
    println!("{}", sum(10, 20));
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("Result : {}", result);
}

#[test]
fn test_closure_as_parameter() {
    let name = String::from("Raditya Yusuf Aslam");
    print_with_filter(name, |value: String| -> String { value.to_uppercase() });
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_closure_as_closure() {
    let name = String::from("Raditya Yusuf Aslam");
    print_with_filter(name, to_uppercase);
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("increment");
    };
    increment();
    increment();
    increment();
    println!("{}", counter);
}

struct Counter {
    counter: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}
#[test]
fn test_closure_scope_alt() {
    let mut counter = Counter { counter: 0 };
    counter.increment();
    counter.increment();
    counter.increment();
    println!("{}", counter.counter);
}

#[test]
fn test_vector() {
    let mut names = Vec::<String>::new();
    names.push(String::from("Raditya"));
    names.push(String::from("Yusuf"));
    names.push(String::from("Aslam"));
    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);
}
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Raditya"));
    names.push_back(String::from("Yusuf"));
    names.push_front(String::from("Aslam"));
    println!("{:?}", names);
    println!("{:?}", names[0]);
}
#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_back(String::from("Raditya"));
    names.push_back(String::from("Yusuf"));
    names.push_front(String::from("Aslam"));
    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);
}

#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert(String::from("name"), String::from("Yusuf"));
    map.insert(String::from("age"), String::from("26"));
    let name = map.get("name");
    let age = map.get("age");
    for entry in map {
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn test_btree_map() {
    let mut map = BTreeMap::new();
    map.insert(String::from("name"), String::from("Yusuf"));
    map.insert(String::from("age"), String::from("26"));
    let name = map.get("name");
    let age = map.get("age");
    for entry in map {
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn test_hash_set() {
    let mut set = HashSet::new();
    set.insert(String::from("Ucup"));
    set.insert(String::from("Ucup"));
    set.insert(String::from("Aslam"));
    set.insert(String::from("Aslam"));
    set.insert(String::from("Radit"));
    set.insert(String::from("Radit"));
    for value in set {
        println!("{}", value);
    }
}

#[test]
fn test_btree_set() {
    let mut set = BTreeSet::new();
    set.insert(String::from("Ucup"));
    set.insert(String::from("Ucup"));
    set.insert(String::from("Aslam"));
    set.insert(String::from("Aslam"));
    set.insert(String::from("Radit"));
    set.insert(String::from("Radit"));
    for value in set {
        println!("{}", value);
    }
}

#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5];
    let mut iter = array.iter();
    while let Some(value) = iter.next() {
        println!("{}", value);
    }
    for value in iter {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", v);

    let sum: i32 = v.iter().sum();
    println!("{}", sum);

    let count = v.iter().count();
    println!("{}", count);

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = v.iter().filter(|x| *x % 2 == 1).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        Some(host) => {
            println!("Connecting to database {}", host);
        }
        None => {
            panic!("No database host provided");
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
    connect_database(None);
}

use std::rc::Rc;
// Menyertakan Result dari pustaka standar
use std::result::{self, Result};

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No cache host provided".to_string()),
    }
}

#[test]
fn test_recoverable_error() {
    let cache = connect_cache(Some("localhost".to_string()));
    match cache {
        Ok(host) => {
            println!("Success connecting to cache {:?}", host);
        }
        Err(error) => {
            println!("Error connecting to cache {}", error);
        }
    }
}
fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No email host provided".to_string()),
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    // let cache_result = connect_cache(host.clone());
    // match cache_result {
    //     Ok(_) => {},
    //     Err(error) => {
    //         Err(error)
    //     }
    // }
    // let connect_email = connect_email(host.clone());
    // match connect_email {
    //     Ok(_) => {},
    //     Err(error) => {
    //         Err(error)
    //     }
    // }
    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Success".to_string())
}

#[test]
fn test_application_error() {
    // let result = connect_application(Some("localhost".to_string()));
    let result = connect_application(None);
    match result {
        Ok(host) => println!("Success connecting with message: {}", host),
        Err(error) => println!("Error connecting with message: {}", error),
    }
}

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 5;
        // r = &x;
    }
    r = &30;
    println!("{}", r);
}

fn longest<'a>(value: &'a str, value2: &'a str) -> &'a str {
    if value.len() > value2.len() {
        value
    } else {
        value2
    }
}

#[test]
fn test_lifetime_annotation() {
    let val1 = "Ucup";
    let val2 = "Aslam";
    let result = longest(val1, val2);
    println!("{}", result);
}

#[test]
fn test_lifetime_annotation_dangling_ref() {
    let str1 = String::from("ucup");
    let result;
    let str2 = String::from("aslam");
    {
        result = longest(str1.as_str(), str2.as_str());
    }
    println!("{}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    username: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student = Student {
        name: "Ucup",
        username: "ucuppppp",
    };
    let student2 = Student {
        name: "asdfp",
        username: "ucuasdfppppp",
    };

    println!("{}", student.name);
    println!("{}", longest_student_name(&student, &student2));
    println!("{}", student.longest_name(&student2));
}

struct Teacher<'a, ID>
where
    ID: Ord,
{
    id: ID,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_generic() {
    let teacher: Teacher<i32> = Teacher {
        id: 10,
        name: "Ucup",
    };
    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_derive() {
    let com = Company {
        name: "Cjeux".to_string(),
        location: "Indonesia".to_string(),
        website: "cjeux.com".to_string(),
    };
    let com2 = Company {
        name: "Tekopedia".to_string(),
        location: "Indonesia".to_string(),
        website: "tekopedia.com".to_string(),
    };

    println!("{:?}", com);
    let result = com == com2;
    println!("{}", result);
    let result = com2 > com;
    println!("{}", result);
}

#[test]
fn test_box() {
    let val = Box::new(10);
    println!("{}", val);
    display_number(*val);
    display_number_ref(&val);
}

fn display_number(val: i32) {
    println!("{}", val);
}
fn display_number_ref(val: &i32) {
    println!("{}", val);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End),
        )),
    );
    print_category(&category);
}

fn print_category(category: &ProductCategory) {
    println!("{:?}", category);
}

#[test]
fn test_dereference() {
    let val1 = Box::new(10);
    let val2 = Box::new(20);
    let result = *val1 + *val2;
    println!("{}", result);
}

struct MyValue<T> {
    value: T,
}

use std::ops::Deref;

impl<T> Deref for MyValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref_struct() {
    let val = MyValue { value: 10 };
    let realval = *val;
    println!("{}", realval);
}

fn say_hello_ref(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_deref_ref() {
    let name = MyValue {
        value: "Ucup".to_string(),
    };
    say_hello_ref(&name);
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Droping Book: {}", self.title);
    }
}

#[test]
fn test_drop() {
    let book = Book {
        title: "Pemrograman Rust".to_string(),
    };
    println!("{}", book.title);
}

enum Brand {
    Of(String, Rc<Brand>),
    End,
}

#[test]
fn test_multiple_ownership() {
    let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    let laptop = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    {
        let smartphone = Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple));
    }
    println!("Apple reference count: {}", Rc::strong_count(&apple));
    // let apple = ProductCategory::Of("Apple".to_string(), Box::new(ProductCategory::End));
    // let laptop = ProductCategory::Of("Laptop".to_string(), Box::new(apple));
    // let smartphone = ProductCategory::Of("Smartphone".to_string(), Box::new(apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Ucup".to_string()),
        active: RefCell::new(true),
    };
    {
        let mut result = seller.name.borrow_mut();
        *result = "Yusuf".to_string();
    }

    println!("{:?}", seller);
}

static APPLICATION: &str = "Application";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}

static mut COUNTER: i32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_unsafe() {
    unsafe {
        increment();
        COUNTER += 1;
        println!("{}", COUNTER);
    }
}

macro_rules! hi {
    () => {
        println!("Hi!");
    };
    ($name: expr) => {
        println!("Hi {}!", $name);
    };
}

#[test]
fn test_macro() {
    hi!();
    hi!("Eko");
    hi! {
        "Ekooo"
    }

    let name = "Eko";
    hi!(name);
}

macro_rules! iterate {
    ($array:expr) => {
        for i in $array {
            println!("{}", i);
        }
    };
    ($($item:expr), *) => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_macro_iterate() {
    iterate!([1, 2, 3, 4, 5, 6, 7, 7, 8]);
    iterate!(1, 2, 3, 4, 5, 6, 7, 7, 8);
}

use std::{
    ops::Sub,
    time::{Duration, Instant},
};

#[test]
fn test_std_time() {
    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(14500);

    let dur3 = dur1.checked_sub(dur2);
    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();
    std::thread::sleep(Duration::from_millis(200));

    println!("Elapsed time: {}", now.elapsed().as_micros());
}

extern crate chrono;

use chrono::NaiveDate;

#[test]
fn test_chrono() {
    let utc_now = chrono::Utc::now();
    println!("{}", utc_now.format("%H %Y %b %d"));

    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Z %H %Y %b %d"));

    let date1 = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Sun);
    let unwrapped_date = date1.unwrap();
    println!("{}", unwrapped_date.format("Day of year is : %j"));

    unwrapped_date
        .iter_days()
        .take(4)
        .for_each(|d| println!("{}", d.format("%j")));

    let date2 = NaiveDate::from_yo_opt(2024, 366);
    println!("{}", date2.unwrap().format("%A %B %d"));

    let birthday = NaiveDate::parse_from_str("2024||07||03", "%Y||%m||%d");
    //println!("{}", birthday.err().unwrap());
    match birthday {
        Ok(date) => println!("{}", date.format("%Y-%m-%d")),
        Err(err) => println!("{}", err),
    }
}
