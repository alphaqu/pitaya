
use protobuf_codegen::Codegen;

fn main() {
	Codegen::new()
		.pure()
		.cargo_out_dir("protobuf")
		.input("src/protos/mvt.proto")
		.include("src/protos")
		.run_from_script();
}