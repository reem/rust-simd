#![feature(macro_rules, globs)]
#![allow(experimental, dead_code)]

use std::simd::f64x2;

pub struct VectorPlain {
    x: f64,
    y: f64
}

pub struct VectorSimd {
    inner: f64x2
}

pub trait Vector {
    fn shift(&mut self, other: &Self);
    fn as_tuple(&self) -> (f64, f64);
}

impl VectorPlain {
    #[inline]
    fn new(x: f64, y: f64) -> VectorPlain {
        VectorPlain {
            x: x,
            y: y
        }
    }
}

impl Vector for VectorPlain {
    #[inline]
    fn shift(&mut self, other: &VectorPlain) {
        self.x += other.x;
        self.y += other.y;
    }

    #[inline]
    fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl VectorSimd {
    #[inline]
    fn new(x: f64, y: f64) -> VectorSimd {
        VectorSimd {
            inner: f64x2(x, y)
        }
    }
}

impl Vector for VectorSimd {
    #[inline]
    fn shift(&mut self, other: &VectorSimd) {
        self.inner += other.inner;
    }

    #[inline]
    fn as_tuple(&self) -> (f64, f64) {
        let f64x2(x, y) = self.inner;
        (x, y)
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use self::test::Bencher;
    use super::*;

    macro_rules! test_vector_add {
        ($name:ident, $vec_type:ident) => {
            #[test]
            fn $name() {
                let mut a = $vec_type::new(10.0, 20.0);
                let b = $vec_type::new(20.0, 30.0);
                a.shift(&b);
                assert_eq!(a.as_tuple(), (30.0, 50.0));
                a.shift(&b);
                assert_eq!(a.as_tuple(), (50.0, 80.0));
            }
        }
    }

    macro_rules! bench_vector_add {
        ($name:ident, $vec_type:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                b.iter(|| {
                    for _ in range(0u, 100) {
                        test::black_box(
                            $vec_type::new(10.0, 20.0)
                                .shift(&$vec_type::new(20.0, 30.0))
                        );
                    }
                });
            }
        }
    }

    test_vector_add!(test_add_plain, VectorPlain)
    test_vector_add!(test_add_simd, VectorSimd)

    bench_vector_add!(bench_add_plain, VectorPlain)
    bench_vector_add!(bench_add_simd, VectorSimd)
}
