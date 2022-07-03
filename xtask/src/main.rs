use std::{env, fs, path::PathBuf};

mod gen;

fn main() {
    match env::args().nth(1) {
        Some(cmd) => match cmd.as_str() {
            "gen" => gen(),
            _ => print_help(),
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!(r#"cargo xtask gen"#)
}

fn gen() {
    let proto_root = PathBuf::from("xtask/proto/googleapis");
    let protos = gen::find_proto(proto_root.clone());

    // let gates = gen::feature_gates(&protos);
    // println!("{}", gates);

    let out_dir = PathBuf::from("googapis/genproto");
    let _ = fs::remove_dir_all(out_dir.as_path());
    let _ = fs::create_dir(out_dir.as_path());
    let includes = [proto_root];

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir.clone())
        .compile_with_config(config, &gen::proto_path(&protos), &includes)
        .unwrap();

    let mut out_path = PathBuf::from("googapis/src/googapis.rs");
    let root = gen::from_protos(protos);
    fs::write(out_path.clone(), root.gen_code()).unwrap();

    out_path.pop();
}
