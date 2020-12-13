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
			println!("Response: {}", response_str);
		}
		Err(err) => eprintln!("Docker-rust Error: {}", err),
	}

	Ok(response_str)
}


///Create a docker container from an image
///Must specify the full JSON encoded body to send to the API
///See <https://docs.docker.com/engine/api/v1.41/#operation/ContainerCreate> for complete list of
///valid options
pub async fn create_container(body_str: String) -> Result<String, Box<dyn Error + Send + Sync>> {
	let mut response_str: String = "".to_string();

	let endpoint = format!("http://localhost/containers/create");

	match make_http_request(endpoint, body_str).await {
		Ok(resp) => response_str = resp,
		Err(err) => eprintln!("Docker-rust http request error: {}", err),
	}

	Ok(response_str)
}

///Start a docker container from container ID
pub async fn start_container(container_id: String) -> Result<String, Box<dyn Error + Send + Sync>> {
	let mut response_str: String = "".to_string();

	let endpoint = format!("http://localhost/containers/{}/start", container_id);

	match make_http_request(endpoint, "".to_string()).await {
		Ok(resp) => response_str = resp,
		Err(err) => eprintln!("Docker-rust http request error: {}", err),
	}

	Ok(response_str)
}

///Stop a docker container from container ID
pub async fn stop_container(container_id: String) -> Result<String, Box<dyn Error + Send + Sync>> {
	let mut response_str: String = "".to_string();

	let endpoint = format!("http://localhost/containers/{}/stop", container_id);

	match make_http_request(endpoint, "".to_string()).await {
		Ok(resp) => response_str = resp,
		Err(err) => eprintln!("Docker-rust http request error: {}", err),
	}

	Ok(response_str)
}

///Pause a docker container from container ID
pub async fn pause_container(container_id: String) -> Result<String, Box<dyn Error + Send + Sync>> {
	let mut response_str: String = "".to_string();

	let endpoint = format!("http://localhost/containers/{}/pause", container_id);

	match make_http_request(endpoint, "".to_string()).await {
		Ok(resp) => response_str = resp,
		Err(err) => eprintln!("Docker-rust http request error: {}", err),
	}

	Ok(response_str)
}


///Unpause a docker container from container ID
pub async fn unpause_container(container_id: String) -> Result<String, Box<dyn Error + Send + Sync>> {
	let mut response_str: String = "".to_string();

	let endpoint = format!("http://localhost/containers/{}/unpause", container_id);

	match make_http_request(endpoint, "".to_string()).await {
		Ok(resp) => response_str = resp,
		Err(err) => eprintln!("Docker-rust http request error: {}", err),
	}

	Ok(response_str)
}
