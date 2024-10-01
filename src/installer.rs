use crate::helpers::vec_of_strings;
use crate::packages::Package;

pub struct User {
    pub username: String,
    pub password: String,
}
pub struct Installer {
    user: User,
    shell: String,

    packages: Vec<Package>,
}

impl Installer {
    pub fn new(username: String, password: String, shell: String) -> Self {
        let mut apt_packages = vec_of_strings(vec!["zip", "unzip", "curl", "wget", "git"]);
        if shell != "bash" {
            apt_packages.push(shell.to_string());
        }

        let installer = Self {
            user: User { username, password },
            shell,
            preparation_scripts: vec![],
            installation_scripts: vec![],
            apt_packages,
            post_installation_scripts: vec![],
        };

        installer
    }

    pub fn add_preparation_script(&mut self, script: String) {
        self.preparation_scripts.push(script);
    }

    pub fn add_installation_script(&mut self, script: String) {
        self.installation_scripts.push(script);
    }

    pub fn add_apt_package(&mut self, package: String) {
        self.apt_packages.push(package);
    }

    pub fn add_apt_packages(&mut self, package: Vec<String>) {
        self.apt_packages.extend(package);
    }

    pub fn add_post_installation_script(&mut self, script: String) {
        self.post_installation_scripts.push(script);
    }
}
