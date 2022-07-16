use raytracer::run;
mod algorithm;

fn main() {
    pollster::block_on(run(Box::new(algorithm::Raytracer::new())));
}
