<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

A fork of djkoloski's [rust_serialization_benchmark](https://github.com/djkoloski/rust_serialization_benchmark).
The goal of these benchmarks is to provide benchmarks for various rust serialization frameworks on [twilight](https://github.com/twilight-rs/twilight) types.

## Format

All tests benchmark the following properties (time or size):

* **Serialize**: serialize data into a buffer
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2023-3-18

## `channels`

750-ish [channels](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/channel/mod.rs#L57)

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 404.70 µs | 580.45 µs | 194537 | 155969 | 148081 |
| cbor | 1.6627 ms | 2.1005 ms | 472208 | 213341 | 191095 |
| rkyv | 186.20 µs | <span title="unvalidated">*301.28 µs\**</span> <span title="validated upfront with error">*527.11 µs\**</span> | 799160 | 181549 | 171739 |
| scale | 407.25 µs | 702.86 µs | 185404 | 153020 | 146289 |
| serde_json | 2.8789 ms | 3.4340 ms | 590964 | 237502 | 192566 |
| simd-json | 2.7367 ms | 2.2112 ms | 590964 | 237502 | 192566 |
| speedy | 244.32 µs | 481.20 µs | 194537 | 155969 | 148081 |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 46.01% | 51.90% | 95.31% | 98.11% | 98.79% |
| cbor | 11.20% | 14.34% | 39.26% | 71.73% | 76.55% |
| rkyv | 100.00% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*57.16%\**</span> | 23.20% | 84.29% | 85.18% |
| scale | 45.72% | 42.86% | 100.00% | 100.00% | 100.00% |
| serde_json | 6.47% | 8.77% | 31.37% | 64.43% | 75.97% |
| simd-json | 6.80% | 13.63% | 31.37% | 64.43% | 75.97% |
| speedy | 76.21% | 62.61% | 95.31% | 98.11% | 98.79% |

## `members`

1000-ish [members](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/guild/member.rs#L16)

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 267.85 µs | 467.24 µs | 229770 | 198582 | 189898 |
| cbor | 2.8936 ms | 2.2773 ms | 565352 | 263608 | 239022 |
| rkyv | 86.851 µs | <span title="unvalidated">*237.32 µs\**</span> <span title="validated upfront with error">*390.04 µs\**</span> | 360872 | 212922 | 201307 |
| scale | 277.97 µs | 490.71 µs | 221764 | 195056 | 187815 |
| serde_json | 3.8721 ms | 3.3908 ms | 677676 | 277807 | 240628 |
| simd-json | 3.4712 ms | 2.1274 ms | 677676 | 277807 | 240628 |
| speedy | 119.15 µs | 375.33 µs | 229770 | 198582 | 189898 |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 32.43% | 50.79% | 96.52% | 98.22% | 98.90% |
| cbor | 3.00% | 10.42% | 39.23% | 73.99% | 78.58% |
| rkyv | 100.00% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*60.85%\**</span> | 61.45% | 91.61% | 93.30% |
| scale | 31.24% | 48.36% | 100.00% | 100.00% | 100.00% |
| serde_json | 2.24% | 7.00% | 32.72% | 70.21% | 78.05% |
| simd-json | 2.50% | 11.16% | 32.72% | 70.21% | 78.05% |
| speedy | 72.89% | 63.23% | 96.52% | 98.22% | 98.90% |

## `roles`

2500-ish [roles](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/guild/role.rs#L11)

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 39.538 µs | 99.143 µs | 28675 | 24148 | 22986 |
| cbor | 185.71 µs | 314.57 µs | 74423 | 33198 | 30345 |
| rkyv | 12.183 µs | <span title="unvalidated">*44.391 µs\**</span> <span title="validated upfront with error">*83.616 µs\**</span> | 60000 | 27476 | 26260 |
| scale | 37.654 µs | 97.323 µs | 26423 | 23508 | 22734 |
| serde_json | 330.36 µs | 436.74 µs | 104652 | 37044 | 30382 |
| simd-json | 301.93 µs | 288.85 µs | 104652 | 37044 | 30382 |
| speedy | 10.264 µs | 68.129 µs | 28675 | 24148 | 22986 |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 25.96% | 44.77% | 92.15% | 97.35% | 98.90% |
| cbor | 5.53% | 14.11% | 35.50% | 70.81% | 74.92% |
| rkyv | 84.25% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*53.09%\**</span> | 44.04% | 85.56% | 86.57% |
| scale | 27.26% | 45.61% | 100.00% | 100.00% | 100.00% |
| serde_json | 3.11% | 10.16% | 25.25% | 63.46% | 74.83% |
| simd-json | 3.40% | 15.37% | 25.25% | 63.46% | 74.83% |
| speedy | 100.00% | 65.16% | 92.15% | 97.35% | 98.90% |

## `guilds`

24-ish [guilds](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/guild/mod.rs#L78) (huge type, contains everything but messages)

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 18.825 ms | 57.921 ms | 9659272 | 7830265 | 7462689 |
| cbor | 67.808 ms | 114.46 ms | 17901405 | 9583586 | 8791955 |
| rkyv | 18.077 ms | <span title="unvalidated">*33.415 ms\**</span> <span title="validated upfront with error">*50.434 ms\**</span> | 20606264 | 9347965 | 8505628 |
| scale | 19.628 ms | 46.108 ms | 8670942 | 7495629 | 7227599 |
| serde_json | 103.15 ms | 159.45 ms | 22283724 | 10238346 | 8763902 |
| simd-json | 95.978 ms | 109.34 ms | 22283724 | 10238346 | 8763902 |
| speedy | 15.905 ms | 38.092 ms | 9659272 | 7830276 | 7462731 |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 84.49% | 57.69% | 89.77% | 95.73% | 96.85% |
| cbor | 23.46% | 29.19% | 48.44% | 78.21% | 82.21% |
| rkyv | 87.98% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*66.25%\**</span> | 42.08% | 80.18% | 84.97% |
| scale | 81.03% | 72.47% | 100.00% | 100.00% | 100.00% |
| serde_json | 15.42% | 20.96% | 38.91% | 73.21% | 82.47% |
| simd-json | 16.57% | 30.56% | 38.91% | 73.21% | 82.47% |
| speedy | 100.00% | 87.72% | 89.77% | 95.73% | 96.85% |

## `messages`

500-ish [messages](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/channel/message/mod.rs#L52) (without components)

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 973.05 µs | 2.7497 ms | 675021 | 580780 | 571019 |
| cbor | 4.7447 ms | 8.8227 ms | 1241348 | 735550 | 685829 |
| rkyv | 1.0052 ms | <span title="unvalidated">*1.7846 ms\**</span> <span title="validated upfront with error">*2.8240 ms\**</span> | 1922120 | 659305 | 627630 |
| scale | 1.0820 ms | 2.8080 ms | 623876 | 562322 | 555842 |
| serde_json | 7.2285 ms | 11.106 ms | 1504073 | 787539 | 686368 |
| simd-json | 7.0921 ms | 6.5365 ms | 1504073 | 787539 | 686368 |
| speedy | 696.69 µs | 2.4226 ms | 675021 | 580795 | 571039 |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| borsh | 71.60% | 64.90% | 92.42% | 96.82% | 97.34% |
| cbor | 14.68% | 20.23% | 50.26% | 76.45% | 81.05% |
| rkyv | 69.31% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*63.19%\**</span> | 32.46% | 85.29% | 88.56% |
| scale | 64.39% | 63.55% | 100.00% | 100.00% | 100.00% |
| serde_json | 9.64% | 16.07% | 41.48% | 71.40% | 80.98% |
| simd-json | 9.82% | 27.30% | 41.48% | 71.40% | 80.98% |
| speedy | 100.00% | 73.66% | 92.42% | 96.82% | 97.34% |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
