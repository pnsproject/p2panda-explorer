fn main() {
    cynic_codegen::register_schema("p2panda")
        .from_sdl_file("../schemas/p2panda.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
