use std::io::{self, Write};

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}


fn main () {
  
 println!("enter a one for celsius to farhenheit or two for farhenheit to celcius");

  let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect ("failed to read line");

  let choice: f64 = choice.trim().parse().expect("please type a number");

  match choice {
    1.0 => {
      io::stdout().flush().unwrap();

        let mut temp = String::new();

          io::stdin()
          .read_line(&mut temp)
          .expect ("failed to read line");

       let celsius: f64 = temp.trim().parse().unwrap_or(0.0);

       let fahrenheit = celsius_to_fahrenheit(celsius); 
      println!("{:.0} Celsius is {:.0} Fahrenheit", celsius, fahrenheit);
      println!("it works! 🚀");
    }
    2.0 => {
      io::stdout().flush().unwrap();
      
      let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect ("failed to read line");

     let fahrenheit: f64 = temp.trim().parse().unwrap_or(0.0);
      
     let celsius = fahrenheit_to_celsius(fahrenheit);
      
      println!("{:.0} Fahrenheit is {:.0} Celsius", fahrenheit, celsius);
  
      
    }
    _ => {
            println!("Invalid option. Please choose either 1 or 2.");
        }
  }
}

