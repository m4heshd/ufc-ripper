// Libs
use winresource::WindowsResource;

fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let profile = std::env::var("PROFILE").unwrap();
    let exe_res = std::env::var("CARGO_FEATURE_EXE_RES").is_ok();

    // Adds resources to the Windows builds
    if target == "windows" && profile == "release" && exe_res {
        println!("cargo:warning=Adding Windows build resources");

        let mut res = WindowsResource::new();

        res.set_icon("../project-res/images/ufc-ripper-icon.ico")
            .set("ProductName", "UFC Ripper")
            .set("FileDescription", "UFC Ripper")
            .set(
                "LegalCopyright",
                "Copyright © 2024 Mahesh Bandara Wijerathna (m4heshd)",
            )
            .compile()
            .unwrap();
    }
}
