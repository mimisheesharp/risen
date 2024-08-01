use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write};

fn license() {
    let licenses: Vec<&str> = vec!["mit"];

    println!("License Generator");
    println!("What license do you want?");
    print!("(type 'list' to output licenses): ");
    let mut input: String = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);
    match input.trim() {
        "list" => {
            println!("Available Licenses:");
            for keyword in licenses {
                println!("{}", keyword);
            }
            println!("");
            println!("");
            license()
        }
        "mit" => {
            print!("your name: ");
            let _ = stdout().flush();
            let mut name: String = String::new();
            let _ = stdin().read_line(&mut name);
            print!("project year: ");
            let _ = stdout().flush();
            let mut year = String::new();
            let _ = stdin().read_line(&mut year);
            println!("Generating a mit license...");
            let license = File::create("LICENSE");
            let txt = format!(
                r#"MIT License

Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
                year.trim(), name.trim()
            );
            let _ = license.expect("error").write_all(txt.as_bytes());
        }
        _ => {
            println!("\x1b[31mThe license you entered has not been added yet.");
            println!("(Maybe you have the wrong keyword. Please check the list.)\x1b[m");
        }
    }
}
fn main() {
    let version: String = "v1.0.0".to_string();
    println!("Risen version: \x1b[35m{}\x1b[m", version);
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let gen_type: String = args[1].to_ascii_lowercase(); //.to_ascii_lowercase(): 大文字があったら小文字にする
        if &gen_type == "license" {
            license()
        } else {
        }
    }
}
