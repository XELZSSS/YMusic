fn main() {
    tauri_build::build();
    println!("cargo:rerun-if-changed=../src/scripts/");
    println!("cargo:rerun-if-changed=../src/styles/");
    println!("cargo:rerun-if-changed=../src/index.html");
    println!("cargo:rerun-if-changed=../src/assets/");
    println!("cargo:rerun-if-changed=tauri.conf.json");
    println!("cargo:rerun-if-changed=../.gitignore");
}
