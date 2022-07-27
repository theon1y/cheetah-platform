use std::{error::Error, net::SocketAddr};

use jwt_tonic_user_uuid::JWTUserTokenParser;
use tonic::Response;
use tonic::{transport::Server, Request, Status};
use uuid::Uuid;
use ydb::Client;

use cheetah_libraries_microservice::{init, trace::trace_err};
use update::UpdateService;
use userstore::update_server::UpdateServer;

use crate::ydb::Error as YdbError;

use self::{fetch::FetchService, userstore::fetch_server::FetchServer};

mod fetch;
mod reply;
mod update;
mod value;
mod userstore {
	tonic::include_proto!("cheetah.user.store");
}

pub struct Service {
	ydb_client: Client,
	jwt_public_key: String,
}

impl Service {
	pub fn new(ydb_client: Client, jwt_public_key: String) -> Self {
		Self {
			ydb_client,
			jwt_public_key,
		}
	}

	pub async fn serve(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
		init("userstore");

		let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

		let update_service =
			UpdateService::new(self.ydb_client.table_client(), self.jwt_public_key.clone());
		health_reporter
			.set_serving::<UpdateServer<UpdateService>>()
			.await;

		let fetch_service =
			FetchService::new(self.ydb_client.table_client(), self.jwt_public_key.clone());
		health_reporter
			.set_serving::<FetchServer<FetchService>>()
			.await;

		Server::builder()
			.accept_http1(true)
			.add_service(tonic_web::enable(health_service))
			.add_service(tonic_web::enable(UpdateServer::new(update_service)))
			.add_service(tonic_web::enable(FetchServer::new(fetch_service)))
			.serve(addr)
			.await?;

		Ok(())
	}
}

fn verify_credentials<T>(request: Request<T>, jwt_public_key: &str) -> Result<(Uuid, T), Status> {
	let parser = JWTUserTokenParser::new(jwt_public_key.to_string());
	match parser.get_user_uuid_from_grpc(request.metadata()) {
		Err(e) => {
			trace_err("Unauthorized access attempt", e);
			Err(Status::permission_denied(""))
		}
		Ok(user) => {
			let args = request.into_inner();
			Ok((user, args))
		}
	}
}

impl YdbError {
	pub fn lift<R>(self, f: impl FnOnce(userstore::Status) -> R) -> Result<Response<R>, Status> {
		match self {
			Self::FieldNotFound => Ok(Response::new(f(userstore::Status::FieldNotFound))),
			Self::DatabaseError(_) => Err(Status::internal("")),
		}
	}
}