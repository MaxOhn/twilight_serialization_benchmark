use std::iter;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, SeedableRng};
use rust_serialization_benchmark::values::Values;
use surprise_me::{Surprise, SurpriseFactor};
use twilight_model::{channel::Message, guild::Guild};

#[allow(unused)]
use criterion::black_box;

#[cfg(feature = "abomonation")]
use rust_serialization_benchmark::bench_abomonation;
#[cfg(feature = "alkahest")]
use rust_serialization_benchmark::bench_alkahest;
#[cfg(feature = "serde_bare")]
use rust_serialization_benchmark::bench_bare;
#[cfg(feature = "bincode")]
use rust_serialization_benchmark::bench_bincode;
#[cfg(feature = "borsh")]
use rust_serialization_benchmark::bench_borsh;
#[cfg(feature = "bson")]
use rust_serialization_benchmark::bench_bson;
#[cfg(feature = "capnp")]
use rust_serialization_benchmark::bench_capnp;
#[cfg(feature = "serde_cbor")]
use rust_serialization_benchmark::bench_cbor;
#[cfg(feature = "dlhn")]
use rust_serialization_benchmark::bench_dlhn;
#[cfg(feature = "flatbuffers")]
use rust_serialization_benchmark::bench_flatbuffers;
#[cfg(feature = "nachricht-serde")]
use rust_serialization_benchmark::bench_nachricht;
#[cfg(feature = "postcard")]
use rust_serialization_benchmark::bench_postcard;
#[cfg(feature = "prost")]
use rust_serialization_benchmark::bench_prost;
#[cfg(feature = "rkyv")]
use rust_serialization_benchmark::bench_rkyv;
#[cfg(feature = "rmp-serde")]
use rust_serialization_benchmark::bench_rmp;
#[cfg(feature = "ron")]
use rust_serialization_benchmark::bench_ron;
#[cfg(feature = "scale")]
use rust_serialization_benchmark::bench_scale;
#[cfg(feature = "serde_json")]
use rust_serialization_benchmark::bench_serde_json;
#[cfg(feature = "simd-json")]
use rust_serialization_benchmark::bench_simd_json;
#[cfg(feature = "speedy")]
use rust_serialization_benchmark::bench_speedy;

macro_rules! bench_values {
    ($name:literal, $ty:ident, $count:literal, $criterion:ident) => {{
        // nothing up our sleeves, state and stream are first 10 digits of pi
        const STATE: u64 = 3141592653;

        let mut rng = SmallRng::seed_from_u64(STATE);

        let data = {
            let factor = SurpriseFactor::<$ty>::default();

            let values = iter::repeat_with(|| $ty::generate_with_factor(&mut rng, &factor))
                .take($count)
                .collect();

            Values { values }
        };

        #[cfg(feature = "borsh")]
        bench_borsh::bench($name, $criterion, &data);

        #[cfg(feature = "serde_cbor")]
        bench_cbor::bench($name, $criterion, &data);

        #[cfg(feature = "rkyv")]
        bench_rkyv::bench($name, $criterion, &data);

        #[cfg(feature = "scale")]
        bench_scale::bench($name, $criterion, &data);

        #[cfg(feature = "serde_json")]
        bench_serde_json::bench($name, $criterion, &data);

        #[cfg(feature = "simd-json")]
        bench_simd_json::bench($name, $criterion, &data);

        #[cfg(feature = "speedy")]
        bench_speedy::bench($name, $criterion, &data);
    }};
}

fn bench_twilight_guilds(c: &mut Criterion) {
    bench_values!("twilight_guilds", Guild, 23, c);
}

fn bench_twilight_messages(c: &mut Criterion) {
    bench_values!("twilight_messages", Message, 500, c);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_twilight_guilds(c);
    bench_twilight_messages(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
