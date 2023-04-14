//! ADXL355 register addresses
#![allow(non_camel_case_types)]

// See datasheet https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL375.PDF

#[allow(dead_code)]
#[repr(u8)]
pub enum Register {
    DEVID       	    = 0x00,
    THRESH_SHOCK	    = 0x1D,
    OFSX	            = 0x1E,
    OFSY	            = 0x1F,
    OFSZ	            = 0x20,
    DUR	                = 0x21,
    Latent	            = 0x22,
    Window	            = 0x23,
    THRESH_ACT	        = 0x24,
    THRESH_INACT	    = 0x25,
    TIME_INACT	        = 0x26,
    ACT_INACT_CTL	    = 0x27,
    SHOCK_AXES	        = 0x2A,
    ACT_SHOCK_STATUS	= 0x2B,
    BW_RATE	            = 0x2C,
    POWER_CTL	        = 0x2D,
    INT_ENABLE	        = 0x2E,
    INT_MAP	            = 0x2F,
    INT_SOURCE	        = 0x30,
    DATA_FORMAT	        = 0x31,
    DATAX0	            = 0x32,
    DATAX1	            = 0x33,
    DATAY0	            = 0x34,
    DATAY1	            = 0x35,
    DATAZ0	            = 0x36,
    DATAZ1	            = 0x37,
    FIFO_CTL	        = 0x38,
    FIFO_STATUS	        = 0x39
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }

}
