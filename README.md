# แอป KeyAuth โดย Rust

นี่คือโปรเจกต์ที่ใช้ Rust เพื่อสร้างแอปที่เชื่อมต่อกับ KeyAuth API สมัครสมาชิกและเข้าสู่ระบบ

## Requirements

- [Rust](https://www.rust-lang.org/) รุ่นล่าสุด
- [obfstr](https://github.com/CasualX/obfstr) รุ่นล่าสุด
- [termion](https://github.com/redox-os/termion) รุ่นล่าสุด
- บัญชี [KeyAuth](https://keyauth.win/) และ API ที่ถูกต้อง

## Installation

1. Clone โปรเจกต์นี้:

   ```bash
   git clone https://github.com/pichxyaponn/simpleKeyAuth.git
   cd simpleKeyAuth

2. ติดตั้ง Dependencies:

   ```bash
   cargo build

## Usage

1. รันแอป:

   ```bash
   Copy code
   cargo run
   ```
2. เลือก Login หรือ Register

## Main Features

- **Login**: ใช้ชื่อผู้ใช้และรหัสผ่านเพื่อเข้าสู่ระบบ.
- **Register**: สมัครสมาชิกใหม่โดยใช้ชื่อผู้ใช้ รหัสผ่าน และ License.
- **Set Variable**: ตั้งค่าตัวแปร
- **Get Variable**: รับค่าตัวแปร.

## Support

หากมีปัญหาหรือมีคำแนะนำ ให้สร้าง Issue หรือ Pull Request ใน GitHub repository นี้.

## Contributors

- [KeyAuth](https://keyauth.win/) - A Cloud-Based Subscription Authentication Platform.

## License

Copyright © 2024 pichxyaponn

This project is licensed under the MIT License.
