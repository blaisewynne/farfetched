use std::process::{Command, Stdio};
use std::process;
use std::env;

pub fn get_desktop() {
   let desktop_environment = "XDG_CURRENT_DESKTOP";

   match env::var(desktop_environment).expect("").as_str() {
      "Hyprland" => get_hyprland(),
      "Cinnamon" => get_cinnamon(),
      "GNOME" => get_gnome(),
      _ => print!("Unknown Desktop\n")
   }
}

fn get_hyprland() {
   let hyprland_command = Command::new("hyprctl")
       .arg("version")
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let hyprland_sed = Command::new("sed")
       .args(["-n", r"s/.*Tag:\(.*\),.*/\1/p"])
       .stdin(Stdio::from(hyprland_command.stdout.unwrap()))
       .stdout(Stdio::piped())
       .spawn()
       .unwrap();
   let output = hyprland_sed.wait_with_output().unwrap();
   let hyprland = String::from_utf8_lossy(&output.stdout);
   print!("DE: Hyprland {}\n", hyprland.to_string().trim());
}

fn get_gnome() {
   let gnome_command = Command::new("gnome-shell")
       .arg("--version")
       .output()
       .expect("");
   let gnome = String::from_utf8_lossy(&gnome_command.stdout);
   print!("DE: {}", gnome.to_string());
}

fn get_cinnamon() {
   let cinnamon_command = Command::new("cinnamon-shell")
       .arg("--version")
       .output()
       .expect("");
   let cinnamon = String::from_utf8_lossy(&cinnamon_command.stdout);
   print!("DE: {}", cinnamon.to_string());
}
