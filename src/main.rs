use std::process::{Command, Stdio};
fn main() {
    let os = get_os();
    let user = get_user();
    let strg = get_storage();
    let bashv = get_bash();
    let cpu = get_cpu();
    let kernel = get_kernel();
    print!("{}", os);
    print!("{}", user);
    print!("{} / {} (\x1b[32m{}\x1b[0m)\n", strg[3], strg[1], strg[4]);
    print!("{}", kernel);
    print!("{}", bashv);
    print!("{}", cpu);
}

fn get_os() -> String {

    let char = '"';
    let char_string = char.to_string();
    let os_cat_command = Command::new("cat")
        .arg("/etc/os-release")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let os_grep_command = Command::new("grep")
        .arg("PRETTY_NAME")
        .stdin(Stdio::from(os_cat_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let os_cut_command = Command::new("cut")
        .arg("-d=")
        .arg("-f2-")
        .stdin(Stdio::from(os_grep_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let os_tr_command = Command::new("tr")
        .arg("-d")
        .arg(char_string)
        .stdin(Stdio::from(os_cut_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = os_tr_command.wait_with_output().unwrap();
    let os = String::from_utf8_lossy(&output.stdout);
    os.to_string()
}

fn get_user() -> String {
    let usr_command =  Command::new("whoami")
        .output()
        .expect("Failed to get user.");
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

fn get_cpu() -> String {
   let cpumod_command = Command::new("lscpu")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let cpumod_grep = Command::new("grep")
       .arg("-oP")
       .arg(r"Model name:\K.*")
       .stdin(Stdio::from(cpumod_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let cpumod_sed = Command::new("sed")
       .arg("-e")
       .arg(r"s/^[[:space:]]*//g")
       .stdin(Stdio::from(cpumod_grep.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = cpumod_sed.wait_with_output().unwrap();
   let cpu = String::from_utf8_lossy(&output.stdout);
   cpu.to_string()
}

fn get_kernel() -> String {
   let output = Command::new("uname").arg("-r").
       output()
       .expect("Kernel version not found.");
   let kernel = String::from_utf8_lossy(&output.stdout);
   kernel.to_string()
}
