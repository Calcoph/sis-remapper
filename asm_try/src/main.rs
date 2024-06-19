#![feature(stdarch_x86_avx512)]

use m512::{f32x16, ConstM512};

mod m512;

const INPUT_F32: ConstM512 = ConstM512::full([
    0.0,1.0,2.0,3.0,
    4.0,5.0,6.0,7.0,
    8.0,9.0,10.0,11.0,
    12.0,13.0,14.0,15.0,
]);

const INPUT2_F32: ConstM512 = ConstM512::full([
    16.0,17.0,18.0,19.0,
    20.0,21.0,22.0,23.0,
    24.0,25.0,26.0,27.0,
    28.0,29.0,30.0,31.0,
]);

const FIVE: ConstM512 = ConstM512::single(5.0);
const ONE: ConstM512 = ConstM512::single(1.0);


fn main() {
    let a = f32x16::load_aligned(INPUT_F32.as_ptr());
    let b = f32x16::load_aligned(INPUT2_F32.as_ptr());
    let comp = &mut f32x16::load_aligned(FIVE.as_ptr());
    let one = f32x16::load_aligned(ONE.as_ptr());
    let counter = &mut f32x16::zero();

    let mut go = true;
    while go {
        let (count, exit, passed) = a.incr_if_ge(*comp, *counter, one);
        *comp = *comp + one;
        *counter = count;
        go = !exit
    }

    let mut out = ConstM512::full([
        0.1,0.1,0.1,0.1,
        0.1,0.1,0.1,0.1,
        0.1,0.1,0.1,0.1,
        0.1,0.1,0.1,0.1,
    ]);

    let dest = out.as_mut_ptr();
    (*counter).recover(dest);


    for (i, a) in out.0.iter().enumerate() {
        print!("{a:4} ");
        if i % 4 == 3 {
            println!();
        }
    }
}
