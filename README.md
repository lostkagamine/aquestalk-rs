# AquesTalk-rs
A crate for using the [AquesTalk10](https://www.a-quest.com/index.html) voice synthesis library in the Rust programming language.

You might know this synth voice from UTAU, as it is the voice of Defoko, the default preset.

## Disclaimer
I do not speak Japanese that well, so I probably didn't get things right. I also don't own a copy of this library, so I have no idea if this will work in actual applications. That said, please stick to the developers' TOS and purchase a license before shipping this.

## Testing
Download AquesTalk10 for your platform at [AQUEST's website](https://www.a-quest.com/download.html), then place it in a location where `rustc` can find it (for example, `/usr/lib64` worked in my case on Linux).

Afterwards, run `cargo r`. An `output.wav` file should be created.