mod fetch;
mod reply;
mod request;
mod update;
mod userstore {
	tonic::include_proto!("cheetah.userstore");
}

use cheetah_libraries_microservice::jwt::grpc::get_user_uuid;
use cheetah_libraries_microservice::{init, trace::trace};
use std::{error::Error, net::SocketAddr};
use tonic::Response;
use tonic::{transport::Server, Request, Status};
use tonic_web;
use update::UpdateService;
use userstore::update_server::UpdateServer;
use userstore::Status as UserStoreStatus;
use uuid::Uuid;
use ydb::Client;

use crate::ydb::Error as YdbError;

use self::{fetch::FetchService, userstore::fetch_server::FetchServer};

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

fn unwrap_request<T>(request: Request<T>, jwt_public_key: String) -> Result<(Uuid, T), Status> {
	match get_user_uuid(request.metadata(), jwt_public_key) {
		Err(e) => {
			trace("Unauthorized access attempt", e);
			Err(Status::permission_denied(""))
		}
		Ok(user) => {
			let args = request.into_inner();
			Ok((user, args))
		}
	}
}

impl YdbError {
	pub fn lift<R>(self, f: impl FnOnce(UserStoreStatus) -> R) -> Result<Response<R>, Status> {
		match self {
			Self::FieldNotFound => Ok(Response::new(f(UserStoreStatus::FieldNotFound))),
			Self::DatabaseError(_) => Err(Status::internal("")),
		}
	}
}