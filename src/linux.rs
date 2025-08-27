use std::process::{Command, Stdio};
#[allow(unused)]
pub fn main() {
    let os = get_os();
    let user = get_user();
    let strg = get_storage();
    let bashv = get_bash();
    let cpu = get_cpu();
    let kernel = get_kernel();
    let mem = get_ram();
    let memp = get_ram_percentage();
    let battery = get_battery_percentage();
    let (sysfamily, vendor) = get_system();
    let terminal = get_termtest();
    print!("{}", os);
    print!("{}", user);
    print!("{} / {} (\x1b[32m{}\x1b[0m)\n", strg[3], strg[1], strg[4]);
    print!("{}", kernel);
    print!("{}", bashv);
    print!("{}", cpu);
    print!("{} / {} (\x1b[32m{}%\x1b[0m)\n", mem[2], mem[1], memp);
    modify_battery();
    print!("{} {}", vendor, sysfamily);
    print!("{}", terminal);
    print!("\n\x1b[0;30m████\x1b[0;31m████\x1b[0;32m████\x1b[0;33m████\x1b[0;34m████\x1b[0;35m████\x1b[0;36m████\x1b[0;37m████\n");
    print!("\x1b[0;30m▀▀▀▀\x1b[0;31m▀▀▀▀\x1b[0;32m▀▀▀▀\x1b[0;33m▀▀▀▀\x1b[0;34m▀▀▀▀\x1b[0;35m▀▀▀▀\x1b[0;36m▀▀▀▀\x1b[0;37m▀▀▀▀\n");
}

fn get_os() -> String {
    
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
        .arg("\"")
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

fn get_termtest() -> String {
    let term_command = Command::new("sh")
        .arg("-c")
        .arg("$SHELL")
        .output()
        .expect("");
    let terminal = String::from_utf8_lossy(&term_command.stdout);
    terminal.to_string()
}

fn get_term() -> String {
   let term_read_command = Command::new("echo")
       .arg("$SHELL")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let term_grep_command = Command::new("grep")
       .arg("-oP")
       .arg(r"bin/\K.*")
       .stdin(Stdio::from(term_read_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let output = term_grep_command.wait_with_output().unwrap();
    let term = String::from_utf8_lossy(&output.stdout);
    let terminal = term.to_string();
    terminal
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
   let output = Command::new("uname").arg("-r")
       .output()
       .expect("Kernel version not found.");
   let kernel = String::from_utf8_lossy(&output.stdout);
   kernel.to_string()
}

fn get_ram() -> Vec<String> {
   let mem_free_command = Command::new("free")
       .arg("-h")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let mem_grep_command = Command::new("grep")
       .arg("-oP")
       .arg(r"Mem:[^ ]* (.*)")
       .stdin(Stdio::from(mem_free_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let output = mem_grep_command.wait_with_output().unwrap();
    let mem = String::from_utf8_lossy(&output.stdout);
    let mem = mem.to_string();
    let mem_array: Vec<String> = mem.split_whitespace().map(str::to_string).collect();
    mem_array
}

fn get_ram_percentage() -> i64 {
   let memp_free_command = Command::new("free")
       .arg("-g")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let memp_grep_command = Command::new("grep")
       .arg("-oP")
       .arg(r"Mem:[^ ]* (.*)")
       .stdin(Stdio::from(memp_free_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = memp_grep_command.wait_with_output().unwrap();
   let memp = String::from_utf8_lossy(&output.stdout);
   let memp = memp.to_string();
   let memp_array: Vec<String> = memp.split_whitespace().map(str::to_string).collect();
   let memp_total: f64 = memp_array[1].parse().unwrap();
   let memp_used: f64 = memp_array[2].parse().unwrap();
   let ram_percentage: f64 = memp_used / memp_total * 100.0;
   let ram_percentage: i64 = ram_percentage as i64;
   ram_percentage
}

fn get_battery_percentage() -> i64 {
   let battery_cat_command = Command::new("cat")
   .arg("/sys/class/power_supply/BAT0/capacity")
   .output()
   .expect("");
   let output = String::from_utf8_lossy(&battery_cat_command.stdout);
   let battery = output.to_string();
   let battery: Vec<String> = battery.split_whitespace().map(str::to_string).collect();
   let battery: i64 = battery[0].parse().unwrap();
   battery
}

fn get_battery_status() -> String {
    
    let battery_cat_command = Command::new("cat")
        .arg("/sys/class/power_supply/BAT0/uevent")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let battery_grep_command = Command::new("grep")
        .arg("POWER_SUPPLY_STATUS")
        .stdin(Stdio::from(battery_cat_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let battery_cut_command = Command::new("cut")
        .arg("-d=")
        .arg("-f2-")
        .stdin(Stdio::from(battery_grep_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let battery_tr_command = Command::new("tr")
        .arg("-d")
        .arg("\"")
        .stdin(Stdio::from(battery_cut_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = battery_tr_command.wait_with_output().unwrap();
    let bstatus = String::from_utf8_lossy(&output.stdout);
    let bstatus = bstatus.trim_end();
    bstatus.to_string()
}

fn modify_battery() {
   let battery = get_battery_percentage();
   let batterystatus = get_battery_status();
   if battery >= 50 {
      print!("{}, (\x1b[32m{}%\x1b[0m)\n", batterystatus, battery);
   } else if battery <= 40 {
      print!("{}, (\x1b[31m{}%\x1b[0m)\n", batterystatus, battery);
   } else {
      print!("{}, (\x1b[33m{}%\x1b[0m)\n", batterystatus, battery);
   }
}

fn get_system() -> (String, String) {
   let system_family_command = Command::new("cat")
       .arg("/sys/devices/virtual/dmi/id/product_family")
       .output()
       .expect("");
    let family_output = String::from_utf8_lossy(&system_family_command.stdout);
    let system_vendor_command = Command::new("cat")
        .arg("/sys/devices/virtual/dmi/id/sys_vendor")
        .output()
        .expect("");
    let vendor_output = String::from_utf8_lossy(&system_vendor_command.stdout);
    let vendor = vendor_output.trim_end();
    (family_output.to_string(), vendor.to_string())
}
