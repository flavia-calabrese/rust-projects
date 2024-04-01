use std::fs;
use std::time::SystemTime;
use clap::builder::Str;

enum Error {
    Simple (SystemTime),
    Complex (SystemTime, String),
}
fn print_error(e:Error){
    match e {
        Error::Simple(time) => println!("Simple Error: {}", time.elapsed().unwrap().as_secs()),
        Error::Complex(time, str) => println!("Complex Error: {}, {}", time.elapsed().unwrap().as_secs(), str),

    }
}
#[derive(Debug)]
pub enum MulErr {
    Overflow,
    NegativeNumber
}

fn mul(a: i32, b: i32) -> Result<u32, MulErr> {
    if a < 0 || b < 0 {
        return Err(MulErr::NegativeNumber);
    }
    return match (a as u32).checked_mul(b as u32) {
        Some(res) => Ok(res),
        None => Err(MulErr::Overflow),
    }
}


struct Node {
    name: String, // 8 byte + 8 byte + 8 byte
    size: u32,    // 4 byte
    count: u32,   // 4 byte
}
impl Node{
    pub fn new(name:String) -> Self{
        Node {name, size:0, count:0}
    }

    pub fn size(mut self, size:u32) -> Self {
        self.size = size;
        return self;
    }

    pub fn count(mut self, count:u32) -> Self {
        self.count = count;
        return self;
    }

    pub fn change_name(mut self, new_name:String) -> Self{
        self.name = format!("{}{}",self.name, new_name );
        return self;
    }

    pub fn to_string(&self) -> String {
        return format!("name:{}, size:{}, count:{}", self.name, self.size, self.count);
    }

    pub fn grow(& mut self) {
        self.size += 1;
    }
    pub fn inc(& mut self) {
        self.count += 1;
    }
}


fn main() {
    // read_file("test.txt".to_string());
    // read_fun2("t2.txt");

    // print_error(Error::Simple(SystemTime::now()));
    // print_error(Error::Complex(SystemTime::UNIX_EPOCH, "uffa".to_string()));
    // println!("{:?}", mul(22222222,66666666) );

    let mut n = Node::new("brendo".to_string()).size(1).count(2);
    println!("{}", n.to_string());
    n.grow();
    n.inc();
    println!("{}", n.to_string());


}

fn read_file(str:String){
    let content = fs::read_to_string(&str);
    let result = match content {
       Err(err) => panic!("Read the file is not possible. {}", err),
        Ok(str) => str,
    };
    let mut new_content = String::new();
    for _ in 0..10 {
       new_content.push_str(&result);
    }
    let after_write = fs::write(&str, new_content);
    match after_write {
        Err(err)=> panic!("Write the file is not possible. {}", err),
        Ok(()) => {},
    };
}

fn read_fun2(str:&str){
    let res = fs::read(&str).expect("File not found");
    println!("{:02X?}", res);

}
