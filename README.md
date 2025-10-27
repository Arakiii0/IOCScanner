Learning Rust by building a IOC Scanner, it goes through directories and files to see if there are any IP Address, domain names, etc.

Installation:
git clone https://github.com/Arakiii0/IOCScanner.git
cd IOCScanner
cargo build --release
cd target/release

Usage:
`IOCScanner.exe --help`
`IOCScanner.exe -p <Folder/File Path>`