use std::process::Command;

fn main() {
    println!("Building!!!");
    // Build the frontend with wasm32
    let output = Command::new("cargo")
        .current_dir("app/")
        .arg("build")
        .arg("--target=asmjs-unknown-emscripten")
        .output()
        .expect("Front end didn't build successfully");

    println!("{:?}", output);

    let copy_js = Command::new("cp")
        .arg("app/target/asmjs-unknown-emscripten/debug/app.js")
        .arg("static/script/app/app.js")
        .output()
        .expect("Couldn't move app.js");

    println!("{:?}", copy_js);

}