//! Module with the [VecBool] implementation

/// Underlying datatype to store the bits
type Chunk = u8;
const CHUNK_SIZE: usize = 8;

/// Wrapper around [Vec<u8>]. You can use it similarly to a `Vec<bool>`.
pub struct VecBool {
    len: usize,
    chunks: Vec<Chunk>,
}

impl VecBool {
    #[inline]
    /// Creates a new empty [VecBool].
    ///
    /// Does not allocate memory on heap until elements are added.
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            len: 0,
        }
    }

    #[inline]
    // Create a [VecBool] with preallocated memory.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chunks: Vec::with_capacity((capacity / CHUNK_SIZE) + 1),
            len: 0,
        }
    }

    #[inline]
    // Create a [VecBool] with all bits set to `0`
    pub fn with_zeros(len: usize) -> Self {
        Self {
            chunks: vec![0; (len / CHUNK_SIZE) + 1],
            len,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.chunks.len() * CHUNK_SIZE
    }

    #[inline]
    /// Get the boolean value stored on `index`. If outbounds, return [None]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }

        Some(self.get_unchecked(index))
    }

    #[inline]
    /// Get the boolean value from `index` position. This method **panics** if `index` is out of bounds.
    pub fn get_unchecked(&self, index: usize) -> bool {
        let (chunk_index, mask) = VecBool::get_index(index);

        let bits = self.chunks[chunk_index];

        (bits & mask) != 0
    }

    #[inline]
    /// Set the boolean value in `index`. Returns `false` if `index` is out of bounds.
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        if index >= self.len {
            return false;
        }

        self.set_unchecked(index, value);

        true
    }

    #[inline]
    /// Set the boolean value in `index`. This method **panics** if `index` is out of bounds.
    pub fn set_unchecked(&mut self, index: usize, value: bool) {
        let (chunk_index, mask) = VecBool::get_index(index);

        if value {
            self.chunks[chunk_index] |= mask;
        } else {
            self.chunks[chunk_index] &= !mask;
        }
    }

    #[inline]
    /// Push an `bool` to the end of vector.
    pub fn push(&mut self, value: bool) {
        if self.len >= self.capacity() {
            self.chunks.push(0)
        }

        self.len += 1;
        self.set_unchecked(self.len - 1, value);
    }

    #[inline]
    /// Remove the last `bool` value from the vector. If the vector is empty, it return [None] otherwise it returns
    /// the removed value.
    pub fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let data = self.get_unchecked(self.len);

        if self.len % CHUNK_SIZE == 0 {
            self.chunks.pop();
        }

        Some(data)
    }

    #[inline]
    /// Unlike [VecBool::pop()], actually removes the last `bool` no matter where it is
    /// 
    /// More presice, but a bit more resourse intensive
    pub fn pop_bit(&mut self) -> Option<bool> {
        if self.len == 0 {
            return None;
        }

        let data = self.get_unchecked(self.len - 1);

        self.len -= 1;
        let bit_position = self.len % CHUNK_SIZE;

        if bit_position == 0 {
            self.chunks.pop();
        } else {
            //let mask: u8 = 0xFF / bit_position.pow(2) as u8;
            let mask: u8 = (0xFF as u8).overflowing_shl(bit_position as u32).0;
            self.chunks[self.len / CHUNK_SIZE] &= !mask;
        }

        

        Some(data)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.chunks
            .iter()
            .take(self.len / CHUNK_SIZE)
            .flat_map(|chunk| (0..CHUNK_SIZE).map(move |shift| chunk & (1 << shift) != 0))
            .chain({
                let chunk = self.chunks.last().copied().unwrap_or_default();
                (0..(self.len % CHUNK_SIZE)).map(move |shift| chunk & (1 << shift) != 0)
            })
    }

    #[inline]
    fn get_index(index: usize) -> (usize, Chunk) {
        let chunk_index = index / CHUNK_SIZE;
        let shifts = index % CHUNK_SIZE;
        let mask = 1 << shifts;

        (chunk_index, mask)
    }
}

impl From<Vec<bool>> for VecBool {
    fn from(value: Vec<bool>) -> Self {
        let mut out = Self::with_capacity(value.len() / CHUNK_SIZE);

        for v in value {
            out.push(v);
        };

        out
    }
}

impl From<VecBool> for Vec<bool> {
    fn from(value: VecBool) -> Self {
        let mut out = Self::with_capacity(value.len());

        for i in 0..value.len() {
            out.push(value.get(i).unwrap());
        };

        out
    }
}

impl From<VecBool> for Vec<u8> {
    /// Basically unwraps [VecBool]
    /// 
    /// 
    /// Preserving value beforehand from [VecBool::len()] method is recommended
    fn from(value: VecBool) -> Self {
        value.chunks
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let mask = VecBool::new();
        simple_test(mask);
    }

    #[test]
    fn conversions() {
        let v1: Vec<bool> = vec![true, true, false, true, false, false, true, false, false, true];
        let v2 = VecBool::from(v1.clone());
        let v3: Vec<bool> = Vec::from(v2);

        assert_eq!(v1, v3);
    }

    #[test]
    fn pop() {
        let v1: Vec<bool> = vec![true, true, false, true, false, false, true, false, false, true, false];
        let mut v2 = VecBool::from(v1.clone());

        let result = v2.pop_bit().unwrap();
        assert_eq!(result, v1[v1.len() - 1]);

        let result = v2.pop_bit().unwrap();
        assert_eq!(result, v1[v1.len() - 2]);

        assert_eq!(9, v2.len());
        assert_eq!(v2.get_unchecked(7), v1[v1.len() - 3])
    }

    #[test]
    fn with_capacity() {
        let mask = VecBool::with_capacity(CHUNK_SIZE * 4);
        simple_test(mask);
    }

    fn simple_test(mut mask: VecBool) {
        assert_eq!(mask.get(0), None);

        mask.push(true);
        assert_eq!(mask.get(0), Some(true));
        assert_eq!(mask.get(1), None);

        mask.push(false);
        assert_eq!(mask.get(0), Some(true));
        assert_eq!(mask.get(1), Some(false));
        assert_eq!(mask.get(2), None);

        mask.set(0, false);
        mask.set(1, true);
        assert_eq!(mask.iter().collect::<Vec<_>>(), vec![false, true]);

        assert_eq!(mask.pop(), Some(true));
        assert_eq!(mask.pop(), Some(false));
        assert_eq!(mask.pop(), None);
        assert_eq!(mask.iter().collect::<Vec<_>>(), vec![]);

        let size = CHUNK_SIZE * 4;
        for i in 0..size {
            mask.push(i % 3 == 0);
        }

        assert_eq!(
            mask.iter().collect::<Vec<_>>(),
            (0..size).map(|i| i % 3 == 0).collect::<Vec<_>>()
        )
    }

    #[test]
    fn with_len() {
        let len = 16;
        let mask = VecBool::with_zeros(len);

        for i in 0..len {
            assert_eq!(mask.get(i), Some(false));
        }

        assert_eq!(mask.get(len), None);
    }
}
