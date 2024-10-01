mod helpers;
mod installer;
mod packages;
mod questions;

use installer::Installer;

fn main() {
    let (username, password) = questions::get_credentials();
    let shell = questions::get_shell();

    let mut installer = Installer::new(username, password, shell);

    installer.add_apt_packages(questions::get_packages_to_install());

    // let additionalpackages = questions::get_additional_packages();

    println!();
    println!("sudo apt install {}", apt_install.join(" "));
    println!("sudo chsh -s /usr/bin/{} {}", using_shell, user.username);
}
