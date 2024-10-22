// fn main(){
//    let num:i8 = 65;
//    let is_val:bool = true;
//    let greet = String::from("Hello");
//    let char = greet.chars().nth(0);
//    match char {
//       Some(x) => println!("The value of x is: {}",x),
//       None => println!("There is no value"),
//    };
//    println!("The value of num is: {}",num);
//    println!("The value of is_val is: {}",is_val);
//    println!("The value of greet is: {}",greet);
// }


// fn main() {
//     let val:bool = true;
//     if val {
//         println!("The value of val is: {}",val);
//     }else {
//         println!("The value of val is: {}",val);
//     }
// }

// fn main(){
//     for i in 0..10 {
//         println!("The value of i is: {}",i);
//     }
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("The value of s2 is: {}",s2);
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = &s1;
//     println!("The value of s2 is: {}",s2);
//     println!("The value of s1 is: {}",s1);
// }

// fn main(){
//     let mut s1 = String::from("hello");
//     update(&mut s1);
   
//     // let s2 = &mut s1;
//     // s2.push_str("do");
//     let s3 = &mut s1;
//     println!("The value of s1 is: {}",s1);
//     // println!("The value of s2 is: {}",s2);
//     println!("The value of s3 is: {}",s3);
// }

// fn update(s1:&mut String){
//     s1.push_str(" world");
// }

// use std::fmt::Debug;

// struct Rect {
//     width:u16,
//     height:u16
// }

// impl Debug for Rect {
//     fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Rect {{ width: {}, height: {} }}", self.width, self.height)
//     }
// }

// impl Rect {
//     fn area(&self) -> u16 {
//         self.width * self.height
//     }
// }

// fn main(){
//     let area1 = Rect{
//         width:10,
//         height:20
//     };
//     println!("The value of user1 is: {:?}",area1);
//     println!("The area of user1 is: {}",area1.area());
// }


// enum Direction {
//     East,
//     West,
//     North,
//     South
// }

// fn main(){
//     let dir = Direction::East;
//     match dir {
//         Direction::East => println!("The value of dir is: East"),
//         Direction::West => println!("The value of dir is: West"),
//         Direction::North => println!("The value of dir is: North"),
//         Direction::South => println!("The value of dir is: South"),
//     }
// }

// struct Point <T> {
//     x:T,
//     y:T
// }

// fn main(){
//     let point = Point{x:10,y:20};
//     println!("The value of point is: {} and {}",point.x,point.y);
// }

// use std::fs;

// fn main(){
//     let file = fs::read_to_string("example.txt");
//     match file {
//         Ok(content) => println!("The value of x is: {}",content),
//         Err(error) => println!("The value of x is: {}",error),
//     }
// }


// fn return_first_char(s:&String) -> Option<char> {
//     return s.chars().nth(0);
// }

// fn main(){
//     let s = String::from("rakesh");
//     match return_first_char(&s) {
//         Some(x) => println!("The value of x is: {}",x),
//         None => println!("There is no value"),
//     }
// }

// use std::fs;

// fn main(){
//     let file = fs::read_to_string("example.txt");
//     match file {
//         Ok(content) => println!("The value of x is: {}",content),
//         Err(error) => println!("The value of x is: {}",error),
//     }
// }


// enum Opt<T, E> {
//     Ok(T),
//     Error(E),
// }

// fn func<T>(x: T) -> Opt<T, &'static str>
// where
//     T: PartialEq<i32>,
// {
//     if x == 1 {
//         return Opt::Ok(x);
//     }
//     return Opt::Error("Error");
// }

// fn main() {
//     let x = func(2);
//     match x {
//         Opt::Ok(x) => println!("The value of x is: {}", x),
//         Opt::Error(x) => println!("Error: {}", x),
//     }
// }


// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
// }

// use std::collections::HashMap;


// fn main(){
//     let mut hm = HashMap::new();
//     hm.insert("rakesh", 1);
//     hm.insert("sumit", 2);
//     hm.insert("himanshu", 3);
//     hm.insert("vivek", 4);
//     let name = hm.get("rakesh");
//     match name{
//         Some(x) => println!("The value of x is: {}",x),
//         None => println!("There is no value"),
//     }
//     println!("{:?}",hm);
// }


// fn main(){
//     let a = 5;
//     let b = 6;
//     let r = a * b;
//     println!("The value of r is: {}",r);
// }

// fn main(){
//     let v = vec![1,2,3];
//     let v_iter = v.iter();
//     let total = v_iter.sum::<i32>();
//     println!("The value of total is: {}",total);
//     println!("{:?}",v);
// }

// fn main(){
//     let v = vec![1,2,3];
//     let v_iter = v.iter();
//     let v_i = v_iter.map(|x| x+1);
//     for i in v_i {
//         println!("The value of i is: {}",i);
//     }
// }

// fn main(){
//     let v = vec![1,2,3,4,5,6];
//     let v_iter = v.iter();
//     let v_i = v_iter.filter(|x| *x%2==0);
//     let new_v = v_i.collect::<Vec<_>>();
//     println!("{:?}",new_v);
// }

// fn main(){
//     let s = String::from("hello world");
//     let word = &s[0..5];
//     println!("The value of word is: {}",word);
// }

// pub trait Summary {
//     fn summary(&self) -> String;
// }

// struct Tweet {
//     username: String,
//     content: String,
// }

// impl Summary for Tweet {
//     fn summary(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }


// fn main(){
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//     };
//     println!("The value of tweet is: {}",tweet.summary());
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = String::from("king");
//     let str = longest(s1,s2);
//     println!("The value of str is: {}",str);
// }

// fn longest(x: String, y: String) -> String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main(){
//     let s1 = String::from("hello");
//     let ans;

//     {
//         let s2 = String::from("king");
//         ans = longest(&s1,&s2);
//         println!("The value of ans is: {}",ans);
//     }
// }

// fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// struct User<'a>{
//     name: &'a str,
// }
// fn main(){
//     let s1 = String::from("hello");
//     let user = User{name:&s1};
//     println!("The value of user is: {}",user.name);
// }

// use std::thread;


// fn main(){

//     thread::spawn(|| { 
//         for i in  1..5{
//             println!("The value of spawn: {}",i);
//         }
//     });

//     for i in 1..5{
//         println!("The value of main: {}",i);
//     }
// }

// use std::sync::mpsc::channel;
// use std::thread;
// fn main(){
//     let (tx, rx) = channel();
//     tx.send("hello").unwrap();
//     let x = rx.recv().unwrap();
//     println!("The value of x is: {}",x);
// }

// #[derive(Debug)]
// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main(){
//     let rect1 = Rect{
//         width: 10,
//         height: 20
//     };
//     println!("The value of rect1 is: {:?}",rect1);
//     println!("The area of rect1 is: {}",rect1.area());
// }


fn main(){
    
}