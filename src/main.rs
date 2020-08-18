extern crate rs_docker;
use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
    let images = match docker.get_images(false) {
        Ok(images) => images,
        Err(e) => { panic!("{}", e); }
    };

}
