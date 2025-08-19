use std::process::{Command, Stdio};
fn main() {
    let os = get_os();
    let user = get_user();
    let strg = get_storage();
    print!("{}", os);
    print!("{}", user);
    print!("{}", strg);
}


fn get_os() -> String {
    let os_command = Command::new("uname").output().expect("Failed to get OS.");
    let os_output = String::from_utf8_lossy(&os_command.stdout);
    let os = format!("{}", os_output);
    os
}

fn get_user() -> String {
    let usr_command =  Command::new("whoami").output().expect("Failed to get user.");
    let usr_output = String::from_utf8_lossy(&usr_command.stdout);
    let usr = format!("{}", usr_output);
    usr

}

fn get_storage() -> String{
 let ps_child = Command::new("df")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()                    
        .unwrap();                    
    let grep_child_one = Command::new("grep")
        .arg("-oP")
        .arg("Used")
        .stdin(Stdio::from(ps_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_one.wait_with_output().unwrap();
    let result = String::from_utf8_lossy(&output.stdout);
    result.to_string()
}