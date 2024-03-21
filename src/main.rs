use std::io;
fn main() {
    println!("RustStorageConverter v1.0");
    println!("Licensed under GNU General Public Lisence v3.0 (GPLv3)");
    println!("What would you like to convert from");
    println!("(1) = Bytes, (2) = Kilobytes, (3) = Megabytes, (4) = Gigabytes, (5) = Terabytes, (6) = Petabytes");
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read line");
    let first: i8 = first.trim().parse().expect("Please insert a number");
    println!("What would you like to convert to?");
    println!("(1) = Bytes, (2) = Kilobytes, (3) = Megabytes, (4) = Gigabytes, (5) = Terabytes, (6) = Petabytes");
    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read line");
    let second: i8 = second.trim().parse().expect("Please insert a number");
    if first == 1 || first == 2 || first == 3 || first == 4 || first == 5 || first == 6 {
        if second == 1 || second == 2 || second == 3 || second == 4 || second == 5 || second == 6 {
            let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line");
            let number: f64 = number.trim().parse().expect("Please insert a number");
            // Bytes
            if first == 1 {
                // Bytes to Bytes
                if second == 1 {
                    println!("Cannot convert to same size");
                }
                // Bytes to Kilobytes
                if second == 2 {
                    let conv = number / 1000.0;
                    println!("{conv} Kilobytes");
                }
                // Bytes to Megabytes
                if second == 3 {
                    let conv = number / 1000000.0;
                    println!("{conv} Megabytes");
                }
                // Bytes to Gigabytes
                if second == 4 {
                    let conv = number / 1000000000.0;
                    println!("{conv} Gigabytes");
                }
                // Bytes to Terabytes
                if second == 5 {
                    let conv = number / 1000000000000.0;
                    println!("{conv} Terabytes")
                }
                // Bytes to Petabytes
                if second == 6 {
                    let conv = number / 1000000000000000.0;
                    println!("{conv} Petabytes");
                }
            }
            // Kilobytes
            if first == 2 {
                // Kilobytes to Bytes
                if second == 1 {
                    let conv = number * 1000.0;
                    println!("{conv} Bytes");
                }
                // Kilobytes to Kilobytes
                if second == 2 {
                    println!("Cannot convert to same size");
                }
                // Kilobytes to Megabytes
                if second == 3 {
                    let conv = number / 1000.0;
                    println!("{conv} Megabytes");
                }
                // Kilobytes to Gigabytes
                if second == 4 {
                    let conv = number / 1000000.0;
                    println!("{conv} Gigabytes");
                }
                // Kilobytes to Terabytes
                if second == 5 {
                    let conv = number / 1000000000.0;
                    println!("{conv} Terabytes");
                }
                // Kilobytes to Petabytes
                if second == 6 {
                    let conv = number / 1000000000000.0;
                    println!("{conv} Petabytes");
                }
            }
            // Megabytes
            if first == 3 {
                // Megabytes to Bytes
                if second == 1 {
                    let conv = number * 1000000.0;
                    println!("{conv} Bytes");
                }
                // Megabytes to Kilobytes
                if second == 2 {
                    let conv = number * 1000.0;
                    println!("{conv} Kilobytes");
                }
                // Megabytes to Megabytes
                if second == 3 {
                    println!("Cannot convert to same size");
                }
                // Megabytes to Gigabytes
                if second == 4 {
                    let conv = number / 1000.0;
                    println!("{conv} Gigabytes");
                }
                // Megabytes to Terabytes
                if second == 5 {
                    let conv = number / 1000000.0;
                    println!("{conv} Terabytes");
                }
                // Megabytes to Petabytes
                if second == 6 {
                    let conv = number / 1000000000.0;
                    println!("{conv} Petabytes");
                }
            }
            // Gigabytes
            if first == 4 {
                // Gigabytes to Bytes
                if second == 1 {
                    let conv = number * 1000000000.0;
                    println!("{conv} Bytes");
                }
                // Gigabytes to Kilobytes
                if second == 2 {
                    let conv = number * 1000000.0;
                    println!("{conv} Kilobytes");
                }
                // Gigabytes to Megabytes
                if second == 3 {
                    let conv = number * 1000.0;
                    println!("{conv} Kilobytes");
                }
                // Gigabytes to Gigabytes
                if second == 4 {
                    println!("Cannot convert to same size");
                }
                // Gigabytes to Terabytes
                if second == 5 {
                    let conv = number / 1000.0;
                    println!("{conv} Terabytes");
                }
                // Gigabytes to Petabytes
                if second == 6 {
                    let conv = number / 1000000.0;
                    println!("{conv} Petabytes");
                }
            }
            // Terabytes
            if first == 5 {
                // Terabytes to Bytes
                if second == 1 {
                    let conv = number * 1000000000000.0;
                    println!("{conv} Bytes");
                }
                // Terabytes to Kilobytes
                if second == 2 {
                    let conv = number * 1000000000.0;
                    println!("{conv} Kilobytes");
                }
                // Terabytes to Megabytes
                if second == 3 {
                    let conv = number * 1000000.0;
                    println!("{conv} Megabytes");
                }
                // Terabytes to Gigabytes
                if second == 4 {
                    let conv = number * 1000.0;
                    println!("{conv} Gigabytes");
                }
                // Terabytes to Terabytes
                if second == 5 {
                    println!("Cannot Convert to Same Size");
                }
                // Terabytes to Petabytes
                if second == 6 {
                    let conv = number / 1000.0;
                    println!("{conv} Petabytes");
                }
            }
            // Petabytes
            if first == 6 {
                // Petabytes to Bytes
                if second == 1 {
                    let conv = number * 1000000000000000.0;
                    println!("{conv} Bytes");
                }
                // Petabytes to Kilobytes
                if second == 2 {
                    let conv = number * 1090000000000.0;
                    println!("{conv} Bytes");
                }
                // Petabytes to Megabytes
                if second == 3 {
                    let conv = number * 1000000000.0;
                    println!("{conv} Megabytes");
                }
                // Petabytes to Gigabytes
                if second == 4 {
                    let conv = number * 1000000.0;
                    println!("{conv} Gigabytes");
                }
                // Petabytes to Terabytes
                if second == 5 {
                    let conv = number * 1000.0;
                    println!("{conv} Terabytes");
                }
                // Petabytes to Petabytes
                if second == 6 {
                    println!("Cannot Convert to Same size");
                }
            }
        }
        else {
            println!("That is not an option");
        }
    }
    else {
        println!("That is not an option");
    }
    // Exit
    println!("---------------");
    println!("Press the ENTER key to exit!");
    let mut exit = String::new();
    io::stdin()
        .read_line(&mut exit)
        .expect("Closing with Failed to read line Error");

}
