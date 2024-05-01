use std::io::{self, Write};

use mav_sdk::{grpc::telemetry::AttitudeEulerResponse, Drone};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: telemetry_subscription [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }
    let url = args.first().unwrap().to_string();

    let mut drone = Drone::connect(url.clone()).await.expect("Should connect");
    println!("We have a DRONE connection with {}", url);

    let subscribe_euler_request = mav_sdk::grpc::telemetry::SubscribeAttitudeEulerRequest {};
    let mut response = drone
        .telemetry
        .subscribe_attitude_euler(subscribe_euler_request)
        .await
        .expect("Should subscribe");

    while let Some(AttitudeEulerResponse { attitude_euler }) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        if let Some(attitude_euler) = attitude_euler {
            println!("Euler Angles! {:?}", attitude_euler)
        }
    }
}
