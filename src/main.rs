use std::io;

fn main() {
    println!("Simple Calculator in Rust - Retroboi64 \n");

    loop {
        println!("Pick 1-4: \n");
        println!("1. Adding");
        println!("2. Subtracting");
        println!("3. multiplication");
        println!("4. division \n");
        println!("Pick: ");
        let mut pick_option = String::new();
        io::stdin().read_line(&mut pick_option).expect("Failed");
        let pick_option = pick_option.trim();


        if pick_option == "1" { // Adding
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

            println!("second Number: ");
            io::stdin().read_line(&mut y).expect("Failed");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    return;
                }
            };

            let result = x + y;
            println!("\n {x} + {y} = {result} \n");
        }

        if pick_option == "2" {
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

            println!("second Number: ");
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

        }

        if pick_option == "3" {
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

            println!("second Number: ");
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
        }

        if pick_option == "4"{
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

            println!("second Number: ");
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

        }

    }
}
