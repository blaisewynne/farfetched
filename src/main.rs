use std::process::{Command, Stdio};
fn main() {
    let os = get_os();
    let user = get_user();
    let strg = get_storage();
    let bashv = get_bash();
    print!("{}", os);
    print!("{}", user);
    print!("{} / {} ({})\n", strg[3], strg[1], strg[4]);
    print!("{}", bashv);
}


fn get_os() -> String {
    let os_command = Command::new("uname").output().expect("Failed to get OS.");
    let os = String::from_utf8_lossy(&os_command.stdout);
    os.to_string()
}

fn get_user() -> String {
    let usr_command =  Command::new("whoami").output().expect("Failed to get user.");
    let usr = String::from_utf8_lossy(&usr_command.stdout);
    usr.to_string()
}

fn get_storage() -> Vec<String> {
 let ps_child = Command::new("df")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()                    
        .unwrap();                    
    let grep_child_one = Command::new("grep")
        .arg("-oP")
        .arg("/dev/nvme0n1p2[^ ]* (.*)")
        .stdin(Stdio::from(ps_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_one.wait_with_output().unwrap();
    let strg = String::from_utf8_lossy(&output.stdout);
    let strg = strg.to_string();
    let strg_array: Vec<String> = strg.split_whitespace().map(str::to_string).collect();
    strg_array

}

fn get_bash() -> String {
   let bashver_command = Command::new("bash")
        .arg("--version")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
   let bashver_grep = Command::new("grep")
        .arg("version")
        .stdin(Stdio::from(bashver_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
   let bashver_head = Command::new("head")
       .arg("-1")
       .stdin(Stdio::from(bashver_grep.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = bashver_head.wait_with_output().unwrap();
   let bashv = String::from_utf8_lossy(&output.stdout);
   bashv.to_string()
}


