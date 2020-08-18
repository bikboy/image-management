extern crate rs_docker;
use clap::{Arg, App};
use rs_docker::Docker;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

fn main() {
    let matches = App::new("Docker image retention")
        .version("0.1.0")
        .author("Tim Bikbaev <t.bikbaev@gmail.com>")
        .about("Small util for specific images retention ")
        .arg(Arg::with_name("image")
                 .short("i")
                 .long("image")
                 .takes_value(true)
                 .help("Specify image name"))
        .arg(Arg::with_name("retention")
                 .short("r")
                 .long("retention")
                 .takes_value(true)
                 .help("Retention period in seconds"))
        .get_matches();
    let ret = match matches.value_of("retention") {
        None => "No retention set",
        Some(ret) => ret, 
    };
    let ret: u64 = ret.parse().unwrap();
    println!("{}",ret);
    
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
    let images = match docker.get_images(false) {
        Ok(images) => images,
        Err(e) => { panic!("{}", e); }
    };
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(now) => now.as_secs(),
        Err(_) => panic!("Time error!"),
    };
    for i in images {
        let delta = now - i.Created;
        //println!("{:?}", i);
        for r in i.RepoTags {
            let nametag : Vec<&str> = r.split(":").collect();
            if nametag[0].to_string() == matches.value_of("image").as_deref().unwrap_or("ubuntu") {
                println!("Age: {:?}", delta);
                println!("Tag: {:?}", nametag[1]);
                if delta > ret {
                    let statuses = match docker.delete_image(&r) {
                        Ok(statuses) => statuses,
                        Err(e) => { panic!("{}", e); }
                    };
                }

            }

            
        }
        /*let name : Vec<&str> = i.RepoTags[0].split(":").collect();
        if name[0].to_string() != matches.value_of("image").as_deref().unwrap_or("ubuntu") {
            println!("Image name: {:?}", name[0]);
            println!("Image age: {:?}", now - i.Created);
        }
        println!("Image name: {:?}", i.RepoTags);*/
    };

}
