
#[cfg(not(feature = "oci-v1-tar"))]
fn main() {}

#[cfg(feature = "oci-v1-tar")]
fn main() {
    use anyhow::Context;
    use oci_tar_builder::Builder;
    use std::{path::PathBuf, env, fs::File};
    use oci_spec::image as spec;

    use sha256::try_digest;

    env_logger::init();


    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let p = out_dir.join("img.tar");
    let bin_output_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let app_path = bin_output_dir.join("http_server_lib.wasm");
    let slight_config = PathBuf::from("slightfile.toml");
    let windows_file = PathBuf::from("blank.txt");


    let layer_path = out_dir.join("layer.tar");
    let mut b = tar::Builder::new(File::create(&layer_path).unwrap());
    
    // windows stuff
    b.append_dir("Files", ".").unwrap();
    b.append_dir("Files\\Windows", ".").unwrap();
    b.append_dir("Files\\Windows\\System32", ".").unwrap();
    b.append_dir("Files\\Windows\\System32\\config", ".").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\DEFAULT").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SAM").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SECURITY").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SOFTWARE").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SYSTEM").unwrap();

    // files
    b.append_path_with_name(&app_path, "app.wasm").unwrap();
    b.append_path_with_name(&slight_config, "slightfile.toml").unwrap();

    let mut builder = Builder::default();

    builder.add_layer(&layer_path);

    let config = spec::ConfigBuilder::default()
        .entrypoint(vec!["/".to_owned()])
        .env(vec![
            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".to_owned(),
        ])
        .build()
        .unwrap();

    let layer_digest = try_digest(layer_path.as_path()).unwrap();
    let img = spec::ImageConfigurationBuilder::default()
        .config(config)
        .os("windows")
        .architecture("wasm")
        .rootfs(
            spec::RootFsBuilder::default()
                .diff_ids(vec!["sha256:".to_owned() + &layer_digest])
                .build()
                .unwrap(),
        )
        .build()
        .context("failed to build image configuration")
        .unwrap();

    builder.add_config(
        img,
        "docker.io/jsturtevant/qs-wasm-slight:all-in-one".to_string(),
    );

    let f = File::create(&p).unwrap();
    builder.build(f).unwrap();
    std::fs::rename(&p, bin_output_dir.join("slight-img.tar")).unwrap();
}