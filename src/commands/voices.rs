use rand::{Rng, thread_rng};

// enum Voices {
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
//     Ten,
//     Eleven,
// }
pub fn choose_voice() -> &'static str {
    const V01: &str = "108305-2-3.mp4";
    const V02: &str = "108305-2-140.mp4";
    const V03: &str = "108305-2-143.mp4";
    const V04: &str = "108305-2-163.mp4";
    const V05: &str = "108305-2-196.mp4";
    const V06: &str = "108305-2-197.mp4";
    const V07: &str = "108305-2-208.mp4";
    const V08: &str = "108305-2-209.mp4";
    const V09: &str = "108305-2-210.mp4";
    const V10: &str = "108305-2-241.mp4";
    const V11: &str = "108305-2-250.mp4";
    let mut rng = thread_rng();
    let roll: u8 = rng.gen_range(0..11);
    // let roll: Voices = rng.gen();
    let voice = match roll {
        0 => V01,
        1 => V02,
        2 => V03,
        3 => V04,
        4 => V05,
        5 => V06,
        6 => V07,
        7 => V08,
        8 => V09,
        9 => V10,
        10 => V11,
        _ => unreachable!(),
        // One => V01,
        // Two => V02,
        // Three => V03,
        // Four => V04,
        // Five => V05,
        // Six => V06,
        // Seven => V07,
        // Eight => V08,
        // Nine => V09,
        // Ten => V10,
        // Eleven => V11,
    };
    voice
}