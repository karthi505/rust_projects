use std::thread;
use std::time::Duration;

pub fn timer() {

    println!("Start the Timer(Type Yes)\n");
    println!("Press Ctrl + C to Break or Stop the Timer");
    let mut input : String = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    //let mut vec_timer : Vec<i32> = Vec::new();

    let mut count = 0;
    let mut min_count = 0;
    
    // you would have to forcfully break from the loop in cmd line(ctrl + c)
    //which i am not to happy about

        while true {
    
            if count == 60  || count > 60{
                count = 0;
                min_count += 1;

                while count < 60{
                    
                    println!("Time: {}min {}s",min_count,count);
                    thread::sleep(Duration::from_secs(1));
                    count += 1
            }
            }
        else{
            println!("Time: {}s",count);
            thread::sleep(Duration::from_secs(1));
            count = count + 1;
        }
    }

}

