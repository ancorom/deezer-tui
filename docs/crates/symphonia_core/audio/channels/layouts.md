**symphonia_core > audio > channels > layouts**

# Module: audio::channels::layouts

## Contents

**Constants**

- [`CHANNEL_LAYOUT_22P2`](#channel_layout_22p2) - 22.2 audio.
- [`CHANNEL_LAYOUT_2P1`](#channel_layout_2p1) - Stereophonic audio with low-frequency effects.
- [`CHANNEL_LAYOUT_3P0`](#channel_layout_3p0) - 3.0 audio.
- [`CHANNEL_LAYOUT_3P0_REAR`](#channel_layout_3p0_rear) - 3.0 audio with rear surround channel.
- [`CHANNEL_LAYOUT_3P1`](#channel_layout_3p1) - 3.1 audio.
- [`CHANNEL_LAYOUT_4P0`](#channel_layout_4p0) - 4.0 audio.
- [`CHANNEL_LAYOUT_4P0_QUAD`](#channel_layout_4p0_quad) - Quadrophonic audio with rear channels.
- [`CHANNEL_LAYOUT_4P0_QUAD_SIDE`](#channel_layout_4p0_quad_side) - Quadrophonic audio with side channels.
- [`CHANNEL_LAYOUT_4P1`](#channel_layout_4p1) - 4.1 audio.
- [`CHANNEL_LAYOUT_5P0`](#channel_layout_5p0) - 5.0 audio.
- [`CHANNEL_LAYOUT_5P0_SIDE`](#channel_layout_5p0_side) - 5.0 audio with side channels.
- [`CHANNEL_LAYOUT_5P1`](#channel_layout_5p1) - 5.1 audio.
- [`CHANNEL_LAYOUT_5P1_SIDE`](#channel_layout_5p1_side) - 5.1 audio with side channels and low-frequency effects.
- [`CHANNEL_LAYOUT_6P0`](#channel_layout_6p0) - 6.0 audio.
- [`CHANNEL_LAYOUT_6P0_FRONT`](#channel_layout_6p0_front) - 6.0 audio with additional front channels.
- [`CHANNEL_LAYOUT_6P0_HEX`](#channel_layout_6p0_hex) - Hexagonal audio.
- [`CHANNEL_LAYOUT_6P1`](#channel_layout_6p1) - 6.1 audio with low-frequency effects.
- [`CHANNEL_LAYOUT_6P1_FRONT`](#channel_layout_6p1_front) - 6.1 audio with additional front channels and low-frequency effects.
- [`CHANNEL_LAYOUT_6P1_HEX`](#channel_layout_6p1_hex) - Hexagonal audio with low-frequency effects.
- [`CHANNEL_LAYOUT_7P0`](#channel_layout_7p0) - 7.0 surround audio.
- [`CHANNEL_LAYOUT_7P0_WIDE`](#channel_layout_7p0_wide) - 7.0 audio with wide front channels.
- [`CHANNEL_LAYOUT_7P0_WIDE_SIDE`](#channel_layout_7p0_wide_side) - 7.0 audio with wide front channels and side channels.
- [`CHANNEL_LAYOUT_7P1`](#channel_layout_7p1) - 7.1 surround audio with low-frequency effects.
- [`CHANNEL_LAYOUT_7P1_WIDE`](#channel_layout_7p1_wide) - 7.1 audio with wide front channels and low-frequency effects.
- [`CHANNEL_LAYOUT_7P1_WIDE_SIDE`](#channel_layout_7p1_wide_side) - 7.1 audio with wide front channels and side channels.
- [`CHANNEL_LAYOUT_AAC_3P0`](#channel_layout_aac_3p0) - AAC 3-channel channel layout.
- [`CHANNEL_LAYOUT_AAC_4P0`](#channel_layout_aac_4p0) - AAC 4-channel surround-based layout.
- [`CHANNEL_LAYOUT_AAC_5P0`](#channel_layout_aac_5p0) - AAC 5-channel surround-based layout.
- [`CHANNEL_LAYOUT_AAC_5P1`](#channel_layout_aac_5p1) - AAC 5.1-channel surround-based layout.
- [`CHANNEL_LAYOUT_AAC_6P0`](#channel_layout_aac_6p0) - AAC 6-channel surround-based layout.
- [`CHANNEL_LAYOUT_AAC_6P1`](#channel_layout_aac_6p1) - AAC 6.1-channel surround-based layout.
- [`CHANNEL_LAYOUT_AAC_7P1`](#channel_layout_aac_7p1) - AAC 7.1-channel surround-based layout.
- [`CHANNEL_LAYOUT_AAC_QUADRAPHONIC`](#channel_layout_aac_quadraphonic) - AAC quadraphonic surround-based layout.
- [`CHANNEL_LAYOUT_MONO`](#channel_layout_mono) - Monophonic audio.
- [`CHANNEL_LAYOUT_MPEG_1P0`](#channel_layout_mpeg_1p0) - MPEG 1-channel channel layout.
- [`CHANNEL_LAYOUT_MPEG_2P0`](#channel_layout_mpeg_2p0) - MPEG 2-channel channel layout.
- [`CHANNEL_LAYOUT_MPEG_3P0_A`](#channel_layout_mpeg_3p0_a) - MPEG 3-channel, configuration A, channel layout.
- [`CHANNEL_LAYOUT_MPEG_3P0_B`](#channel_layout_mpeg_3p0_b) - MPEG 3-channel, configuration B, channel layout.
- [`CHANNEL_LAYOUT_MPEG_4P0_A`](#channel_layout_mpeg_4p0_a) - MPEG 4-channel, configuration A, channel layout.
- [`CHANNEL_LAYOUT_MPEG_4P0_B`](#channel_layout_mpeg_4p0_b) - MPEG 4-channel, configuration B, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P0_A`](#channel_layout_mpeg_5p0_a) - MPEG 5-channel, configuration A, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P0_B`](#channel_layout_mpeg_5p0_b) - MPEG 5-channel, configuration B, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P0_C`](#channel_layout_mpeg_5p0_c) - MPEG 5-channel, configuration C, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P0_D`](#channel_layout_mpeg_5p0_d) - MPEG 5-channel, configuration D, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P1_A`](#channel_layout_mpeg_5p1_a) - MPEG 5.1-channel, configuration A, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P1_B`](#channel_layout_mpeg_5p1_b) - MPEG 5.1-channel, configuration B, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P1_C`](#channel_layout_mpeg_5p1_c) - MPEG 5.1-channel, configuration C, channel layout.
- [`CHANNEL_LAYOUT_MPEG_5P1_D`](#channel_layout_mpeg_5p1_d) - MPEG 5.1-channel, configuration D, channel layout.
- [`CHANNEL_LAYOUT_MPEG_6P1_A`](#channel_layout_mpeg_6p1_a) - MPEG 6.1-channel, configuration A, channel layout.
- [`CHANNEL_LAYOUT_MPEG_7P1_A`](#channel_layout_mpeg_7p1_a) - MPEG 7.1-channel, configuration A, channel layout.
- [`CHANNEL_LAYOUT_MPEG_7P1_B`](#channel_layout_mpeg_7p1_b) - MPEG 7.1-channel, configuration B, channel layout.
- [`CHANNEL_LAYOUT_OGG_1P0`](#channel_layout_ogg_1p0) - OGG 1-channel layout.
- [`CHANNEL_LAYOUT_OGG_2P0`](#channel_layout_ogg_2p0) - OGG 2-channel layout.
- [`CHANNEL_LAYOUT_OGG_3P0`](#channel_layout_ogg_3p0) - OGG 3-channel layout.
- [`CHANNEL_LAYOUT_OGG_4P0`](#channel_layout_ogg_4p0) - OGG 4-channel layout.
- [`CHANNEL_LAYOUT_OGG_5P0`](#channel_layout_ogg_5p0) - OGG 5-channel layout.
- [`CHANNEL_LAYOUT_OGG_5P1`](#channel_layout_ogg_5p1) - OGG 5.1-channel layout.
- [`CHANNEL_LAYOUT_OGG_6P1`](#channel_layout_ogg_6p1) - OGG 6.1-channel layout.
- [`CHANNEL_LAYOUT_OGG_7P1`](#channel_layout_ogg_7p1) - OGG 7.1-channel layout.
- [`CHANNEL_LAYOUT_STEREO`](#channel_layout_stereo) - Stereophonic audio.

---

## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_22P2

*Constant*: `super::Channels`

22.2 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects 1
* Rear left
* Rear right
* Front left-of-center
* Front right-of-center
* Rear center
* Low-frequency effects 2
* Side left
* Side right
* Top front left
* Top front right
* Top front center
* Top center
* Top rear left
* Top rear right
* Top side left
* Top side right
* Top rear center
* Bottom front center
* Bottom front left
* Bottom front right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_2P1

*Constant*: `super::Channels`

Stereophonic audio with low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_3P0

*Constant*: `super::Channels`

3.0 audio.

The channels in this layout are:
* Front left
* Front right
* Front center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_3P0_REAR

*Constant*: `super::Channels`

3.0 audio with rear surround channel.

The channels in this layout are:
* Front left
* Front right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_3P1

*Constant*: `super::Channels`

3.1 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_4P0

*Constant*: `super::Channels`

4.0 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_4P0_QUAD

*Constant*: `super::Channels`

Quadrophonic audio with rear channels.

The channels in this layout are:
* Front left
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_4P0_QUAD_SIDE

*Constant*: `super::Channels`

Quadrophonic audio with side channels.

The channels in this layout are:
* Front left
* Front right
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_4P1

*Constant*: `super::Channels`

4.1 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_5P0

*Constant*: `super::Channels`

5.0 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_5P0_SIDE

*Constant*: `super::Channels`

5.0 audio with side channels.

The channels in this layout are:
* Front left
* Front right
* Front center
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_5P1

*Constant*: `super::Channels`

5.1 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_5P1_SIDE

*Constant*: `super::Channels`

5.1 audio with side channels and low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_6P0

*Constant*: `super::Channels`

6.0 audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Side left
* Side right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_6P0_FRONT

*Constant*: `super::Channels`

6.0 audio with additional front channels.

The channels in this layout are:
* Front left
* Front right
* Front left-of-center
* Front right-of-center
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_6P0_HEX

*Constant*: `super::Channels`

Hexagonal audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear left
* Rear right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_6P1

*Constant*: `super::Channels`

6.1 audio with low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear center
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_6P1_FRONT

*Constant*: `super::Channels`

6.1 audio with additional front channels and low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Low-frequency effects
* Front left-of-center
* Front right-of-center
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_6P1_HEX

*Constant*: `super::Channels`

Hexagonal audio with low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_7P0

*Constant*: `super::Channels`

7.0 surround audio.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear left
* Rear right
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_7P0_WIDE

*Constant*: `super::Channels`

7.0 audio with wide front channels.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear left
* Rear right
* Front left-of-center
* Front right-of-center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_7P0_WIDE_SIDE

*Constant*: `super::Channels`

7.0 audio with wide front channels and side channels.

The channels in this layout are:
* Front left
* Front right
* Front center
* Front left-of-center
* Front right-of-center
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_7P1

*Constant*: `super::Channels`

7.1 surround audio with low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_7P1_WIDE

*Constant*: `super::Channels`

7.1 audio with wide front channels and low-frequency effects.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right
* Front left-of-center
* Front right-of-center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_7P1_WIDE_SIDE

*Constant*: `super::Channels`

7.1 audio with wide front channels and side channels.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Front left-of-center
* Front right-of-center
* Side left
* Side right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_3P0

*Constant*: `super::Channels`

AAC 3-channel channel layout.

The channels in this layout are:
* Front center
* Front left
* Front right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_4P0

*Constant*: `super::Channels`

AAC 4-channel surround-based layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_5P0

*Constant*: `super::Channels`

AAC 5-channel surround-based layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_5P1

*Constant*: `super::Channels`

AAC 5.1-channel surround-based layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_6P0

*Constant*: `super::Channels`

AAC 6-channel surround-based layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear left
* Rear right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_6P1

*Constant*: `super::Channels`

AAC 6.1-channel surround-based layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear left
* Rear right
* Rear center
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_7P1

*Constant*: `super::Channels`

AAC 7.1-channel surround-based layout.

The channels in this layout are:
* Front center
* Front left-of-center
* Front right-of-center
* Front left
* Front right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_AAC_QUADRAPHONIC

*Constant*: `super::Channels`

AAC quadraphonic surround-based layout.

The channels in this layout are:
* Front left
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MONO

*Constant*: `super::Channels`

Monophonic audio.

The channels in this layout are:
* Front center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_1P0

*Constant*: `super::Channels`

MPEG 1-channel channel layout.

The channels in this layout are:
* Front center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_2P0

*Constant*: `super::Channels`

MPEG 2-channel channel layout.

The channels in this layout are:
* Front left
* Front right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_3P0_A

*Constant*: `super::Channels`

MPEG 3-channel, configuration A, channel layout.

The channels in this layout are:
* Front left
* Front right
* Front center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_3P0_B

*Constant*: `super::Channels`

MPEG 3-channel, configuration B, channel layout.

The channels in this layout are:
* Front center
* Front left
* Front right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_4P0_A

*Constant*: `super::Channels`

MPEG 4-channel, configuration A, channel layout.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_4P0_B

*Constant*: `super::Channels`

MPEG 4-channel, configuration B, channel layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P0_A

*Constant*: `super::Channels`

MPEG 5-channel, configuration A, channel layout.

The channels in this layout are:
* Front left
* Front right
* Front center
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P0_B

*Constant*: `super::Channels`

MPEG 5-channel, configuration B, channel layout.

The channels in this layout are:
* Front left
* Front right
* Rear left
* Rear right
* Front center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P0_C

*Constant*: `super::Channels`

MPEG 5-channel, configuration C, channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P0_D

*Constant*: `super::Channels`

MPEG 5-channel, configuration D, channel layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P1_A

*Constant*: `super::Channels`

MPEG 5.1-channel, configuration A, channel layout.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P1_B

*Constant*: `super::Channels`

MPEG 5.1-channel, configuration B, channel layout.

The channels in this layout are:
* Front left
* Front right
* Rear left
* Rear right
* Front center
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P1_C

*Constant*: `super::Channels`

MPEG 5.1-channel, configuration C, channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_5P1_D

*Constant*: `super::Channels`

MPEG 5.1-channel, configuration D, channel layout.

The channels in this layout are:
* Front center
* Front left
* Front right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_6P1_A

*Constant*: `super::Channels`

MPEG 6.1-channel, configuration A, channel layout.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right
* Rear center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_7P1_A

*Constant*: `super::Channels`

MPEG 7.1-channel, configuration A, channel layout.

The channels in this layout are:
* Front left
* Front right
* Front center
* Low-frequency effects
* Rear left
* Rear right
* Front left-of-center
* Front right-of-center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_MPEG_7P1_B

*Constant*: `super::Channels`

MPEG 7.1-channel, configuration B, channel layout.

The channels in this layout are:
* Front center
* Front left-of-center
* Front right-of-center
* Front left
* Front right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_1P0

*Constant*: `super::Channels`

OGG 1-channel layout.

The channels in this layout are:
* Front center



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_2P0

*Constant*: `super::Channels`

OGG 2-channel layout.

The channels in this layout are:
* Front left
* Front right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_3P0

*Constant*: `super::Channels`

OGG 3-channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_4P0

*Constant*: `super::Channels`

OGG 4-channel layout.

The channels in this layout are:
* Front left
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_5P0

*Constant*: `super::Channels`

OGG 5-channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right
* Rear left
* Rear right



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_5P1

*Constant*: `super::Channels`

OGG 5.1-channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_6P1

*Constant*: `super::Channels`

OGG 6.1-channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right
* Side left
* Side right
* Rear center
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_OGG_7P1

*Constant*: `super::Channels`

OGG 7.1-channel layout.

The channels in this layout are:
* Front left
* Front center
* Front right
* Side left
* Side right
* Rear left
* Rear right
* Low-frequency effects



## symphonia_core::audio::channels::layouts::CHANNEL_LAYOUT_STEREO

*Constant*: `super::Channels`

Stereophonic audio.

The channels in this layout are:
* Front left
* Front right



