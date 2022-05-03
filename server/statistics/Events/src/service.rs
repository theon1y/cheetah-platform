use std::time::Duration;

use tonic::{Code, Request, Response, Status};

use crate::loki::Loki;
use crate::proto;
use crate::proto::{EventRequest, EventResponse};

pub struct EventsService {
	loki: Loki,
	namespace: String,
}

impl EventsService {
	pub fn new(loki_url: &str, namespace: &str) -> Self {
		Self {
			loki: Loki::new(loki_url),
			namespace: namespace.to_owned(),
		}
	}
}

#[tonic::async_trait]
impl proto::events_server::Events for EventsService {
	async fn send_event(&self, request: Request<EventRequest>) -> Result<Response<EventResponse>, Status> {
		let request = request.into_inner();
		let mut labels = request.labels.clone();
		labels.insert("namespace".to_owned(), self.namespace.clone());
		labels.insert("source".to_owned(), "client".to_owned());
		self.loki
			.send_to_loki(labels, Duration::from_millis(request.time), request.value.as_str())
			.await
			.map(|_| Response::new(EventResponse {}))
			.map_err(|e| Status::new(Code::Internal, e))
	}
}