use std::process::{Command, Stdio};
#[allow(unused)]
pub fn main() {
    println!("\x1b[34m| OS |\x1b[37m");
    get_os();
    get_user_hostname();
    get_bash();
    println!("\x1b[32m| SYSTEM |\x1b[37m");
    get_ram_percentage();
    get_storage();
    get_cpu();
    get_gpu();
    get_battery();
    get_system();
    get_uptime();
    get_colours();
}

fn get_os() { 
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
        .args(["-d=", "-f2-"])
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
    let os = os.to_string();
    print!("OS: <-> {}", os);
}

fn get_user_hostname() {
    let usr_command = Command::new("whoami")
        .output()
        .expect("Failed to get user.");
    let usr = String::from_utf8_lossy(&usr_command.stdout);
    let usr = usr.to_string();
    let hostname_command = Command::new("hostname")
        .output()
        .expect("");
    let hostname = String::from_utf8_lossy(&hostname_command.stdout);
    let hostname = hostname.to_string();
    let hostname = hostname.trim_end();
    print!("USR/HOST: <-> {}@{}", hostname, usr);
}

fn get_storage() {
    let ps_child = Command::new("df")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()                    
        .unwrap();                    
    let grep_child_one = Command::new("grep")
        .args(["-oP", "/dev/nvme0n1p2[^ ]* (.*)"])
        .stdin(Stdio::from(ps_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_one.wait_with_output().unwrap();
    let strg = String::from_utf8_lossy(&output.stdout);
    let strg = strg.to_string();
    let strg_array: Vec<String> = strg.split_whitespace().map(str::to_string).collect();
    let storage_percentage = strg_array[4].trim_end_matches("%");
    let storage_percentage: i64 = storage_percentage.parse().unwrap();
    match storage_percentage {
        1..=65 => print!("DISK: <-> {} / {} (\x1b[32m{}%\x1b[0m)\n", strg_array[3], strg_array[1], storage_percentage),
        66..=85 => print!("DISK: <-> {} / {} (\x1b[33m{}%\x1b[0m)\n", strg_array[3], strg_array[1], storage_percentage),
        86..=100 => print!("DISK: <-> {} / {} (\x1b[31m{}%\x1b[0m)\n", strg_array[3], strg_array[1], storage_percentage),
        _ => print!("DISK: <-> {} / {} (\x1b[32m{}\x1b[0m)\n", strg_array[3], strg_array[1], storage_percentage),
    }
}

fn get_bash() {
   let bashver_command = Command::new("bash")
        .arg("--version")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
   let bashver_grep = Command::new("grep")
        .args(["-oP", "version[^(]*"])
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
   let bashv = bashv.to_string();
   print!("SHELL: <-> bash {}", bashv);
}

fn get_cpu() {
   let cpumod_command = Command::new("lscpu")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let cpumod_grep = Command::new("grep")
       .args(["-oP", r"Model name:\K.*"])
       .stdin(Stdio::from(cpumod_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let cpumod_sed = Command::new("sed")
       .args(["-e", r"s/^[[:space:]]*//g"])
       .stdin(Stdio::from(cpumod_grep.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = cpumod_sed.wait_with_output().unwrap();
   let cpu = String::from_utf8_lossy(&output.stdout);
   let cpu = cpu.to_string();
   print!("CPU: <-> {}", cpu);
}

fn get_gpu() {
   let gpumod_command = Command::new("inxi")
       .arg("-Gx")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let gpumod_grep = Command::new("grep")
       .args(["-oP", r"Device-1:\K.*(?=vendor:)"])
       .stdin(Stdio::from(gpumod_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let output = gpumod_grep.wait_with_output().unwrap();
    let gpu = String::from_utf8_lossy(&output.stdout);
    let gpu = gpu.to_string();
    print!("GPU: <->{}", gpu);
}

fn get_ram() -> Vec<String> {
   let mem_free_command = Command::new("free")
       .arg("-h")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
    let mem_grep_command = Command::new("grep")
       .args(["-oP", r"Mem:[^ ]* (.*)"])
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

fn get_ram_percentage() {
   let memp_free_command = Command::new("free")
       .arg("-g")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let memp_grep_command = Command::new("grep")
       .args(["-oP", r"Mem:[^ ]* (.*)"])
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
   let mem = get_ram();
   match ram_percentage {
     1..=30 => print!("MEMORY: <-> {} / {} (\x1b[32m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
     31..=70 => print!("MEMORY: <-> {} / {} (\x1b[33m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
     71..=100 => print!("MEMORY: <-> {} / {} (\x1b[31m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
     _ => print!("MEMORY: <-> {} / {} (\x1b[32m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
   }
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

fn get_battery_status() -> () {
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
        .args(["-d=", "-f2-"])
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
    let bstatus = bstatus.to_string();
    match bstatus.as_str() {
        "Charging" => print!("BATTERY: <-> \x1b[36mAC Connected\x1b[0m "),
        "Discharging" => print!("BATTERY: <-> \x1b[33mAC Disconnected\x1b[0m "),
        _ => print!("BATTERY: <->\x1b[33mUnknown\x1b[0mPower Connection "),
    }
}

fn get_battery() {
   let battery = get_battery_percentage();
   get_battery_status();
   match battery {
    1..=30 => print!("(\x1b[31m{}%\x1b[0m)\n", battery),
    50..=100 => print!("(\x1b[32m{}%\x1b[0m)\n", battery),
    _ => print!("(\x1b[33m{}%\x1b[0m)\n", battery),

   }
}

fn get_system() {
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
    let family = family_output.to_string(); 
    let vendor = vendor.to_string();
    print!("HOST: <-> {} {}", vendor, family);
}

fn get_uptime() {
   let uptime_cat_command = Command::new("cat")
       .arg("/proc/uptime")
       .output()
       .expect("");
   let uptime_output = String::from_utf8_lossy(&uptime_cat_command.stdout);
   let uptime_output = uptime_output.to_string();
   let uptime_array: Vec<String> = uptime_output.split_whitespace().map(str::to_string).collect();
   let uptime: f64 = uptime_array[0].parse().unwrap();
   let uptime_hours = uptime / 3600.0 % 24.0;
   let uptime_seconds = uptime % 3600.0 / 60.0;
   let uptime_hours: u32 = uptime_hours as u32;
   let uptime_seconds: u32 = uptime_seconds as u32;
   print!("UPTIME: <-> {} hours, {} mins", uptime_hours, uptime_seconds);
}

fn get_colours() {
    print!("\n\x1b[0;30m████\x1b[0;31m████\x1b[0;32m████\x1b[0;33m████\x1b[0;34m████\x1b[0;35m████\x1b[0;36m████\x1b[0;37m████\n");
    print!("\x1b[0;90m████\x1b[0;91m████\x1b[0;92m████\x1b[0;93m████\x1b[0;94m████\x1b[0;95m████\x1b[0;96m████\x1b[0;97m████\n");
}
