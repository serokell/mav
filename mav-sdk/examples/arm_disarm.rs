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
            .write_all(b"Usage: arm_disarm [connection_url]\n")
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

    arm_disarm(drone).await;
}

async fn arm_disarm(mut drone: Drone) {
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

    sleep(Duration::from_secs(2));

    print!("Disarming drone... ");

    let disarm_request = mav_sdk::grpc::action::DisarmRequest {};
    let disarm_response = drone.action.disarm(disarm_request).await.unwrap();

    println!(
        "{}",
        &disarm_response
            .get_ref()
            .action_result
            .as_ref()
            .unwrap()
            .result_str
    );
}
