use std::process::{Command, Stdio};
use std::process;
mod battery;
mod desktop;
mod terminal;
pub fn main() {
    get_user_hostname();
    get_os();
    get_kernel();
    get_shell();
    desktop::desktops::get_desktop();
    terminal::terminal::get_terminal();
    get_ram_percentage();
    get_storage();
    get_cpu();
    get_gpu();
    battery::battery::get_battery();
    get_system();
    get_uptime();
    get_locale();
    get_ip();
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
    print!("\nOS: {}", os.to_string());
}

fn get_kernel() {
   let kernel_command = Command::new("uname")
       .arg("-r")
       .output()
       .expect("");
   let kernel = String::from_utf8_lossy(&kernel_command.stdout);
   print!("Kernel: {}", kernel.to_string());
}

fn get_user_hostname() {
    let whoami = Command::new("whoami")
        .output()
        .expect("");
    let user = String::from_utf8_lossy(&whoami.stdout);
    let hostcommand = Command::new("hostname")
        .output()
        .expect("");
    let hostname = String::from_utf8_lossy(&hostcommand.stdout);
    let str_output = format!("\x1b[0;32m{}\x1b[0;37m@\x1b[0;32m{}\x1b[0m\n", user.trim_end(), hostname.trim_end());
    print!("{}", str_output);
    let iter_output = format!("{}@{}", user.trim_end(), hostname.trim_end());
    for _ in 0..iter_output.len() {
        print!("-")
    }
}

fn get_storage() {
    let ps_command = Command::new("df")
        .arg("-h")
        .stdout(Stdio::piped())
        .spawn()                    
        .unwrap();                    
    let grep_child_one = Command::new("grep")
        .args(["-oP", "/dev/nvme0n1p2[^ ]* (.*)"])
        .stdin(Stdio::from(ps_command.stdout.unwrap()))
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
        1..=65 => print!("Disk: {}i / {}i (\x1b[32m{}%\x1b[0m)\n", strg_array[2], strg_array[1], storage_percentage),
        66..=85 => print!("Disk: {}i / {}i (\x1b[33m{}%\x1b[0m)\n", strg_array[2], strg_array[1], storage_percentage),
        86..=100 => print!("Disk: {}i / {}i (\x1b[31m{}%\x1b[0m)\n", strg_array[2], strg_array[1], storage_percentage),
        _ => print!("Disk: {} / {} (\x1b[32m{}\x1b[0m)\n", strg_array[2], strg_array[1], storage_percentage),
    }
}

fn get_shell() {
  let bash_command = Command::new("pgrep")
      .arg("bash")
      .output()
      .expect("");
   let output = String::from_utf8_lossy(&bash_command.stdout);
   let _bash = output.to_string();
   if bash_command.stdout.is_empty() {
    get_zsh();
   } else {
     get_bash()
   }
}

fn get_zsh() {
   let zshver_command = Command::new("zsh")
        .arg("--version")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
   let zshver_grep = Command::new("grep")
        .args(["-o", "^[^(]*"])
        .stdin(Stdio::from(zshver_command.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
   let zshver_head = Command::new("head")
       .arg("-1")
       .stdin(Stdio::from(zshver_grep.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = zshver_head.wait_with_output().unwrap();
   let zshv = String::from_utf8_lossy(&output.stdout);
   print!("Shell: {}", zshv.to_string());
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
   print!("Shell: bash {}", bashv.to_string());
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
   print!("CPU: {}", cpu.to_string());
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
    print!("GPU:{}", gpu.to_string());
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
     1..=30 => print!("Memory: {} / {} (\x1b[32m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
     31..=80 => print!("Memory: {} / {} (\x1b[33m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
     81..=100 => print!("Memory: {} / {} (\x1b[31m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
     _ => print!("\x1b[33mMemory: \x1b[0m){} / {} (\x1b[32m{}%\x1b[0m)\n", mem[2], mem[1], ram_percentage),
   }
}

fn get_system() {
   let system_family_command = Command::new("cat")
       .arg("/sys/devices/virtual/dmi/id/product_family")
       .output()
       .expect("");
    let family = String::from_utf8_lossy(&system_family_command.stdout);
    let system_vendor_command = Command::new("cat")
        .arg("/sys/devices/virtual/dmi/id/sys_vendor")
        .output()
        .expect("");
    let vendor_output = String::from_utf8_lossy(&system_vendor_command.stdout);
    let vendor = vendor_output.trim_end();
    print!("Host: {} {}", vendor.to_string(), family.to_string());
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
   let uptime: u32 = uptime as u32;
   if uptime >= 86400 {
      let uptime_days = uptime / 86400;
      let uptime_hours = uptime / 3600 % 24;
      let uptime_seconds = uptime % 3600 / 60;
      print!("Uptime: {} days, {} hours, {} mins", uptime_days, uptime_hours, uptime_seconds);
   } else {
      let uptime_hours = uptime / 3600 % 24;
      let uptime_seconds = uptime % 3600 / 60;
      print!("Uptime: {} hours, {} mins", uptime_hours, uptime_seconds);
   }
}

fn get_locale() {
   let locale = Command::new("locale")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let locale_sed = Command::new("sed")
       .args(["-n", "s/^.*LC_MESSAGES=//p"])
       .stdin(Stdio::from(locale.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let locale_tr = Command::new("tr")
       .args(["-d", "\""])
       .stdin(Stdio::from(locale_sed.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
  let output = locale_tr.wait_with_output().unwrap();
  let locale = String::from_utf8_lossy(&output.stdout);
  print!("\nLocale: {}", locale.to_string());
}

fn get_ip() {
   let ip_command = Command::new("ip")
       .args(["addr", "show", "wlp61s0"])
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let ip_grep = Command::new("grep")
       .args(["-oP", r"inet\K.*(?= brd )"])
       .stdin(Stdio::from(ip_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let ip_output = ip_grep.wait_with_output().unwrap();
   let ip = String::from_utf8_lossy(&ip_output.stdout);
   print!("IP:{}", ip.to_string().trim_end());
}

fn get_colours() {
    print!("\n\x1b[0;30m████\x1b[0;31m████\x1b[0;32m████\x1b[0;33m████\x1b[0;34m████\x1b[0;35m████\x1b[0;36m████\x1b[0;37m████\n");
    print!("\x1b[0;90m████\x1b[0;91m████\x1b[0;92m████\x1b[0;93m████\x1b[0;94m████\x1b[0;95m████\x1b[0;96m████\x1b[0;97m████\n");
}
