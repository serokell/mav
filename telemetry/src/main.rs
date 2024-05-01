use std::{
    io::{self, Write},
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use log::{error, info};

use mav_sdk::{
    grpc::telemetry::{AttitudeQuaternionResponse, PositionResponse, TelemetryServiceClient},
    transport::Channel,
    Drone,
};

// use simple_logger::SimpleLogger;
use telemetry::{Entry, FileStore, Recorder};

#[tokio::main]
async fn main() {
    // init logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: flight_telemetry [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }
    let url = args.first().unwrap().to_string();

    let drone = Drone::connect(url.clone()).await.expect("Should connect");
    info!("We have a DRONE connection with {}", url);

    // use the same time for all logs files
    let time = Utc::now();

    tokio::spawn(run_quaternion_recording(drone.telemetry.clone(), time));
    tokio::spawn(run_position_recording(drone.telemetry.clone(), time));

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
}

/// Generates timestamped file name with the service name
fn get_file_path(service: &str, time: DateTime<Utc>) -> PathBuf {
    PathBuf::from(format!("./{}-{}.json", time, service))
}

async fn run_quaternion_recording(
    mut telemetry: TelemetryServiceClient<Channel>,
    time: DateTime<Utc>,
) -> anyhow::Result<()> {
    let file_path = get_file_path("quaternions", time);
    let recorder = FileStore::<AttitudeQuaternionResponse>::create(file_path).await?;

    let request = mav_sdk::grpc::telemetry::SubscribeAttitudeQuaternionRequest {};

    // let rate = telemetry.rate

    let mut response = telemetry.subscribe_attitude_quaternion(request).await?;

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response.get_mut().message().await? {
        // Log errors
        if let Err(err) = recorder.add(Entry::new(response)).await {
            error!("Recorder error: {}", err)
        }
    }

    Ok(())
}

async fn run_position_recording(
    mut telemetry: TelemetryServiceClient<Channel>,
    time: DateTime<Utc>,
) -> anyhow::Result<()> {
    let file_path = get_file_path("position", time);

    let recorder = FileStore::<PositionResponse>::create(file_path).await?;

    let request = mav_sdk::grpc::telemetry::SubscribePositionRequest {};

    let mut response = telemetry.subscribe_position(request).await?;

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response.get_mut().message().await? {
        // Log errors
        if let Err(err) = recorder.add(Entry::new(response)).await {
            error!("Recorder error: {}", err)
        }
    }

    Ok(())
}
