use std::io;
use std::process::exit;

mod user_prompt;

fn main() {
    let mut keyauthapp = keyauth::v1_2::KeyauthApi::new(
        "Application_Name", // Application Name
        "Owner_ID", // Owner ID
        "Application_Secret",  // Application Secret
        "1.0", // Application Version
        "https://keyauth.win/api/1.2/", // API URL
    );
    keyauthapp.init(None).unwrap();

    println!("เลือกตัวเลือก:\n\t[1] Login or [2] Register");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("ไม่สามารถอ่านตัวเลือก.");

    let choice = user_prompt::strip_newline(&mut choice);
    if choice != "1" && choice != "2" {
        println!("ตัวเลือกไม่ถูกต้อง! กรุณาเลือก 1 or 2 เท่านั้น.");
        exit(1);
    }

    let (username, password) = user_prompt::prompt_credentials();

    if choice == "1" {
        // Login เข้าสู่ระบบโดยใช้ Username และ Password (ไม่มี HWID)
        match keyauthapp.login(username.clone(), password, None) {
            Ok(_) => println!("เข้าสู่ระบบสำเร็จ!"),
            Err(e) => {
                println!("เข้าสู่ระบบล้มเหลว!: {}", e);
                exit(1);
            }
        }
    } else {
        let license = user_prompt::prompt_license();
        // Register โดยใช้ Username, Password และ License (ไม่มี HWID) ในระบบ keyauth.cc/app/?page=licenses
        match keyauthapp.register(username.clone(), password, license, None) {
            Ok(_) => println!("สมัครสมาชิกสำเร็จ!"),
            Err(e) => {
                println!("สมัครสมาชิกล้มเหลว!: {}", e);
                exit(1);
            }
        }
    }
    println!("สวัสดีครับ, {}", username);

    // ตั้งตัวแปร Money และให้ Value = 20
    match keyauthapp.setvar("money".to_string(), "20".to_string()) {
        Ok(_) => println!("ตั้งตัวแปร 'Money' สำเร็จ!"),
        Err(_) => {
            println!("ตั้งตัวแปร 'Money' ล้มเหลว!");
            exit(1);
        }
    }

    // แสดงค่าในตัวแปรที่รับมา
    match keyauthapp.getvar("money".to_string()) {
        Ok(balance) => println!("ยอดเงินคงเหลือ: {}", balance),
        Err(_) => println!("เรียกยอดคงเหลือ ล้มเหลว."),
    }
}
