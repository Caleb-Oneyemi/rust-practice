use clap::{ App, Arg };

fn main() {
    let matches = App::new("echo-clone")
        .version("0.1.0")
        .author("Caleb Oneyemi <caloneyemi@gmail.com>")
        .about("An echo clone built with rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text to print")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("excludes newline from output")
                .takes_value(false)
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    let result = text.join(" ");
    
    if omit_newline {
        print!("{}", result)
    } else {
        println!("{}", result)
    }
}
