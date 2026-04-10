fn main() {
    let out_dir = String::from("src/generated");
    let protobufs_directory = String::from("..");
    let proto_filenames = [
        "post.proto",
        "user.proto",
        "ads.proto",
        "lupyd-md.proto",
        "auth.proto",
        "credits.proto",
        "chats.proto",
        "notification.proto",
    ];

    let input_files = proto_filenames.map(|x| protobufs_directory.clone() + "/" + x);

    let config =
        pb_rs::ConfigBuilder::new(&input_files, None, Some(&out_dir), &[protobufs_directory])
            .unwrap()
            .custom_struct_derive(Vec::new());
    pb_rs::types::FileDescriptor::run(&config.build()).unwrap();
}
