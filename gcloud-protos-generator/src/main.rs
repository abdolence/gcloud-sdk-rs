use std::{fs, path::PathBuf};

mod gen;

fn main() {
    let proto_root = PathBuf::from("gcloud-protos-generator/proto/googleapis");
    let proto_includes = PathBuf::from("gcloud-protos-generator/protobuf");
    let protos = gen::find_proto(proto_root.clone());

    let out_dir = PathBuf::from("gcloud-sdk/genproto");
    let _ = fs::remove_dir_all(out_dir.as_path());
    let _ = fs::create_dir(out_dir.as_path());
    let includes = [proto_root, proto_includes];

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    config.extern_path(
        ".google.cloud.secretmanager.v1.SecretPayload",
        "crate::proto_ext::secretmanager::SecretPayload",
    );

    config.extern_path(
        ".google.cloud.kms.v1.EncryptRequest",
        "crate::proto_ext::kms::EncryptRequest",
    );

    config.extern_path(
        ".google.cloud.kms.v1.DecryptResponse",
        "crate::proto_ext::kms::DecryptResponse",
    );

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir)
        .compile_protos_with_config(config, &gen::proto_path(&protos), &includes)
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
