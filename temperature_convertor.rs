use std::io;

pub fn cel_to_faren(user_input: &String) -> f64 {
    let celcius: f64 = user_input.trim().parse().expect("Failed to Parse");
    let farenheit : f64 = (celcius * 1.8) + 32.0;

    return farenheit;

}


pub fn faren_to_cel(user_input : &String) -> f64 {
    let farenheit: f64 = user_input.trim().parse().expect("Failed to Parse");
    let celcius : f64 = (farenheit + 32.0) * 0.56;

    return celcius;
}

pub fn temp_convertor(){
    let mut cont = true;
    while cont {
         println!("=========================");
        println!("    Temperature Convertor    ");
        println!("=========================\n");
        println!("1.Celcius to Farenheit");
        println!("2. Farenheit to Celcius");
        println!("3.Break from Loop/Exit");

        println!("Enter Your Choice: ");
        let mut choice : String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to Read the Choice");

        println!("Please Enter your Temperature: ");
        let mut user_input : String = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to Read Temperature");

        match choice.trim().chars().next() {
            Some('1') =>{
                let result = cel_to_faren(&user_input);
                println!("Farenheit value: {}",result);
            },
            Some('2') =>{ 
                let result = faren_to_cel(&user_input);
                println!("Celcius Value: {}",result);
            }Some('3') => {
                println!("You have exited the loop");
                cont = false;
                
            }
            _ => {
                println!("Invalid Input");
                
            }

            
        }
    }    
}