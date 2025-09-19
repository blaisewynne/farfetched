use std::process::{Command, Stdio};
use std::process;

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
        "Charging" => print!("Battery: \x1b[36m ⚡︎ AC Connected ⚡︎ \x1b[0m"),
        "Discharging" => print!("Battery: \x1b[33mAC Disconnected\x1b[0m "),
        "Full" => print!("Battery: \x1b[32mFully Charged\x1b[0m"),
        "Not charging" => print!("Battery: \x1b[31mNot Charging\x1b[0m"),
        _ => print!("Battery: \x1b[33mUnknown \x1b[0mPower Connection"),
    }
}

pub fn get_battery() {
   let battery = get_battery_percentage();
   get_battery_status();
   match battery {
    1..=30 => print!("(\x1b[31m{}%\x1b[0m)\n", battery),
    50..=100 => print!("(\x1b[32m{}%\x1b[0m)\n", battery),
    _ => print!("(\x1b[33m{}%\x1b[0m)\n", battery),

   }
}