use docker_api::{Docker, VolumeCreateOptions};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    let docker = Docker::new("tcp://127.0.0.1:80").unwrap();

    let volume_name = env::args()
        .nth(1)
        .expect("You need to specify an volume name");

    let mut labels = HashMap::new();
    labels.insert("com.github", "docker_api");

    match docker
        .volumes()
        .create(
            &VolumeCreateOptions::builder()
                .name(&volume_name)
                .labels(&labels)
                .build(),
        )
        .await
    {
        Ok(info) => println!("{:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }
}
