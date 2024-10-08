use std::io;

fn main() {
    println!("Simple Calculator in Rust - Retroboi64 \n");

    loop {
        // Main Loop
        println!("Pick 1-4: \n");
        println!("1. Adding");
        println!("2. Subtracting");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Info\n");
        println!("Pick: ");

        let mut pick_option = String::new();
        io::stdin().read_line(&mut pick_option).expect("Failed");
        let pick_option = pick_option.trim();

        if pick_option == "1" {
            // Adding
            let mut x = String::new();
            let mut y = String::new();
            println!("First Number: ");
            io::stdin().read_line(&mut x).expect("Failed");
            let x: f64 = match x.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            println!("Second Number: ");
            io::stdin().read_line(&mut y).expect("Failed");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            let result = x + y;
            println!(" Problem: ");
            println!("\n {x} + {y} = {result} \n");

            let mut stop = String::new();
            println!("Stop No/Yes? ");
            io::stdin().read_line(&mut stop).expect("Error");
            let stop = stop.trim();

            if stop == "yes" {
                break;
            } else {
                if stop == "no" {
                    // Back to main loop
                }
                println!("\n{stop} is not an option\n");
                break;
            }
        }

        if pick_option == "2" {
            // Subtracting
            let mut x = String::new();
            let mut y = String::new();
            println!("First Number: ");
            io::stdin().read_line(&mut x).expect("failed"); //Funny Number
            let x: f64 = match x.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            println!("Second Number: ");
            io::stdin().read_line(&mut y).expect("Failed");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            let result = x - y;
            println!("\n {x} - {y} = {result} \n");

            let mut stop = String::new();
            println!("Stop No/Yes? ");
            io::stdin().read_line(&mut stop).expect("Error");
            let stop = stop.trim();

            if stop == "yes" {
                break;
            } else {
                if stop == "no" {
                    // Back to main loop
                }
                println!("\n{stop} is not an option\n");
                break;
            }
        }

        if pick_option == "3" {
            // Multiple
            let mut x = String::new();
            let mut y = String::new();
            println!("First Number: ");
            io::stdin().read_line(&mut x).expect("failed");
            let x: f64 = match x.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            println!("Second Number: ");
            io::stdin().read_line(&mut y).expect("Failed");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            let result = x * y;
            println!("\n {x} * {y} = {result} \n");

            let mut stop = String::new();
            println!("Stop No/Yes? ");
            io::stdin().read_line(&mut stop).expect("Error");
            let stop = stop.trim();

            if stop == "yes" {
                break;
            } else {
                if stop == "no" {
                    // Back to main loop
                }
                println!("\n{stop} is not an option\n");
                break;
            }
        }

        if pick_option == "4" {
            // Division
            let mut x = String::new();
            let mut y = String::new();
            println!("First Number: ");
            io::stdin().read_line(&mut x).expect("failed");
            let x: f64 = match x.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            println!("Second Number: ");
            io::stdin().read_line(&mut y).expect("Failed");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            let result = x / y;
            println!("\n {x} / {y} = {result} \n");

            let mut stop = String::new();
            println!("Stop No/Yes? ");
            io::stdin().read_line(&mut stop).expect("Error");
            let stop = stop.trim();

            if stop == "yes" {
                break;
            } else {
                if stop == "no" {
                    // Back to main loop
                }
                println!("\n{stop} is not an option\n");
                break;
            }
        }

        if pick_option == "5" {
            // Info
            println!("\n");
            println!("Simple Calculator in Rust By Retroboi64 (Because I was bored lol)");
            println!("\n");
        }

        if pick_option == "69" {
            println!("\n");
            println!("Funny Number");
            println!("\n");
        }
    }
}
