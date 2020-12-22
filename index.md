## Shipshape

A simple library for interacting with the Docker API through Rust.

### Install

If you want to use shipshape in your project, simply include the following in your Cargo.toml:
```
[dependencies]
shipshape = "0.1.1"
```

### Usage
| Supported actions | Function | API Call |
|---------------|----------------|-------------|
| Creating a container from an image | `create_container(String)` | `/containers/create` |
| Starting a container | `start_container(String)` | `/containers/{id}/start` |
| Stoping a container | `stop_container(String)` | `/containers/{id}/stop` |
| Pausing a container | `pause_container(String)` | `/containers/{id}/pause` |
| Unpausing a container | `unpause_container(String)` | `/containers/{id}/unpause` |

The return type of every function is `Result<String, Box<dyn std::error::Error + Send + Sync>>`

Every function takes a single parameter. `create_container` takes a JSON formated string, which is sent directly to the Docker API. This is detailed [here](https://docs.docker.com/engine/api/v1.41/#operation/ContainerCreate). The other functions take a string containing the container ID to operate on.

As the goal is to make this tool as simple as possible, the output from each function is the exact string returned by the API call, and API errors are not handled.

### Examples

#### Create and start a container

This example uses [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) to extract the container ID from the string returned by `create_container`. This example also uses [tokio](https://crates.io/crates/tokio) to allow `main` to be async.  
Running this example requires the `alpine` Docker image downloaded. To download this image, simply run
```bash
docker pull alpine
```

```rust
extern crate shipshape;

use shipshape::{create_container, start_container};

#[dervie(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ContainerCreateReturn {
  Id: String,
  Warnings: Vec<String>
}

#[tokio::main]
async fn main() {
  //Create a container using the "alpine" image
  match create_container(r#"{"Image": "alpine", "Cmd": ["echo", "hello world"], "AutoRemove": true"#.to_string()).await {
    Ok(val) => {
      //Convert the returned, JSON formated, String to a struct for easy parsing
      let return_json: ContainerCreateReturn = serde_json::from_str(&val[..]).unwrap();
      
      //Start the container
      match start_container(return_json.Id).await {
        Ok(val) => {
          //If val is empty, the container has started
          assert_eq(val, "".to_string());
        }
        Err(err) => eprintln!("Error: {}", err),
      }
    }
    Err(err) => eprintln!("Error: {}", err),
  }
}

```

### More documentation

You can find more documentation on [docs.rs](https://docs.rs/shipshape/0.1.1/shipshape/), and on [crates.io](https://crates.io/crates/shipshape).
