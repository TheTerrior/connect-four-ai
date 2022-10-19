use wgpu::{Instance, Backends};

fn main() {
    let instances = Instance::new(Backends::all());
    for adapter in instances.enumerate_adapters(Backends::all()) {
        println!("{:?}", adapter.get_info());
    }
}
