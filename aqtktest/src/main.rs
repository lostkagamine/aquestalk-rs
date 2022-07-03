use aquestalk_rs::{AquesTalk, Voice};

fn main() {
    let voice = Voice::Female1;
    
    // "ああ、このオーディオはAquesTalkなRustのテストです！"
    let test_words = "ア'ー、コノ/オーディオワ/アクエ_スト'ークナ/ル_ストノ/テ'_ストデ_ス、";

    let wav = AquesTalk::synth_utf8(voice, test_words)
        .expect("AquesTalk error!");

    std::fs::write("output.wav", wav).unwrap();
}
