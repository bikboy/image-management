use docker::Docker

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => { panic!("something goes wrong: {}", e); }
    }
}
