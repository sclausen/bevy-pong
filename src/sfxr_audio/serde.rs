use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::Deserialize;
use std::fmt;

#[derive(Copy, Clone, Deserialize)]
#[serde(remote = "sfxr::Sample")]
pub struct SampleDef {
	#[serde(deserialize_with = "from_u8")]
	pub wave_type: sfxr::WaveType,
	#[serde(rename = "p_base_freq")]
	pub base_freq: f64,
	#[serde(rename = "p_freq_limit")]
	pub freq_limit: f64,
	#[serde(rename = "p_freq_ramp")]
	pub freq_ramp: f64,
	#[serde(rename = "p_freq_dramp")]
	pub freq_dramp: f64,
	#[serde(rename = "p_duty")]
	pub duty: f32,
	#[serde(rename = "p_duty_ramp")]
	pub duty_ramp: f32,

	#[serde(rename = "p_vib_strength")]
	pub vib_strength: f64,
	#[serde(rename = "p_vib_speed")]
	pub vib_speed: f64,
	#[serde(default)]
	pub vib_delay: f32,

	#[serde(rename = "p_env_attack")]
	pub env_attack: f32,
	#[serde(rename = "p_env_sustain")]
	pub env_sustain: f32,
	#[serde(rename = "p_env_decay")]
	pub env_decay: f32,
	#[serde(rename = "p_env_punch")]
	pub env_punch: f32,

	#[serde(rename = "p_lpf_resonance")]
	pub lpf_resonance: f32,
	#[serde(rename = "p_lpf_freq")]
	pub lpf_freq: f32,
	#[serde(rename = "p_lpf_ramp")]
	pub lpf_ramp: f32,
	#[serde(rename = "p_hpf_freq")]
	pub hpf_freq: f32,
	#[serde(rename = "p_hpf_ramp")]
	pub hpf_ramp: f32,

	#[serde(rename = "p_pha_offset")]
	pub pha_offset: f32,
	#[serde(rename = "p_pha_ramp")]
	pub pha_ramp: f32,

	#[serde(rename = "p_repeat_speed")]
	pub repeat_speed: f32,

	#[serde(rename = "p_arp_speed")]
	pub arp_speed: f32,
	#[serde(rename = "p_arp_mod")]
	pub arp_mod: f64,
}

fn from_u8<'de, D>(deserializer: D) -> Result<sfxr::WaveType, D::Error>
where
	D: Deserializer<'de>,
{
	struct U8Visitor;

	impl<'de> Visitor<'de> for U8Visitor {
		type Value = sfxr::WaveType;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("a valid u8 representing a WaveType")
		}

		fn visit_u64<E: de::Error>(self, value: u64) -> Result<sfxr::WaveType, E> {
			match value {
				0 => Ok(sfxr::WaveType::Square),
				1 => Ok(sfxr::WaveType::Sawtooth),
				2 => Ok(sfxr::WaveType::Sine),
				3 => Ok(sfxr::WaveType::Triangle),
				4 => Ok(sfxr::WaveType::Noise),
				_ => Err(E::invalid_value(
					Unexpected::Unsigned(value),
					&"a value between 0 and 4",
				)),
			}
		}
	}

	deserializer.deserialize_u64(U8Visitor)
}
