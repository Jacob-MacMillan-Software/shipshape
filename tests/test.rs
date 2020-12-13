#[cfg(test)]
extern crate shipshape;

#[cfg(test)]
mod tests {
	use shipshape::*;

	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	#[allow(non_snake_case)]
	struct ContainerCreateReturn {
		Id: String,
		Warnings: Vec<String>
	}


	#[actix_rt::test]
	async fn create_container_test() {
		match create_container(r#"{"Image": "alpine", "Cmd": ["echo", "hello world"]}"#.to_string()).await {
			Ok(val) => {
				let return_json: ContainerCreateReturn	= serde_json::from_str(&val[..]).unwrap();
				let id = return_json.Id;

				//The id will be different every time, so this is the only real way we can check that
				//it returned correctly
				assert_eq!(id.chars().count(), 64);
			}
			Err(err) => panic!("Error craeting container: {}", err),
		}
	}

	#[actix_rt::test]
	async fn start_container_test() {
		match create_container(r#"{"Image": "alpine", "Cmd": ["echo", "hello world"], "AutoRemove": true}"#.to_string()).await {
			Ok(val) => {
				let return_json: ContainerCreateReturn	= serde_json::from_str(&val[..]).unwrap();

				//Try to start the container
				match start_container(return_json.Id).await {
					Ok(val) => {
						//If the container started successfully, it should return an empty string
						assert_eq!(val, "".to_string());
					}
					Err(err) => panic!("Error starting container: {}", err),
				}

			}
			Err(err) => panic!("Error creating container: {}", err),
		}
	}

	#[actix_rt::test]
	async fn stop_container_test() {
		//Create a container that opens a shell and hangs (so that it doesn't stop on it's own)
		match create_container(r#"{"Image": "alpine", "Cmd": ["sh"], "AutoRemove": true}"#.to_string()).await {
			Ok(val) => {
				let return_json: ContainerCreateReturn	= serde_json::from_str(&val[..]).unwrap();
				let id: &str = &return_json.Id[..];

				//Try to start the container
				match start_container(id.to_string()).await {
					Ok(val) => {
						//If the container started successfully, it should return an empty string
						assert_eq!(val, "".to_string());

						//Stop container
						match stop_container(id.to_string()).await {
							Ok(val) => {
								//If the container stopped successfully, it should return an empty string
								assert_eq!(val, "".to_string());
							}
							Err(err) => panic!("Error stopping container: {}", err),
						}
					}
					Err(err) => panic!("Error starting container: {}", err),
				}

			}
			Err(err) => panic!("Error creating container: {}", err),
		}
	}

	#[actix_rt::test]
	async fn pause_container_test() {
		match create_container(r#"{"Image": "alpine", "Cmd": ["echo", "hello world"], "AutoRemove": true}"#.to_string()).await {
			Ok(val) => {
				let return_json: ContainerCreateReturn	= serde_json::from_str(&val[..]).unwrap();
				let id: &str = &return_json.Id[..];

				//Pause a stopped container (returns a specific error message)
				match pause_container(id.to_string()).await {
					Ok(val) => {
						//If the container stopped successfully, it should return an empty string
						assert_eq!(val, format!("{}{}{}\n", r#"{"message":"Container "#, id, r#" is not running"}"#).to_string());
					}
					Err(err) => panic!("Error unpause container: {}", err),
				}
			}
			Err(err) => panic!("Error creating container: {}", err),
		}
	}

	#[actix_rt::test]
	async fn unpause_container_test() {
		//Create a container that sleeps for 4h (so that it doesn't stop on it's own)
		match create_container(r#"{"Image": "alpine", "Cmd": ["sleep", "4h"], "AutoRemove": true}"#.to_string()).await {
			Ok(val) => {
				let return_json: ContainerCreateReturn	= serde_json::from_str(&val[..]).unwrap();
				let id: &str = &return_json.Id[..];

				//Try to start the container
				match start_container(id.to_string()).await {
					Ok(val) => {
						//If the container started successfully, it should return an empty string
						assert_eq!(val, "".to_string());

						//Unpause container without pausing first (should throw a very specific error)
						match unpause_container(id.to_string()).await {
							Ok(val) => {
								//If the container stopped successfully, it should return an empty string
								assert_eq!(val, format!("{}{}{}\n", r#"{"message":"Container "#, id, r#" is not paused"}"#).to_string());
							}
							Err(err) => panic!("Error unpause container: {}", err),
						}
					}
					Err(err) => panic!("Error starting container: {}", err),
				}

			}
			Err(err) => panic!("Error creating container: {}", err),
		}
	}
}


