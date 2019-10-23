#[derive(Default)]
pub struct BitArray {
    data: Vec<u32>,
}

impl BitArray {
    pub fn new() -> BitArray {
        Default::default()
    }

    pub fn set(&mut self, i: usize, v: bool) {
        let index = i / 32;
        let bit = i % 32;
        let l = self.data.len();
        if l <= i {
            self.data
                .extend(vec![0; index.wrapping_sub(l).wrapping_add(1)]);
        }
        if v {
            self.data[index] = self.data[index] | (1 << bit);
        } else {
            self.data[index] = self.data[index] & !(1 << bit);
        }
    }

    pub fn get(&mut self, i: usize) -> bool {
        let index = i / 32;
        let bit = i % 32;
        if index >= self.data.len() {
            return false;
        }
        ((self.data[index] & (1 << bit)) >> bit) == 1
    }
}

#[derive(Default)]
pub struct BitSet {
    pub layer_0: BitArray,
    pub layer_1: BitArray,
    pub layer_2: BitArray,
    pub layer_3: bool,
}

impl BitSet {
    pub fn new() -> BitSet {
        Default::default()
    }

    pub fn set(&mut self, i: usize, v: bool) {
        self.layer_0.set(i,v);
        if v == true {
            self.layer_1.set(i/32,true);
            self.layer_2.set(i/32/32,true);
            self.layer_3 = true;
        } else {
            let mut j = i/32;
            let mut o = false;
            for x in 0..32 {
                o &= self.layer_0.get(j+x)
            }
            if o == false {
                self.layer_1.set(i/32,false);
            }

            j = i/32/32;
            o = false;
            for x in 0..32 {
                o &= self.layer_1.get(j+x)
            }
            if o == false {
                self.layer_2.set(i/32,false);
            }

            j = i/32/32/32;
            o = false;
            for x in 0..32 {
                o &= self.layer_2.get(j+x)
            }
            if o == false {
                self.layer_3 = false;
            }
        }
    }

    pub fn get(&mut self, i: usize) -> bool {
        self.layer_0.get(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn bit_array() {
        let mut l = BitArray::new();
        assert_eq!(l.data.len(), 0);
        l.set(0, true);
        assert_eq!(l.get(12), false);
        l.set(12, true);
        assert_eq!(l.data.len(), 1);
        assert_eq!(l.get(12), true);
        l.set(12, true);
        assert_eq!(l.data.len(), 1);
        l.set(31, true);
        assert_eq!(l.data.len(), 1);
    }

    #[test]
    fn bit_set() {
        let mut b = BitSet::new();
        b.set(1,true);
        assert_eq!(b.get(1), true);
    }
}
