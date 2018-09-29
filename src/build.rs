fn main() {
    println!("cargo:rustc-link-lib=static=mysqlclient");
    println!("cargo:rustc-link-search=native=/usr/local/mysql/lib");
}
