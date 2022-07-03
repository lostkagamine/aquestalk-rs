use std::slice;

type CChar = u8;

#[repr(i32)]
pub enum AqtkVoiceBase {
    F1E = 0,
    F2E = 1,
    M1E = 2,
}

#[repr(C)]
pub struct AqtkVoiceParams {
    /// Voice base, see AqtkVoiceBase enum.
    pub bas: AqtkVoiceBase,

    /// Speed of speech. Range 50 to 300, default 100.
    pub spd: i32,

    /// Volume of speech. Range 0 to 300, default 100.
    pub vol: i32,

    /// Pitch of voice. Range 20 to 200.
    pub pit: i32,

    /// Accent. Range 0 to 200.
    pub acc: i32,

    /// Pitch adjust 1. Range 0 to 200, default 100.
    pub lmd: i32,

    /// Pitch adjust 2 (sampling frequency). Range 50 to 200, default 100.
    pub fsc: i32,
}

macro_rules! voice {
    ($bas:ident, $spd:literal, $vol:literal, $pit:literal, $acc:literal, $lmd:literal, $fsc:literal) => {
        AqtkVoiceParams {
            bas: AqtkVoiceBase::$bas,
            spd: $spd,
            vol: $vol,
            pit: $pit,
            acc: $acc,
            lmd: $lmd,
            fsc: $fsc,
        }
    };
}

const G_VOICE_F1: AqtkVoiceParams =
    voice!(F1E, 100, 100, 100, 100, 100, 100);

const G_VOICE_F2: AqtkVoiceParams =
    voice!(F2E, 100, 100,  77, 150, 100, 100);

const G_VOICE_F3: AqtkVoiceParams =
    voice!(F1E,  80, 100, 100, 100,  61, 148);

const G_VOICE_M1: AqtkVoiceParams =
    voice!(M1E, 100, 100,  30, 100, 100, 100);

const G_VOICE_M2: AqtkVoiceParams =
    voice!(M1E, 105, 100,  45, 130, 120, 100);

const G_VOICE_R1: AqtkVoiceParams =
    voice!(M1E, 100, 100,  30,  20, 190, 100);

const G_VOICE_R2: AqtkVoiceParams =
    voice!(F2E,  70, 100,  50,  50,  50, 180);

#[link(name="AquesTalk10")]
extern "C" {
    fn AquesTalk_Synthe_Utf8(gVoice: *const AqtkVoiceParams, koe: *const CChar, size: *const i32) -> *const u8; 
}

/// Voice enum, offers a choice between the pre-baked voices and a custom one with parameters.
pub enum Voice {
    Female1,
    Female2,
    Female3,
    Male1,
    Male2,
    Robot1,
    Robot2,
    Custom(AqtkVoiceParams)
}

impl Voice {
    pub(crate) fn voice_params(self) -> AqtkVoiceParams {
        match self {
            Voice::Female1 => G_VOICE_F1,
            Voice::Female2 => G_VOICE_F2,
            Voice::Female3 => G_VOICE_F3,
            Voice::Male1 => G_VOICE_M1,
            Voice::Male2 => G_VOICE_M2,
            Voice::Robot1 => G_VOICE_R1,
            Voice::Robot2 => G_VOICE_R2,
            Voice::Custom(x) => x,
        }
    }
}


/// Error enum for if things go wrong while synthesizing
#[derive(Debug)]
pub enum AqtkError {
    Unknown, // code 100 - その他のエラー
    OutOfMemory, // code 101 - メモリ不足
    BadInput, // code 103 - 音声記号列指定エラー(語頭の長音、促音の連続など)
    InputNotUnderstandable, // code 104 - 音声記号列に有効な読みがない
    BadSymbolInInput, // code 105 - 音声記号列に未定義の読み記号が指定された
    BadTagInInput, // code 106 - 音声記号列のタグの指定が正しくない
    InputTooLong, // code 107 - タグの長さが制限を越えている（または[>]がみつからない）
    TagValueIncorrect, // code 108 - タグ内の値の指定が正しくない
    InputStringTooLong, // code 120 - 音声記号列が長すぎる
    PhraseLimitExceeded, // code 121 - １つのフレーズ中の読み記号が多すぎる
    InternalBufferOverflow, // code 122 - 音声記号列が長い（内部バッファオーバー1）
}

impl TryFrom<i32> for AqtkError {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::Unknown),
            101 => Ok(Self::OutOfMemory),
            103 => Ok(Self::BadInput),
            104 => Ok(Self::InputNotUnderstandable),
            105 => Ok(Self::BadSymbolInInput),
            106 => Ok(Self::BadTagInInput),
            107 => Ok(Self::InputTooLong),
            108 => Ok(Self::TagValueIncorrect),
            120 => Ok(Self::InputStringTooLong),
            121 => Ok(Self::PhraseLimitExceeded),
            122 => Ok(Self::InternalBufferOverflow),
            _ => Err(())
        }
    }
}

pub enum AquesTalk {}

impl AquesTalk {
    /// The main synthesis function. "koe" is your text to synthesize in the format AquesTalk wants.
    /// Use something like AqKanji2Koe to get this out of normal Japanese text.
    /// Results come back as a `Vec<u8>` holding bytes to be interpreted as WAV data.
    pub fn synth_utf8<S: Into<String>>(voice: Voice, koe: S) -> Result<Vec<u8>, AqtkError> {
        let s: String = koe.into();
        let size: i32 = 0;

        let res = unsafe {
            AquesTalk_Synthe_Utf8(&voice.voice_params(), s.as_ptr(), &size)
        };

        if res.is_null() {
            return Err(size.try_into().unwrap())
        }

        let res_slice = unsafe {
            slice::from_raw_parts(res, size.try_into().unwrap())
        };
        
        Ok(res_slice.to_vec())
    }
}