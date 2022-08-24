
use protobuf_codegen::Codegen;

fn main() {
	Codegen::new()
		.pure()
		.cargo_out_dir("protobuf")
		.input("src/query/mvt.proto")
		.include("src/query")
		.run_from_script();
}