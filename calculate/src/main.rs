use std::io;
enum Operation{
    Add(f64, f64),
    Substract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),

}




fn main() {

loop{
    let mut number1 = String::new();        //kullanıcıdan 2 tane sayı almak için değişkenlerimi oluşturdum
    let mut number2 = String::new();
    let mut choice = String::new();         //kullanıcı yapmak istediği işlemi seçmesi için choice değişkeninde saklayacam
    

    //Öncelikle kullanıcıdan 2 tane sayı almak için io içindeki read_line() fonksiyonunu kullanmam gerekecek.
    //Kullananıcıdan String tipinde aldğımız için kesirli sayıya çevirmemiz gerek
    //Ama ondan da önce kullanıcıdan alırken belli başlı istenmeyen boşluklar oluşabiliyor ondan dolayı trim() fonksiyonunu kullanıyoruz.
    println!("Please enter the first number...");
    io::stdin().read_line(&mut number1).expect("Giriş okuma hatası!");
    let number1: f64 = number1.trim().parse().expect("Sayıya dönüştürme hatası!");
    println!("Please enter the second number...");
    io::stdin().read_line(&mut number2).expect("Giriş okuma hatası!");
    let number2: f64 = number2.trim().parse().expect("Sayıya dönüştürme hatası!");

    introduce();                                                                        //Ekrana işlemleri yazdırır.
    io::stdin().read_line(&mut choice).expect("Giriş okuma hatası!");          //Kullanıcıdan istediği işlemi alır.
    choice = choice.trim().to_string();
    let mut result : Option<f64> = None;                                                //Option tipinde bir değişken oluşturdum.
    //Aşağıda girilen girdiye göre işlemler yapılıyor ve konsola yazdırılıyor.
    //Eğer kullanıcı geçersiz girdi girdiyse konsola yazdırılıyor                                          
    if choice == "1"{
        result = calculate(Operation::Add(number1, number2));                    
    }
    else if choice == "2"{
        result = calculate(Operation::Substract(number1, number2));
    }
    else if choice == "3"{
        result = calculate(Operation::Multiply(number1, number2));
    }
    else if choice == "4" {
        result = calculate(Operation::Divide(number1, number2));
    }
    else if choice == "5"{
        break;
    }
    match result {
        Some(value) => println!("Result : {}", value),
        None => println!("İnvalid transaction"),
    }
}
}


fn calculate(operation:Operation)->Option<f64>{
    match operation {
        Operation::Add(number1, number2) => {Some(number1 + number2)},
        Operation::Substract(number1, number2) => {Some(number1 - number2)},
        Operation::Multiply(number1, number2) => {Some(number1 * number2)},
        Operation::Divide(number1, number2) => {Some(number1 / number2)},
    }
}

fn introduce(){
    println!("1-Add\n2-Substract\n3-Multiply\n4-Divide\n5-Quit");
}