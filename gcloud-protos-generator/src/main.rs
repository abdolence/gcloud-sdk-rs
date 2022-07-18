use std::{fs, path::PathBuf};

mod gen;

fn main() {
    let proto_root = PathBuf::from("gcloud-protos-generator/proto/googleapis");
    let protos = gen::find_proto(proto_root.clone());

    // let gates = gen::feature_gates(&protos);
    // println!("{}", gates);

    let out_dir = PathBuf::from("gcloud-sdk/genproto");
    let _ = fs::remove_dir_all(out_dir.as_path());
    let _ = fs::create_dir(out_dir.as_path());
    let includes = [proto_root];

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir)
        .compile_with_config(config, &gen::proto_path(&protos), &includes)
        .unwrap();

    let mut out_path = PathBuf::from("gcloud-sdk/src/google_apis.rs");
    let root = gen::from_protos(protos);
    fs::write(out_path.clone(), root.gen_code()).unwrap();

    let input_contents = fs::read_to_string(&out_path).unwrap();
    let syntax_tree = syn::parse_file(&input_contents).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::write(out_path.clone(), formatted).unwrap();

    out_path.pop();
}
