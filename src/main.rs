use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let name: &str = "Tibi";
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("usage: {} filename", args[0]);
        return Ok(());
    }

    let filename: &String = &args[1];
    let content = read_file(filename)?;
    println!("{content}");

    loop {
        print!("Do you want to read another file? (y/n)");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" => {
                print!("Enter file path: ");
                io::stdout().flush()?;
                input = String::new();
                io::stdin().read_line(&mut input)?;
                let content = read_file(&input)?;
                println!("{content}");
            }

            "n" | "" => {
                //n ou rien
                println!("Ciao!");
                break;
            }
            _ => println!("Invalid input, please enter y or n"),
        }
    }

    Ok(())
}

fn read_file(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}
