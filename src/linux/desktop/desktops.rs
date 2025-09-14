use std::process::{Command, Stdio};
use std::process;
use std::env;

pub fn get_desktop() {
   let desktop_environment = "XDG_CURRENT_DESKTOP";

   match env::var(desktop_environment).expect("").as_str() {
      "Hyprland" => get_hyprland(),
      "KDE" => print!("DESKTOP: KDE Plasma\n"),
      "GNOME" => print!("DESKTOP"),
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
   let hyprland_output = hyprland_sed.wait_with_output().unwrap();
   let hyprland = String::from_utf8_lossy(&hyprland_output.stdout);
   print!("DESKTOP: Hyprland {}\n", hyprland.to_string().trim());

}