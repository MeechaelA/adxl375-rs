#![allow(non_camel_case_types)]

#[derive(Copy, Clone, Debug)]
/// Output data rate (odr) and Low pass filter corner frequency (lpf)
pub enum ODR_LPF {
    ODR_3200_Hz     = 15,
    ODR_1600_Hz     = 14,
    ODR_800_Hz      = 13,
    ODR_400_Hz      = 12,
    ODR_200_Hz      = 11,
    ODR_100_Hz      = 10,
    ODR_50_Hz       = 9,
    ODR_25_Hz       = 8,
    ODR_12_5_Hz     = 7,
    ODR_6_25_Hz     = 6,
    ODR_3_13_Hz     = 5,
    ODR_1_56_Hz     = 4,
    ODR_0_78_Hz     = 3,
    ODR_0_39_Hz     = 2,
    ODR_0_20_Hz     = 1,
    ODR_0_10_Hz     = 0
}

impl ODR_LPF {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl From<ODR_LPF> for f32 {
    fn from(rate: ODR_LPF) -> f32 {
        match rate {
            ODR_3200_Hz => 3200.0,
            ODR_1600_Hz => 1600.0, 
            ODR_800_Hz  => 800.0,
            ODR_400_Hz  => 400.0,
            ODR_200_Hz  => 200.0,
            ODR_100_Hz  => 100.0,
            ODR_50_Hz   => 50.0,
            ODR_25_Hz   => 25.0,
            ODR_12_5_Hz => 12.5,
            ODR_6_25_Hz => 6.25,
            ODR_3_13_Hz => 3.13,
            ODR_1_56_Hz => 1.56,
            ODR_0_78_Hz => 0.78,
            ODR_0_39_Hz => 0.39,
            ODR_0_20_Hz => 0.20,
            ODR_0_10_Hz => 0.10
        }
    }
}

impl Default for ODR_LPF {
    fn default() -> Self {
        ODR_LPF::ODR_100_Hz
    }
}

pub struct Config {
    pub(crate) odr: Option<ODR_LPF>,
}

/// ADXL355 configuration struct
impl Config {

    // Creates a new configuration object with default values
    pub fn new() -> Self {
        Config {
            odr: Some(ODR_LPF::ODR_100_Hz),
        }
    }

    /// Sets the output data rate and low pass filter settings.
    /// Default data rate is `100 Hz`
    pub fn odr(&mut self, odr: ODR_LPF) -> &mut Self {
        self.odr = Some(odr);
        self
    }

}
