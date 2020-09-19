use packagekit::PackageKit;
//https://www.freedesktop.org/software/PackageKit/pk-using.html
fn main() {
    println!("arch: {}", std::env::consts::ARCH);
    let pk = PackageKit::new();
    let results = pk.search_package("w3m").unwrap();
    println!("Search results: {:?}", &results);
    for pkg in &results {
        if pkg.arch() == "amd64" {
            println!("Installing {:?}", pkg);
            pk.install_packages(&vec![pkg.to_owned()]).unwrap()
        }
    }
}
