//! Shipshape
//! 
//! A library for interacting with the Docker API from Rust
//! Meant to be as simple as possible

use std::error::Error;

use hyper::{body, Body, Method, Request};
use hyper_socket::UnixSocketConnector;

///Socet to connect to
static DOCKERSOCKET : &'static str = "/var/run/docker.sock";

///Make HTTP requests to Docker
async fn make_http_request(endpoint: String, body_str: String) -> Result<String, Box<dyn Error + Send + Sync>> {
	let socket = UnixSocketConnector::new(DOCKERSOCKET);
	let client = hyper::Client::builder()
		.build::<_, Body>(socket);

	//Build request
	let req = Request::builder()
		.method(Method::POST)
		.header("content-type", "application/json")
		.uri(endpoint)
		.body(Body::from(body_str))?;

	let mut response_str: String = "".to_string();

	//Send request
	match client.request(req).await {
		Ok(res) => {
			let response_bytes = body::to_bytes(res.into_body()).await?;
			response_str = String::from_utf8(response_bytes.to_vec()).expect("response was not valid utf-8");
		}
		Err(err) => eprintln!("shipshape Error: {}", err),
	}

	Ok(response_str)
}


///Create a docker container from an image
///Must specify the full JSON encoded body to send to the API
///See <https://docs.docker.com/engine/api/v1.41/#operation/ContainerCreate> for complete list of
///valid options
pub fn create_container(body_str: String) -> impl futures::Future< Output = Result<String, Box<dyn Error + Send + Sync>>> {
	make_http_request("http://localhost/containers/create".to_string(), body_str)
}

///Start a docker container from container ID
pub fn start_container(container_id: String) -> impl futures::Future< Output = Result<String, Box<dyn Error + Send + Sync>>> {
	make_http_request(format!("http://localhost/containers/{}/start", container_id), "".to_string())
}

///Stop a docker container from container ID
pub fn stop_container(container_id: String) -> impl futures::Future< Output = Result<String, Box<dyn Error + Send + Sync>>> {
	make_http_request(format!("http://localhost/containers/{}/stop", container_id), "".to_string())
}

///Pause a docker container from container ID
pub fn pause_container(container_id: String) -> impl futures::Future< Output = Result<String, Box<dyn Error + Send + Sync>>> {
	make_http_request(format!("http://localhost/containers/{}/pause", container_id), "".to_string())
}


///Unpause a docker container from container ID
pub fn unpause_container(container_id: String) -> impl futures::Future< Output = Result<String, Box<dyn Error + Send + Sync>>> {
	make_http_request(format!("http://localhost/containers/{}/unpause", container_id), "".to_string())
}
