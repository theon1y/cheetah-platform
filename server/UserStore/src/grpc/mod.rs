mod fetch;
mod update;
mod userstore {
	tonic::include_proto!("cheetah.userstore.external");
}

use cheetah_libraries_microservice::jwt::grpc::get_user_uuid;
use std::{error::Error, net::SocketAddr};
use tonic::{transport::Server, Request, Status};
use tonic_web;
use update::UpdateService;
use userstore::update_server::UpdateServer;
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
		let update_service =
			UpdateService::new(self.ydb_client.table_client(), self.jwt_public_key.clone());
		let fetch_service =
			FetchService::new(self.ydb_client.table_client(), self.jwt_public_key.clone());

		Server::builder()
			.accept_http1(true)
			.add_service(tonic_web::enable(UpdateServer::new(update_service)))
			.add_service(tonic_web::enable(FetchServer::new(fetch_service)))
			.serve(addr)
			.await?;

		Ok(())
	}
}

fn unwrap_request<T>(request: Request<T>, jwt_public_key: String) -> Result<(Uuid, T), Status> {
	match get_user_uuid(request.metadata(), jwt_public_key) {
		Err(_) => Err(Status::unknown("")),
		Ok(user) => {
			let args = request.into_inner();
			Ok((user, args))
		}
	}
}

impl YdbError {
	pub fn to_status(&self, field_name: &str) -> Status {
		match self {
			Self::NoSuchField => Status::unavailable(format!(
				"The requested field ({}) cannot be found",
				field_name
			)),
			Self::DatabaseError(_) => Status::unknown(""),
		}
	}
}
