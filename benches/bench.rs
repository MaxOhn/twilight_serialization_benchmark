use std::iter;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, SeedableRng};
use rust_serialization_benchmark::values::Values;
use surprise_me::{factors::VecSurprise, Surprise};
use twilight_model::{
    channel::{message::MessageSurprise, Message},
    guild::{Guild, GuildSurprise},
};

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

fn bench_twilight_guilds(c: &mut Criterion) {
    const BENCH: &str = "twilight_guilds";

    // nothing up our sleeves, state and stream are first 10 digits of pi
    const STATE: u64 = 3141592653;

    let mut rng = SmallRng::seed_from_u64(STATE);

    const COUNT: usize = 23;

    let guilds: Vec<Guild> = {
        let factor = GuildSurprise::default();

        iter::repeat_with(|| Guild::generate_with_factor(&mut rng, &factor))
            .take(COUNT)
            .collect()
    };

    let data = Values { values: guilds };

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "serde_cbor")]
    bench_cbor::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |data| {
            for guilds in data.values.iter() {
                black_box(&guilds.id);
            }
        },
        |mut guilds| {
            for i in 0..guilds.as_ref().values.len() {
                let mut guild = guilds.as_mut().values_pin().index_pin(i);
                let mut id = unsafe { guild.as_mut().map_unchecked_mut(|s| &mut s.id) };
                *id = twilight_model::id::Id::new(i as u64 + 1);
            }
        },
    );

    #[cfg(feature = "scale")]
    bench_scale::bench(BENCH, c, &data);

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "simd-json")]
    bench_simd_json::bench(BENCH, c, &data);

    #[cfg(feature = "speedy")]
    bench_speedy::bench(BENCH, c, &data);
}

fn bench_twilight_messages(c: &mut Criterion) {
    const BENCH: &str = "twilight_messages";

    // nothing up our sleeves, state and stream are first 10 digits of pi
    const STATE: u64 = 3141592653;

    let mut rng = SmallRng::seed_from_u64(STATE);

    const COUNT: usize = 500;

    let msgs: Vec<Message> = {
        let factor = MessageSurprise {
            components: VecSurprise {
                max_len: 0,
                ..Default::default()
            },
            ..Default::default()
        };

        iter::repeat_with(|| Message::generate_with_factor(&mut rng, &factor))
            .take(COUNT)
            .map(|mut msg| {
                if let Some(ref mut msg) = msg.referenced_message {
                    msg.components.clear();
                }

                msg
            })
            .collect()
    };

    let data = Values { values: msgs };

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "serde_cbor")]
    bench_cbor::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |data| {
            for msg in data.values.iter() {
                black_box(&msg.id);
            }
        },
        |mut msgs| {
            for i in 0..msgs.as_ref().values.len() {
                let mut msg = msgs.as_mut().values_pin().index_pin(i);
                let mut id = unsafe { msg.as_mut().map_unchecked_mut(|s| &mut s.id) };
                *id = twilight_model::id::Id::new(i as u64 + 1);
            }
        },
    );

    #[cfg(feature = "scale")]
    bench_scale::bench(BENCH, c, &data);

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "simd-json")]
    bench_simd_json::bench(BENCH, c, &data);

    #[cfg(feature = "speedy")]
    bench_speedy::bench(BENCH, c, &data);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_twilight_guilds(c);
    bench_twilight_messages(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
