use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use mav_sdk::Drone;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        io::stderr()
            .write_all(b"Usage: takeoff [connection_url]\n")
            .unwrap();
        std::process::exit(1);
    }
    let url = args.first().unwrap().to_string();

    let mut drone = Drone::connect(url.clone()).await.expect("Should connect");
    println!("We have a DRONE connection with {}", url);

    let get_version = mav_sdk::grpc::info::GetVersionRequest {};
    let version_response = drone.info.get_version(get_version).await.unwrap();
    let version = version_response.get_ref().version.as_ref().unwrap();

    println!(
        "We have PX4 version: v{}.{}",
        version.flight_sw_major, version.flight_sw_minor
    );

    let identification_request = mav_sdk::grpc::info::GetIdentificationRequest {};
    let identification_response = drone
        .info
        .get_identification(identification_request)
        .await
        .unwrap();
    let identification = &identification_response
        .get_ref()
        .identification
        .as_ref()
        .unwrap()
        .hardware_uid;
    println!("We have a hardware Identification: {}", identification);

    flight(drone).await;
}

async fn flight(mut drone: Drone) {
    print!("Arming drone... ");

    let arm_request = mav_sdk::grpc::action::ArmRequest {};
    let arm_response = drone.action.arm(arm_request).await.unwrap();

    println!(
        "{}",
        &arm_response
            .get_ref()
            .action_result
            .as_ref()
            .unwrap()
            .result_str
    );

    let takeoff_request = mav_sdk::grpc::action::TakeoffRequest {};
    let takeoff_response = drone.action.takeoff(takeoff_request).await.unwrap();

    println!(
        "Takeoff: {}",
        &takeoff_response
            .get_ref()
            .action_result
            .as_ref()
            .unwrap()
            .result_str
    );

    sleep(Duration::from_secs(5));
    println!("Telemetry - Subscribe for positions");

    let telemetry_request = mav_sdk::grpc::telemetry::SubscribePositionRequest {};
    let mut telemetry_streaming = drone
        .telemetry
        .subscribe_position(telemetry_request)
        .await
        .unwrap()
        .into_inner();

    println!("Telemetry - Do we have a Position update?");

    if let Some(next_message) = telemetry_streaming.message().await.unwrap() {
        println!("{:?}", next_message);
    }

    sleep(Duration::from_secs(5));

    let land_request = mav_sdk::grpc::action::LandRequest {};
    let land_response = drone.action.land(land_request).await.unwrap();

    println!(
        "Landing: {}",
        &land_response
            .get_ref()
            .action_result
            .as_ref()
            .unwrap()
            .result_str
    );

    println!("the eagle has (hopefully) landed!");
}
