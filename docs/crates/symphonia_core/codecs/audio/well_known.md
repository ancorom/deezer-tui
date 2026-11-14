**symphonia_core > codecs > audio > well_known**

# Module: codecs::audio::well_known

## Contents

**Modules**

- [`profiles`](#profiles) - Codec profiles for well-known audio codecs.

**Constants**

- [`CODEC_ID_AAC`](#codec_id_aac) - Advanced Audio Coding (AAC)
- [`CODEC_ID_AC3`](#codec_id_ac3) - AC-3 (Dolby Digital, ATSC A/52A)
- [`CODEC_ID_AC4`](#codec_id_ac4) - Dolby AC-4 (ETSI TS 103 190)
- [`CODEC_ID_ADPCM_G722`](#codec_id_adpcm_g722) - G.722 ADPCM
- [`CODEC_ID_ADPCM_G726`](#codec_id_adpcm_g726) - G.726 ADPCM
- [`CODEC_ID_ADPCM_G726LE`](#codec_id_adpcm_g726le) - G.726 ADPCM little-endian
- [`CODEC_ID_ADPCM_IMA_QT`](#codec_id_adpcm_ima_qt) - ADPCM IMA QuickTime
- [`CODEC_ID_ADPCM_IMA_WAV`](#codec_id_adpcm_ima_wav) - ADPCM IMA WAV
- [`CODEC_ID_ADPCM_MS`](#codec_id_adpcm_ms) - Microsoft ADPCM
- [`CODEC_ID_ALAC`](#codec_id_alac) - Apple Lossless Audio Codec (ALAC)
- [`CODEC_ID_APTX`](#codec_id_aptx) - aptX
- [`CODEC_ID_APTX_HD`](#codec_id_aptx_hd) - aptX HD
- [`CODEC_ID_ATRAC1`](#codec_id_atrac1) - Adaptive Transform Acoustic Coding (ATRAC1)
- [`CODEC_ID_ATRAC3`](#codec_id_atrac3) - Adaptive Transform Acoustic Coding 3 (ATRAC3)
- [`CODEC_ID_ATRAC3PLUS`](#codec_id_atrac3plus) - Adaptive Transform Acoustic Coding 3+ (ATRAC3+)
- [`CODEC_ID_ATRAC9`](#codec_id_atrac9) - Adaptive Transform Acoustic Coding 9 (ATRAC9)
- [`CODEC_ID_BINK_AUDIO`](#codec_id_bink_audio) - Bink Audio
- [`CODEC_ID_COOK`](#codec_id_cook) - Cook, Cooker, Gecko (RealAudio 6.0/G2)
- [`CODEC_ID_DCA`](#codec_id_dca) - DTS Coherent Acoustics (DCA/DTS)
- [`CODEC_ID_EAC3`](#codec_id_eac3) - Enhanced AC-3 (EAC-3, ATSC A/52B)
- [`CODEC_ID_FLAC`](#codec_id_flac) - Free Lossless Audio Codec (FLAC)
- [`CODEC_ID_LDAC`](#codec_id_ldac) - Lossless Digital Audio Codec (LDAC)
- [`CODEC_ID_MONKEYS_AUDIO`](#codec_id_monkeys_audio) - Monkey's Audio (APE)
- [`CODEC_ID_MP1`](#codec_id_mp1) - MPEG Layer 1 (MP1)
- [`CODEC_ID_MP2`](#codec_id_mp2) - MPEG Layer 2 (MP2)
- [`CODEC_ID_MP3`](#codec_id_mp3) - MPEG Layer 3 (MP3)
- [`CODEC_ID_MUSEPACK`](#codec_id_musepack) - Musepack
- [`CODEC_ID_OPUS`](#codec_id_opus) - Opus
- [`CODEC_ID_PCM_ALAW`](#codec_id_pcm_alaw) - PCM A-law (G.711)
- [`CODEC_ID_PCM_F32BE`](#codec_id_pcm_f32be) - PCM 32-bit big-endian floating point interleaved
- [`CODEC_ID_PCM_F32BE_PLANAR`](#codec_id_pcm_f32be_planar) - PCM 32-bit big-endian floating point planar
- [`CODEC_ID_PCM_F32LE`](#codec_id_pcm_f32le) - PCM 32-bit little-endian floating point interleaved
- [`CODEC_ID_PCM_F32LE_PLANAR`](#codec_id_pcm_f32le_planar) - PCM 32-bit little-endian floating point planar
- [`CODEC_ID_PCM_F64BE`](#codec_id_pcm_f64be) - PCM 64-bit big-endian floating point interleaved
- [`CODEC_ID_PCM_F64BE_PLANAR`](#codec_id_pcm_f64be_planar) - PCM 64-bit big-endian floating point planar
- [`CODEC_ID_PCM_F64LE`](#codec_id_pcm_f64le) - PCM 64-bit little-endian floating point interleaved
- [`CODEC_ID_PCM_F64LE_PLANAR`](#codec_id_pcm_f64le_planar) - PCM 64-bit little-endian floating point planar
- [`CODEC_ID_PCM_MULAW`](#codec_id_pcm_mulaw) - PCM Mu-law (G.711)
- [`CODEC_ID_PCM_S16BE`](#codec_id_pcm_s16be) - PCM signed 16-bit big-endian interleaved
- [`CODEC_ID_PCM_S16BE_PLANAR`](#codec_id_pcm_s16be_planar) - PCM signed 16-bit big-endian planar
- [`CODEC_ID_PCM_S16LE`](#codec_id_pcm_s16le) - PCM signed 16-bit little-endian interleaved
- [`CODEC_ID_PCM_S16LE_PLANAR`](#codec_id_pcm_s16le_planar) - PCM signed 16-bit little-endian planar
- [`CODEC_ID_PCM_S24BE`](#codec_id_pcm_s24be) - PCM signed 24-bit big-endian interleaved
- [`CODEC_ID_PCM_S24BE_PLANAR`](#codec_id_pcm_s24be_planar) - PCM signed 24-bit big-endian planar
- [`CODEC_ID_PCM_S24LE`](#codec_id_pcm_s24le) - PCM signed 24-bit little-endian interleaved
- [`CODEC_ID_PCM_S24LE_PLANAR`](#codec_id_pcm_s24le_planar) - PCM signed 24-bit little-endian planar
- [`CODEC_ID_PCM_S32BE`](#codec_id_pcm_s32be) - PCM signed 32-bit big-endian interleaved
- [`CODEC_ID_PCM_S32BE_PLANAR`](#codec_id_pcm_s32be_planar) - PCM signed 32-bit big-endian planar
- [`CODEC_ID_PCM_S32LE`](#codec_id_pcm_s32le) - PCM signed 32-bit little-endian interleaved
- [`CODEC_ID_PCM_S32LE_PLANAR`](#codec_id_pcm_s32le_planar) - PCM signed 32-bit little-endian planar
- [`CODEC_ID_PCM_S8`](#codec_id_pcm_s8) - PCM signed 8-bit interleaved
- [`CODEC_ID_PCM_S8_PLANAR`](#codec_id_pcm_s8_planar) - PCM signed 8-bit planar
- [`CODEC_ID_PCM_U16BE`](#codec_id_pcm_u16be) - PCM unsigned 16-bit big-endian interleaved
- [`CODEC_ID_PCM_U16BE_PLANAR`](#codec_id_pcm_u16be_planar) - PCM unsigned 16-bit big-endian planar
- [`CODEC_ID_PCM_U16LE`](#codec_id_pcm_u16le) - PCM unsigned 16-bit little-endian interleaved
- [`CODEC_ID_PCM_U16LE_PLANAR`](#codec_id_pcm_u16le_planar) - PCM unsigned 16-bit little-endian planar
- [`CODEC_ID_PCM_U24BE`](#codec_id_pcm_u24be) - PCM unsigned 24-bit big-endian interleaved
- [`CODEC_ID_PCM_U24BE_PLANAR`](#codec_id_pcm_u24be_planar) - PCM unsigned 24-bit big-endian planar
- [`CODEC_ID_PCM_U24LE`](#codec_id_pcm_u24le) - PCM unsigned 24-bit little-endian interleaved
- [`CODEC_ID_PCM_U24LE_PLANAR`](#codec_id_pcm_u24le_planar) - PCM unsigned 24-bit little-endian planar
- [`CODEC_ID_PCM_U32BE`](#codec_id_pcm_u32be) - PCM unsigned 32-bit big-endian interleaved
- [`CODEC_ID_PCM_U32BE_PLANAR`](#codec_id_pcm_u32be_planar) - PCM unsigned 32-bit big-endian planar
- [`CODEC_ID_PCM_U32LE`](#codec_id_pcm_u32le) - PCM unsigned 32-bit little-endian interleaved
- [`CODEC_ID_PCM_U32LE_PLANAR`](#codec_id_pcm_u32le_planar) - PCM unsigned 32-bit little-endian planar
- [`CODEC_ID_PCM_U8`](#codec_id_pcm_u8) - PCM unsigned 8-bit interleaved
- [`CODEC_ID_PCM_U8_PLANAR`](#codec_id_pcm_u8_planar) - PCM unsigned 8-bit planar
- [`CODEC_ID_RA10`](#codec_id_ra10) - RealAudio 1.0 14.4K (IS-54 VSELP)
- [`CODEC_ID_RA20`](#codec_id_ra20) - RealAudio 2.0 28.8K (G.728 LD-CELP)
- [`CODEC_ID_RALF`](#codec_id_ralf) - RealAudio Lossless Format (RALF)
- [`CODEC_ID_SBC`](#codec_id_sbc) - Low-complexity Subband Coding (SBC)
- [`CODEC_ID_SIPR`](#codec_id_sipr) - SIPR (ACELP.net, RealAudio 4.0/5.0)
- [`CODEC_ID_SMACKER_AUDIO`](#codec_id_smacker_audio) - Smacker Audio
- [`CODEC_ID_SPEEX`](#codec_id_speex) - Speex
- [`CODEC_ID_TRUEHD`](#codec_id_truehd) - Dolby TrueHD Lossless codec
- [`CODEC_ID_TTA`](#codec_id_tta) - True Audio (TTA)
- [`CODEC_ID_VORBIS`](#codec_id_vorbis) - Vorbis
- [`CODEC_ID_WAVPACK`](#codec_id_wavpack) - WavPack
- [`CODEC_ID_WMA`](#codec_id_wma) - Windows Media Audio

---

## symphonia_core::codecs::audio::well_known::CODEC_ID_AAC

*Constant*: `super::AudioCodecId`

Advanced Audio Coding (AAC)



## symphonia_core::codecs::audio::well_known::CODEC_ID_AC3

*Constant*: `super::AudioCodecId`

AC-3 (Dolby Digital, ATSC A/52A)



## symphonia_core::codecs::audio::well_known::CODEC_ID_AC4

*Constant*: `super::AudioCodecId`

Dolby AC-4 (ETSI TS 103 190)



## symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_G722

*Constant*: `super::AudioCodecId`

G.722 ADPCM



## symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_G726

*Constant*: `super::AudioCodecId`

G.726 ADPCM



## symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_G726LE

*Constant*: `super::AudioCodecId`

G.726 ADPCM little-endian



## symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_IMA_QT

*Constant*: `super::AudioCodecId`

ADPCM IMA QuickTime



## symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_IMA_WAV

*Constant*: `super::AudioCodecId`

ADPCM IMA WAV



## symphonia_core::codecs::audio::well_known::CODEC_ID_ADPCM_MS

*Constant*: `super::AudioCodecId`

Microsoft ADPCM



## symphonia_core::codecs::audio::well_known::CODEC_ID_ALAC

*Constant*: `super::AudioCodecId`

Apple Lossless Audio Codec (ALAC)



## symphonia_core::codecs::audio::well_known::CODEC_ID_APTX

*Constant*: `super::AudioCodecId`

aptX



## symphonia_core::codecs::audio::well_known::CODEC_ID_APTX_HD

*Constant*: `super::AudioCodecId`

aptX HD



## symphonia_core::codecs::audio::well_known::CODEC_ID_ATRAC1

*Constant*: `super::AudioCodecId`

Adaptive Transform Acoustic Coding (ATRAC1)



## symphonia_core::codecs::audio::well_known::CODEC_ID_ATRAC3

*Constant*: `super::AudioCodecId`

Adaptive Transform Acoustic Coding 3 (ATRAC3)



## symphonia_core::codecs::audio::well_known::CODEC_ID_ATRAC3PLUS

*Constant*: `super::AudioCodecId`

Adaptive Transform Acoustic Coding 3+ (ATRAC3+)



## symphonia_core::codecs::audio::well_known::CODEC_ID_ATRAC9

*Constant*: `super::AudioCodecId`

Adaptive Transform Acoustic Coding 9 (ATRAC9)



## symphonia_core::codecs::audio::well_known::CODEC_ID_BINK_AUDIO

*Constant*: `super::AudioCodecId`

Bink Audio



## symphonia_core::codecs::audio::well_known::CODEC_ID_COOK

*Constant*: `super::AudioCodecId`

Cook, Cooker, Gecko (RealAudio 6.0/G2)



## symphonia_core::codecs::audio::well_known::CODEC_ID_DCA

*Constant*: `super::AudioCodecId`

DTS Coherent Acoustics (DCA/DTS)



## symphonia_core::codecs::audio::well_known::CODEC_ID_EAC3

*Constant*: `super::AudioCodecId`

Enhanced AC-3 (EAC-3, ATSC A/52B)



## symphonia_core::codecs::audio::well_known::CODEC_ID_FLAC

*Constant*: `super::AudioCodecId`

Free Lossless Audio Codec (FLAC)



## symphonia_core::codecs::audio::well_known::CODEC_ID_LDAC

*Constant*: `super::AudioCodecId`

Lossless Digital Audio Codec (LDAC)



## symphonia_core::codecs::audio::well_known::CODEC_ID_MONKEYS_AUDIO

*Constant*: `super::AudioCodecId`

Monkey's Audio (APE)



## symphonia_core::codecs::audio::well_known::CODEC_ID_MP1

*Constant*: `super::AudioCodecId`

MPEG Layer 1 (MP1)



## symphonia_core::codecs::audio::well_known::CODEC_ID_MP2

*Constant*: `super::AudioCodecId`

MPEG Layer 2 (MP2)



## symphonia_core::codecs::audio::well_known::CODEC_ID_MP3

*Constant*: `super::AudioCodecId`

MPEG Layer 3 (MP3)



## symphonia_core::codecs::audio::well_known::CODEC_ID_MUSEPACK

*Constant*: `super::AudioCodecId`

Musepack



## symphonia_core::codecs::audio::well_known::CODEC_ID_OPUS

*Constant*: `super::AudioCodecId`

Opus



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_ALAW

*Constant*: `super::AudioCodecId`

PCM A-law (G.711)



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F32BE

*Constant*: `super::AudioCodecId`

PCM 32-bit big-endian floating point interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F32BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM 32-bit big-endian floating point planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F32LE

*Constant*: `super::AudioCodecId`

PCM 32-bit little-endian floating point interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F32LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM 32-bit little-endian floating point planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F64BE

*Constant*: `super::AudioCodecId`

PCM 64-bit big-endian floating point interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F64BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM 64-bit big-endian floating point planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F64LE

*Constant*: `super::AudioCodecId`

PCM 64-bit little-endian floating point interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_F64LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM 64-bit little-endian floating point planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_MULAW

*Constant*: `super::AudioCodecId`

PCM Mu-law (G.711)



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S16BE

*Constant*: `super::AudioCodecId`

PCM signed 16-bit big-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S16BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 16-bit big-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S16LE

*Constant*: `super::AudioCodecId`

PCM signed 16-bit little-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S16LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 16-bit little-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S24BE

*Constant*: `super::AudioCodecId`

PCM signed 24-bit big-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S24BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 24-bit big-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S24LE

*Constant*: `super::AudioCodecId`

PCM signed 24-bit little-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S24LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 24-bit little-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S32BE

*Constant*: `super::AudioCodecId`

PCM signed 32-bit big-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S32BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 32-bit big-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S32LE

*Constant*: `super::AudioCodecId`

PCM signed 32-bit little-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S32LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 32-bit little-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S8

*Constant*: `super::AudioCodecId`

PCM signed 8-bit interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_S8_PLANAR

*Constant*: `super::AudioCodecId`

PCM signed 8-bit planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U16BE

*Constant*: `super::AudioCodecId`

PCM unsigned 16-bit big-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U16BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 16-bit big-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U16LE

*Constant*: `super::AudioCodecId`

PCM unsigned 16-bit little-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U16LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 16-bit little-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U24BE

*Constant*: `super::AudioCodecId`

PCM unsigned 24-bit big-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U24BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 24-bit big-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U24LE

*Constant*: `super::AudioCodecId`

PCM unsigned 24-bit little-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U24LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 24-bit little-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U32BE

*Constant*: `super::AudioCodecId`

PCM unsigned 32-bit big-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U32BE_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 32-bit big-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U32LE

*Constant*: `super::AudioCodecId`

PCM unsigned 32-bit little-endian interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U32LE_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 32-bit little-endian planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U8

*Constant*: `super::AudioCodecId`

PCM unsigned 8-bit interleaved



## symphonia_core::codecs::audio::well_known::CODEC_ID_PCM_U8_PLANAR

*Constant*: `super::AudioCodecId`

PCM unsigned 8-bit planar



## symphonia_core::codecs::audio::well_known::CODEC_ID_RA10

*Constant*: `super::AudioCodecId`

RealAudio 1.0 14.4K (IS-54 VSELP)



## symphonia_core::codecs::audio::well_known::CODEC_ID_RA20

*Constant*: `super::AudioCodecId`

RealAudio 2.0 28.8K (G.728 LD-CELP)



## symphonia_core::codecs::audio::well_known::CODEC_ID_RALF

*Constant*: `super::AudioCodecId`

RealAudio Lossless Format (RALF)



## symphonia_core::codecs::audio::well_known::CODEC_ID_SBC

*Constant*: `super::AudioCodecId`

Low-complexity Subband Coding (SBC)



## symphonia_core::codecs::audio::well_known::CODEC_ID_SIPR

*Constant*: `super::AudioCodecId`

SIPR (ACELP.net, RealAudio 4.0/5.0)



## symphonia_core::codecs::audio::well_known::CODEC_ID_SMACKER_AUDIO

*Constant*: `super::AudioCodecId`

Smacker Audio



## symphonia_core::codecs::audio::well_known::CODEC_ID_SPEEX

*Constant*: `super::AudioCodecId`

Speex



## symphonia_core::codecs::audio::well_known::CODEC_ID_TRUEHD

*Constant*: `super::AudioCodecId`

Dolby TrueHD Lossless codec



## symphonia_core::codecs::audio::well_known::CODEC_ID_TTA

*Constant*: `super::AudioCodecId`

True Audio (TTA)



## symphonia_core::codecs::audio::well_known::CODEC_ID_VORBIS

*Constant*: `super::AudioCodecId`

Vorbis



## symphonia_core::codecs::audio::well_known::CODEC_ID_WAVPACK

*Constant*: `super::AudioCodecId`

WavPack



## symphonia_core::codecs::audio::well_known::CODEC_ID_WMA

*Constant*: `super::AudioCodecId`

Windows Media Audio



## Module: profiles

Codec profiles for well-known audio codecs.



