#[cfg(feature = "abomonation")]
pub mod bench_abomonation;
#[cfg(feature = "alkahest")]
pub mod bench_alkahest;
#[cfg(feature = "serde_bare")]
pub mod bench_bare;
#[cfg(feature = "bincode")]
pub mod bench_bincode;
#[cfg(feature = "borsh")]
pub mod bench_borsh;
#[cfg(feature = "bson")]
pub mod bench_bson;
#[cfg(feature = "capnp")]
pub mod bench_capnp;
#[cfg(feature = "serde_cbor")]
pub mod bench_cbor;
#[cfg(feature = "dlhn")]
pub mod bench_dlhn;
#[cfg(feature = "flatbuffers")]
pub mod bench_flatbuffers;
#[cfg(feature = "nachricht-serde")]
pub mod bench_nachricht;
#[cfg(feature = "postcard")]
pub mod bench_postcard;
#[cfg(feature = "prost")]
pub mod bench_prost;
#[cfg(feature = "rkyv")]
pub mod bench_rkyv;
#[cfg(feature = "rlp")]
pub mod bench_rlp;
#[cfg(feature = "rmp-serde")]
pub mod bench_rmp;
#[cfg(feature = "ron")]
pub mod bench_ron;
#[cfg(feature = "scale")]
pub mod bench_scale;
#[cfg(feature = "serde_json")]
pub mod bench_serde_json;
#[cfg(feature = "simd-json")]
pub mod bench_simd_json;
#[cfg(feature = "speedy")]
pub mod bench_speedy;

pub mod values;

pub fn bench_size(name: &str, lib: &str, bytes: &[u8]) {
    println!("{}/{}/size {}", name, lib, bytes.len());
    println!("{}/{}/zlib {}", name, lib, zlib_size(bytes));
    println!("{}/{}/zstd {}", name, lib, zstd_size(bytes));
}

fn zlib_size(mut bytes: &[u8]) -> usize {
    let mut encoder = libflate::zlib::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut bytes, &mut encoder).unwrap();
    encoder.finish().into_result().unwrap().len()
}

fn zstd_size(bytes: &[u8]) -> usize {
    zstd::stream::encode_all(bytes, 0).unwrap().len()
}
