pub struct Package {
    preparation_scripts: Vec<String>,
    installation_scripts: Vec<String>,
    apt_packages: Vec<String>,
    post_installation_scripts: Vec<String>,
}
