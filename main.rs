#[allow(unused)]
mod cp {
    use std::io::{Read, stdin};
    pub struct StandardInputParser {
        iter: std::str::SplitWhitespace<'static>,
    }

    impl StandardInputParser {
        #[inline(always)]
        pub fn load_stdin() -> Self {
            let mut input = String::new();
            unsafe {
                stdin().read_to_string(&mut input).unwrap_unchecked();
            }

            let input_ref: &'static str = Box::leak(input.into_boxed_str());
            StandardInputParser {
                iter: input_ref.split_whitespace(),
            }
        }

        #[inline(always)]
        pub fn get<T: std::str::FromStr>(&mut self) -> T {
            unsafe {
                self.iter
                    .next()
                    .unwrap_unchecked()
                    .parse::<T>()
                    .ok()
                    .unwrap_unchecked()
            }
        }

        #[inline(always)]
        pub fn get_i32(&mut self) -> i32 {
            self.get()
        }
        #[inline(always)]
        pub fn get_i64(&mut self) -> i64 {
            self.get()
        }
        #[inline(always)]
        pub fn get_isize(&mut self) -> isize {
            self.get()
        }
        #[inline(always)]
        pub fn get_u32(&mut self) -> u32 {
            self.get()
        }
        #[inline(always)]
        pub fn get_u64(&mut self) -> u64 {
            self.get()
        }
        #[inline(always)]
        pub fn get_usize(&mut self) -> usize {
            self.get()
        }
    }

    pub struct FastInputParser {
        buf: Vec<u8>,
        idx: usize,
    }

    impl FastInputParser {
        #[inline(always)]
        pub fn load_stdin() -> Self {
            let mut input = Vec::with_capacity(1 << 20);
            stdin().read_to_end(&mut input).unwrap();

            FastInputParser { buf: input, idx: 0 }
        }

        #[inline(always)]
        fn skip_whitespace(&mut self) {
            unsafe {
                let c = *self.buf.get_unchecked(self.idx);
                if c == b' ' || c == b'\n' || c == b'\r' {
                    self.idx += 1;
                }
            }
        }

        #[inline(always)]
        fn read_i64(&mut self) -> i64 {
            self.skip_whitespace();

            let mut neg = false;
            let mut val: i64 = 0;

            unsafe {
                let mut c = *self.buf.get_unchecked(self.idx);
                if c == b'-' {
                    neg = true;
                    self.idx += 1;
                }

                loop {
                    c = *self.buf.get_unchecked(self.idx);
                    if c < b'0' {
                        break;
                    }
                    val = val * 10 + (c - b'0') as i64;
                    self.idx += 1;
                }
            }

            self.skip_whitespace();

            if neg { -val } else { val }
        }

        #[inline(always)]
        fn read_u64(&mut self) -> u64 {
            self.skip_whitespace();

            let mut val: u64 = 0;

            unsafe {
                loop {
                    let c = *self.buf.get_unchecked(self.idx);
                    if c < b'0' {
                        break;
                    }
                    val = val * 10 + (c - b'0') as u64;
                    self.idx += 1;
                }
            }

            self.skip_whitespace();

            val
        }

        #[inline(always)]
        pub fn get_i32(&mut self) -> i32 {
            self.read_i64() as i32
        }
        #[inline(always)]
        pub fn get_i64(&mut self) -> i64 {
            self.read_i64()
        }
        #[inline(always)]
        pub fn get_isize(&mut self) -> isize {
            self.read_i64() as isize
        }

        #[inline(always)]
        pub fn get_u32(&mut self) -> u32 {
            self.read_u64() as u32
        }
        #[inline(always)]
        pub fn get_u64(&mut self) -> u64 {
            self.read_u64()
        }
        #[inline(always)]
        pub fn get_usize(&mut self) -> usize {
            self.read_u64() as usize
        }
    }
}

fn main() {
    let mut ip = cp::StandardInputParser::load_stdin();
}
