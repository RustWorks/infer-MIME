use std::io::Read;
use std::io;

/// Returns whether a buffer is MIDI data.
pub fn is_midi(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x4D && buf[1] == 0x54 && buf[2] == 0x68 && buf[3] == 0x64
}

/// Returns whether a buffer is MP3 data.
pub fn is_mp3(buf: &[u8]) -> bool {
    buf.len() > 2
        && ((buf[0] == 0x49 && buf[1] == 0x44 && buf[2] == 0x33) // ID3v2
			// Final bit (has crc32) may be or may not be set.
			|| (buf[0] == 0xFF && buf[1] == 0xFB))
}

/// Returns whether a buffer is M4A data.
pub fn is_m4a(buf: &[u8]) -> bool {
    buf.len() > 10
        && ((buf[4] == 0x66
            && buf[5] == 0x74
            && buf[6] == 0x79
            && buf[7] == 0x70
            && buf[8] == 0x4D
            && buf[9] == 0x34
            && buf[10] == 0x41)
            || (buf[0] == 0x4D && buf[1] == 0x34 && buf[2] == 0x41 && buf[3] == 0x20))
}

/// Returns whether a buffer is OGG data.
pub fn is_ogg(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x4F && buf[1] == 0x67 && buf[2] == 0x67 && buf[3] == 0x53
}

/// Returns whether a buffer is FLAC data.
pub fn is_flac(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x66 && buf[1] == 0x4C && buf[2] == 0x61 && buf[3] == 0x43
}

/// Returns whether a buffer is WAV data.
pub fn is_wav(buf: &[u8]) -> bool {
    buf.len() > 11
        && buf[0] == 0x52
        && buf[1] == 0x49
        && buf[2] == 0x46
        && buf[3] == 0x46
        && buf[8] == 0x57
        && buf[9] == 0x41
        && buf[10] == 0x56
        && buf[11] == 0x45
}

/// Returns whether a buffer is AMR data.
pub fn is_amr(buf: &[u8]) -> bool {
    buf.len() > 11
        && buf[0] == 0x23
        && buf[1] == 0x21
        && buf[2] == 0x41
        && buf[3] == 0x4D
        && buf[4] == 0x52
        && buf[5] == 0x0A
}

/// Returns whether a buffer is AAC data.
pub fn is_aac(buf: &[u8]) -> bool {
    buf.len() > 1 && buf[0] == 0xFF && (buf[1] == 0xF1 || buf[1] == 0xF9)
}

/// Returns whether a buffer is AIFF data.
pub fn is_aiff(buf: &[u8]) -> bool {
    buf.len() > 11
        && buf[0] == 0x46
        && buf[1] == 0x4F
        && buf[2] == 0x52
        && buf[3] == 0x4D
        && buf[8] == 0x41
        && buf[9] == 0x49
        && buf[10] == 0x46
        && buf[11] == 0x46
}

/// Returns whether a buffer is DSF data.
pub fn is_dsf(buf: &[u8]) -> bool {
    // ref: https://dsd-guide.com/sites/default/files/white-papers/DSFFileFormatSpec_E.pdf
    buf.len() > 4 && buf[0] == b'D' && buf[1] == b'S' && buf[2] == b'D' && buf[3] == b' '
}

/// Returns whether a buffer is APE (Monkey's Audio) data.
pub fn is_ape(buf: &[u8]) -> bool {
    // ref: https://github.com/fernandotcl/monkeys-audio/blob/master/src/MACLib/APEHeader.h
    buf.len() > 4 && buf[0] == b'M' && buf[1] == b'A' && buf[2] == b'C' && buf[3] == b' '
}

super::build_fn_read_api! (
    /// Returns whether data from reader is MIDI data.
    (is_midi_read, is_midi, 4),
    
    /// Returns whether data from reader is is MP3 data.
    (is_mp3_read, is_mp3, 3),

    /// Returns whether data from reader is M4A data.
    (is_m4a_read, is_m4a, 11),

    /// Returns whether data from reader is OGG data.
    (is_ogg_read, is_ogg, 4),

    /// Returns whether data from reader is FLAC data.
    (is_flac_read, is_flac, 4),

    /// Returns whether data from reader is WAV data.
    (is_wav_read, is_wav, 12),

    /// Returns whether data from reader is AMR data.
    (is_amr_read, is_amr, 12),

    /// Returns whether data from reader is AAC data.
    (is_aac_read, is_aac, 2),

    /// Returns whether data from reader is AIFF data.
    (is_aiff_read, is_aiff, 12),

    /// Returns whether data from reader is DSF data.
    (is_dsf_read, is_dsf, 5),

    /// Returns whether data from reader is APE (Monkey's Audio) data.
    (is_ape_read, is_ape, 5)
);