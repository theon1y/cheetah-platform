use std::io::Error;

fn main() -> Result<(), Error> {
	tonic_build::configure().build_server(true).build_client(true).compile(
		&["../../../proto/statistics/Events/statistics.events.external.proto"],
		&["../../../proto/statistics/Events/"],
	)
}