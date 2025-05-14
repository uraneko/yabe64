#![cfg(feature = "base45")]
use crate::makura_alloc::Vec;
use crate::makura_core::ops;

pub(crate) struct BaseTransformer {
    base: u8,
    value: u64,
    seq: Vec<u8>,
}

impl BaseTransformer {
    pub(crate) fn new<T: ops::Shl + ops::MulAssign + Into<u64>>(base: u8, value: T) -> Self {
        Self {
            base,
            value: value.into(),
            seq: Vec::new(),
        }
    }

    // does only 1 division of the whole transform sequence
    // returns owned partially transformed self
    pub(crate) fn transform(mut self) -> Self {
        if self.value % self.base as u64 != 0 {
            self.value = self.value / self.base as u64;
            self.seq.push(self.value as u8);
        }

        self
    }

    // does the whole transform sequence all at once
    // modifies self in place
    // reverses the seq vector as a bonus
    pub(crate) fn transform_all(&mut self) {
        while self.value % self.base as u64 != 0 {
            let res = self.value % self.base as u64;
            self.value = self.value / self.base as u64;
            self.seq.push(res as u8);
        }
    }

    // returns the reversed sequence of residual values
    pub(crate) fn sequence(&self) -> &[u8] {
        self.seq.as_slice()
    }
}
