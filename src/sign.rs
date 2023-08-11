use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

/// reference from: https://blog.csdn.net/zq1391345114/article/details/113815906
/// data: params NOT url-encoded
pub fn sign(data: &str, rng: &mut ThreadRng) -> String {
    const SIGN_TABLE: &[u8; 36] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let mut sign: String = "zza".into();

    let count = rng.gen_range(10..=16);
    sign.extend(SIGN_TABLE.choose_multiple(rng, count).map(|ch| *ch as char));

    let suffix = format!("{:x}", md5::compute("CJBPACrRuNy7".to_owned() + data));
    sign += suffix.as_ref();

    sign
}
