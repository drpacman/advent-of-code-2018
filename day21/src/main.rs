use std::num::Wrapping;
use std::collections::HashSet;

fn main() {
    let mut seen_values_for_r4 = HashSet::<Wrapping<i32>>::new();
    let mut prev_r4 = Wrapping(0i32);
    let mut r0 = Wrapping(2i32);
    let mut r1 = Wrapping(0i32);
    let mut r3 = Wrapping(0i32);
    let mut r4 = Wrapping(0i32);
    let mut r5 = Wrapping(0i32);
    r4 = Wrapping(123i32);
    loop {
        r4 = r4 & Wrapping(456i32);
        if r4.0 == 72 { break; } 
    }
    r4 = Wrapping(0i32);
    'six : loop {
        r5 = r4 | Wrapping(65536i32);
        r4 = Wrapping(1765573i32);
        'eight : loop {
            r1 = r5 & Wrapping(255i32);
            r4 = r4 + r1;
            r4 = ((r4 & Wrapping(16777215i32)) * Wrapping(65899i32)) & Wrapping(16777215i32);
            if r5 < Wrapping(256i32) {
                if seen_values_for_r4.is_empty() {
                    println!("Part 1 - First value for r4 is {}", r4);
                } else if seen_values_for_r4.contains(&r4) {
                    println!("Repeats after {}", seen_values_for_r4.len());
                    // Report the last item before it repeated
                    println!("Part 2 - {:?}", prev_r4);
                    break 'six;
                }
                seen_values_for_r4.insert(r4);
                prev_r4 = r4;
                break 'eight;
            }
            r1 = Wrapping(0);
            'eighteen : loop {
                r3 = (r1 + Wrapping(1i32)) * Wrapping(256i32);
                if r3 > r5 {
                    break;
                }
                r1=r1+Wrapping(1i32);
            }
            r5=r1
        }
    }
}
