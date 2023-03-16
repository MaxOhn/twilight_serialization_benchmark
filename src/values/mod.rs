#[derive(Clone)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Values<T> {
    pub values: Vec<T>,
}

#[cfg(feature = "rkyv")]
const _: () = {
    use core::pin::Pin;
    use rkyv::{Archive, Archived};

    impl<T: Archive> ArchivedValues<T> {
        pub fn values_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<T>>> {
            unsafe { self.map_unchecked_mut(|s| &mut s.values) }
        }
    }
};
