use dialoguer::{Confirm, Input, MultiSelect, Password, Select};

use crate::installer::User;

pub fn get_shell() -> String {
    let shells = vec!["fish", "zsh", "bash"];
    let selected_shell = shells[Select::new()
        .with_prompt("Select shell")
        .items(&shells)
        .default(0)
        .interact()
        .unwrap()];

    selected_shell.to_string()
}

pub fn get_credentials() -> (String, String) {
    let username = Input::new()
        .with_prompt("Enter your username")
        .interact_text()
        .unwrap();
    let password = Password::new()
        .with_prompt("Enter your password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()
        .unwrap();

    (username, password)
}

pub fn get_packages_to_install() -> Vec<String> {
    let mut result = vec!["zip", "unzip", "curl", "wget", "git"];
    let optional_packages = vec!["neovim", "htop", "httpie", "mosh", "tree", "jq", "ripgrep"];
    result.extend(
        MultiSelect::new()
            .with_prompt("Packages to install")
            .items(&optional_packages)
            // all items are selected by default
            .defaults(&vec![true; optional_packages.len()])
            .interact()
            .unwrap()
            .iter()
            .map(|&i| &optional_packages[i]),
    );

    result.iter().map(|s| s.to_string()).collect()
}

pub fn get_additional_packages() -> Vec<String> {
    let additional_packages = vec!["python", "nodejs", "php", "pm2", "docker"];
    let selected_packages: Vec<&str> = MultiSelect::new()
        .with_prompt("Select additional packages")
        .items(&additional_packages)
        .interact()
        .unwrap()
        .iter()
        .map(|&i| additional_packages[i])
        .collect();

    selected_packages.into_iter().map(String::from).collect()

    // if selected_packages.contains(&"python") {
    //     preparation_scripts.push("sudo add-apt-repository ppa:deadsnakes/ppa".to_string());
    //     apt_packages.push("python3".to_string());

    //     if Confirm::new()
    //         .with_prompt("Do you want to install pipx?")
    //         .interact()
    //         .unwrap()
    //     {
    //         apt_packages.push("pipx".to_string());
    //     }
    // }

    // AdditionalPackages {
    //     preparation_scripts,
    //     installation_scripts,
    //     apt_packages,
    //     post_installation_scripts,
    // }
}
