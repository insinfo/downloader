#[derive(Debug)]
pub struct Pessoa {
    pub  name: String,
    pub  idade: u32,
}

pub trait CalculaIdade {
    fn calcula_idade(&self) -> u32;
    fn description(&self) -> String;
}

// Implementing an in-built trait ToString on the Dog struct
impl CalculaIdade for Pessoa {
    // This method takes ownership of both passed arguments,
    fn calcula_idade(&self) -> u32{
        self.idade + 2
    }
    // Method returns an overview of the movie
    fn description(&self) -> String{
        return format!("{}", self.name);
    }
}
/*impl Pessoa{
    #[allow(dead_code)]
    pub fn calcula_idade(&self) -> &i32 {
        &self.idade
    }
}*/
//type ApelidoPerson = Person;
/*#[allow(dead_code)]
pub fn calcula_idade() {
}
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<i8> for Number {
    fn from(item: i8) -> Self {
        Number { value: item as i32 }
    }
}*/
/*let _nome = "Isaque";
    let mut xs: [i8; 5] = [1, 2, 3, 4, 5];

    let p = Pessoa { name: String::from("Isaque"), idade: 36 };
    println!("{:?} ", p);
    println!("array occupies {} bytes", sizeOf(&xs));

    analyze_slice(&xs);
    println!("primeiro item depois: {}", xs[0]);

    let num = Number::from(30);
    println!("My number is {:?}", num);

fn analyze_slice(slice: &[i8]) {
    let mut array = Vec::from(slice);
    array[0] = 6;
    array.push(7);
    println!("primeiro item dentro da função: {}", array[0]);
    println!("the slice has {} elements", array.len());
}*/