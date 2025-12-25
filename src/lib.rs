use bytes::{BufMut, BytesMut};

pub trait BufferFilling {

    fn extend_from_slice_ef(&mut self,slice:&[u8]);
    fn put_u8_ef(&mut self,slice:u8);
}

pub trait HumanInt{
    fn put_into(self, buf: &mut impl BufferFilling);
}

impl BufferFilling for  BytesMut {
    fn extend_from_slice_ef(&mut self, slice: &[u8]) {
        self.extend_from_slice(slice);
    }

    fn put_u8_ef(&mut self, slice: u8) {
        self.put_u8(slice);
    }
}



macro_rules! impl_human_int {
    ($t:ty, $size:expr) => {
        impl HumanInt for $t {
            fn put_into(self, buf: &mut impl BufferFilling) {
                let mut val = self;
                if val == 0 {
                    buf.put_u8_ef(b'0');
                    return;
                }
                let mut temp = [0u8; $size];
                let mut pos = $size;
                while val > 0 {
                    pos -= 1;
                    temp[pos] = (val % 10) as u8 + b'0';
                    val /= 10;
                }
                buf.extend_from_slice_ef(&temp[pos..]);
            }
        }
    };
}

impl_human_int!(u8, 3);
impl_human_int!(u16, 5);
impl_human_int!(u32, 10);
impl_human_int!(u64, 20);
impl_human_int!(usize, 20);
impl_human_int!(u128, 39);