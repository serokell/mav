use std::io::{self, Write};

use mav_sdk::{grpc::telemetry::TelemetryServiceClient, Drone};
use tonic::transport::Channel;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: flight_telemetry [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }
    let url = args.first().unwrap().to_string();

    let drone = Drone::connect(url.clone()).await.expect("Should connect");
    println!("We have a DRONE connection with {}", url);

    tokio::spawn(print_quaternion(drone.telemetry.clone()));
    tokio::spawn(print_position(drone.telemetry.clone()));

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
}

async fn print_quaternion(mut telemetry: TelemetryServiceClient<Channel>) {
    let request = mav_sdk::grpc::telemetry::SubscribeAttitudeQuaternionRequest {};

    let mut response = telemetry
        .subscribe_attitude_quaternion(request)
        .await
        .expect("Should subscribe");

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        let json = serde_json::to_string(&response).expect("Should serialize");
        println!("{}", json);
    }
}

async fn print_position(mut telemetry: TelemetryServiceClient<Channel>) {
    let request = mav_sdk::grpc::telemetry::SubscribePositionRequest {};

    let mut response = telemetry
        .subscribe_position(request)
        .await
        .expect("Should subscribe");

    while let Some(response) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        let json = serde_json::to_string(&response).expect("Should serialize");
        println!("{}", json);
    }
}
