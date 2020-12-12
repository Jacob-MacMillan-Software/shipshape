#[cfg(test)]
extern crate shipshape;

#[cfg(test)]
mod tests {
	use shipshape::create_container;

	#[actix_rt::test]
	async fn create_container_test() {
		
		match create_container(r#"{"Image": "alpine", "Cmd": ["echo", "hello world"]}"#.to_string()).await {
			Ok(id) => assert_eq!(id.chars().count(), 88),
			Err(err) => panic!("Error: {}", err),
		}
	}
}


