use std::io;
use std::process::exit;
use termion::input::TermRead;

pub fn strip_newline(input: &mut String) -> String {
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    input.to_string()
}

pub fn prompt_credentials() -> (String, String) {
    let mut username = String::new();
    let mut password = String::new();

    println!("กรอกชื่อผู้ใช้:");
    io::stdin().read_line(&mut username).expect("ไม่สามารถอ่านชื่อผู้ใช้.");


    println!("กรอกรหัสผ่าน:");
    if let Ok(Some(mut pass)) = TermRead::read_passwd(&mut io::stdin(), &mut io::stdout()) {
        password = pass;
    } else {
        println!("ลองอีกครั้ง!");
        exit(1);
    }

    (strip_newline(&mut username), strip_newline(&mut password))
}

pub fn prompt_license() -> String {
    let mut license = String::new();

    println!("กรอก License:");
    io::stdin().read_line(&mut license).expect("ไม่สามารถอ่าน License.");

    strip_newline(&mut license)
}