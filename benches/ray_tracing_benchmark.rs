use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use ray_tracing::{ray_tracing::ray_color, scenes::*};

pub fn benchmark(
    world: &ObjectList,
    background: Vec3,
    cam: &Camera,
    lights: &Option<ObjectList>,
    rng: &mut SmallRng,
) {
    let ray = cam.get_ray(rng.gen::<f64>(), rng.gen::<f64>(), rng);
    ray_color::<ObjectList>(&ray, &world, background, &lights, 50, rng);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let (world, background, cam, lights) = random_scene_light_static(1.0);
    c.bench_function("ray tracing", |b| {
        b.iter(|| benchmark(&*world, background, &*cam, &*lights, &mut rng))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10000);
    targets = criterion_benchmark
);
criterion_main!(benches);
