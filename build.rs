extern crate pkg_config;

fn main() {

    let force_pkg = std::env::var_os("NLOPT_SYS_USE_PKG_CONFIG").is_some();
    if force_pkg {

        if let Ok(lib) = pkg_config::Config::new().atleast_version("2.9.1").probe("nlopt") {
            println!("cargo:rerun-if-env-changed=NLOPT_SYS_USE_PKG_CONFIG");
            println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");

            for p in lib.link_paths { println!("cargo:rustc-link-search=native={}", p.display()); }
            for n in lib.libs { 
                println!("cargo:rustc-link-lib={n}"); 
            }
            return;
        }
    } else {
        let dst = cmake::Config::new("./nlopt-2.9.1")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("NLOPT_CXX", "OFF")
            .define("NLOPT_PYTHON", "OFF")
            .define("NLOPT_OCTAVE", "OFF")
            .define("NLOPT_MATLAB", "OFF")
            .define("NLOPT_GUILE", "OFF")
            .define("NLOPT_SWIG", "OFF")
            .define("NLOPT_LINK_PYTHON", "OFF")
            .build();
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    }

    println!("cargo:rustc-link-lib=static=nlopt");
}
