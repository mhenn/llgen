use std::{process::{Command, Child}, error::Error};

use crate::init::{write_to_file };

pub fn docker_start(){
    let output = Command::new("sh")
        .arg("-c")
        .arg("docker run --net ref1 --ip 172.18.0.22  --rm refbox-fast")
        .spawn();
}

pub fn docker_kill_all(){
    let output = Command::new("sh")
        .arg("-c")
        .arg("docker container kill $(docker container ls -aq)")
        .output();
}

pub fn execute_BT() -> Result<Child, std::io::Error>  {

    Command::new("sh")
        .arg("-c")
        .arg("../../behavior/build/BTRCLL")
        .spawn()
}

pub fn kill_BT(){
    Command::new("sh")
        .arg("-c")
        .arg("killall BTRCLL")
        .output();
}

pub fn write_result(path: String){

    let output = Command::new("sh")
        .arg("-c")
        .arg("./getResults.bash")
        .output();
    let out : String = String::from_utf8(output.unwrap().stdout).unwrap();
    write_to_file(out, path);
}

pub fn docker_prune(){

    let output = Command::new("sh")
        .arg("-c")
        .arg("docker container prune --force")
        .spawn();
}

pub fn docker_copy(){

    let output = Command::new("sh")
        .arg("-c")
        .arg("docker cp ./kill.bash $(docker container ps -aq):/kill.bash")
        .spawn();
}

pub fn stop_refbox(){

    let output = Command::new("sh")
        .arg("-c")
        .arg("docker exec -it $(docker container ls -aq) bash kill.bash")
        .output();
}



pub fn execute_cmd_test(){
    let output = Command::new("sh")
        .arg("-c")
        .arg("dir")
        .output();
    println!("{:?}",output.unwrap());
}

pub fn execute_bt_test(){
    let out = execute_BT();
    println!("{:?}", out);
    assert!(false)
}



#[test]
pub fn execute_docker_test(){
}
