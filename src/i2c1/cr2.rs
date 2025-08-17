#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `SADD` reader - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\] should be written with the 7-bit slave address to be sent. The bits SADD\\[9\\], SADD\\[8\\] and SADD\\[0\\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\] should be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
pub type SADD_R = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\] should be written with the 7-bit slave address to be sent. The bits SADD\\[9\\], SADD\\[8\\] and SADD\\[0\\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\] should be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN_A {
    #[doc = "0: Master requests a write transfer."]
    B_0x0 = 0,
    #[doc = "1: Master requests a read transfer."]
    B_0x1 = 1,
}
impl From<RD_WRN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type RD_WRN_R = crate::BitReader<RD_WRN_A>;
impl RD_WRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RD_WRN_A {
        match self.bits {
            false => RD_WRN_A::B_0x0,
            true => RD_WRN_A::B_0x1,
        }
    }
    #[doc = "Master requests a write transfer."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RD_WRN_A::B_0x0
    }
    #[doc = "Master requests a read transfer."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RD_WRN_A::B_0x1
    }
}
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG, RD_WRN_A>;
impl<'a, REG> RD_WRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master requests a write transfer."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN_A::B_0x0)
    }
    #[doc = "Master requests a read transfer."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN_A::B_0x1)
    }
}
#[doc = "10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10_A {
    #[doc = "0: The master operates in 7-bit addressing mode,"]
    B_0x0 = 0,
    #[doc = "1: The master operates in 10-bit addressing mode"]
    B_0x1 = 1,
}
impl From<ADD10_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type ADD10_R = crate::BitReader<ADD10_A>;
impl ADD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADD10_A {
        match self.bits {
            false => ADD10_A::B_0x0,
            true => ADD10_A::B_0x1,
        }
    }
    #[doc = "The master operates in 7-bit addressing mode,"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADD10_A::B_0x0
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADD10_A::B_0x1
    }
}
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG, ADD10_A>;
impl<'a, REG> ADD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The master operates in 7-bit addressing mode,"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10_A::B_0x0)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10_A::B_0x1)
    }
}
#[doc = "10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R_A {
    #[doc = "0: The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction."]
    B_0x0 = 0,
    #[doc = "1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
    B_0x1 = 1,
}
impl From<HEAD10R_A> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type HEAD10R_R = crate::BitReader<HEAD10R_A>;
impl HEAD10R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HEAD10R_A {
        match self.bits {
            false => HEAD10R_A::B_0x0,
            true => HEAD10R_A::B_0x1,
        }
    }
    #[doc = "The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HEAD10R_A::B_0x0
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HEAD10R_A::B_0x1
    }
}
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG, HEAD10R_A>;
impl<'a, REG> HEAD10R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The master sends the complete 10 bit slave address read sequence: Start + 2 bytes 10bit address in write direction + Restart + 1st 7 bits of the 10 bit address in read direction."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R_A::B_0x0)
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R_A::B_0x1)
    }
}
#[doc = "Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit generates a START condition once the bus is free. Note: Writing '0' to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: No Start generation."]
    B_0x0 = 0,
    #[doc = "1: Restart/Start generation:"]
    B_0x1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit generates a START condition once the bus is free. Note: Writing '0' to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
pub type START_R = crate::BitReader<START_A>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::B_0x0,
            true => START_A::B_0x1,
        }
    }
    #[doc = "No Start generation."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == START_A::B_0x0
    }
    #[doc = "Restart/Start generation:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == START_A::B_0x1
    }
}
#[doc = "Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit generates a START condition once the bus is free. Note: Writing '0' to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START_A>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Start generation."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::B_0x0)
    }
    #[doc = "Restart/Start generation:"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::B_0x1)
    }
}
#[doc = "Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: Note: Writing '0' to this bit has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: No Stop generation."]
    B_0x0 = 0,
    #[doc = "1: Stop generation after current byte transfer."]
    B_0x1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: Note: Writing '0' to this bit has no effect."]
pub type STOP_R = crate::BitReader<STOP_A>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::B_0x0,
            true => STOP_A::B_0x1,
        }
    }
    #[doc = "No Stop generation."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOP_A::B_0x0
    }
    #[doc = "Stop generation after current byte transfer."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOP_A::B_0x1
    }
}
#[doc = "Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: Note: Writing '0' to this bit has no effect."]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP_A>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Stop generation."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x0)
    }
    #[doc = "Stop generation after current byte transfer."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x1)
    }
}
#[doc = "NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing '0' to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK_A {
    #[doc = "0: an ACK is sent after current received byte."]
    B_0x0 = 0,
    #[doc = "1: a NACK is sent after current received byte."]
    B_0x1 = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing '0' to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
pub type NACK_R = crate::BitReader<NACK_A>;
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::B_0x0,
            true => NACK_A::B_0x1,
        }
    }
    #[doc = "an ACK is sent after current received byte."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NACK_A::B_0x0
    }
    #[doc = "a NACK is sent after current received byte."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NACK_A::B_0x1
    }
}
#[doc = "Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing '0' to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG, NACK_A>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an ACK is sent after current received byte."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NACK_A::B_0x0)
    }
    #[doc = "a NACK is sent after current received byte."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NACK_A::B_0x1)
    }
}
#[doc = "Field `NBYTES` reader - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don't care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
pub type NBYTES_R = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don't care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "NBYTES reload mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_A {
    #[doc = "0: The transfer is completed after the NBYTES data transfer (STOP or RESTART follows)."]
    B_0x0 = 0,
    #[doc = "1: The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    B_0x1 = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software."]
pub type RELOAD_R = crate::BitReader<RELOAD_A>;
impl RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::B_0x0,
            true => RELOAD_A::B_0x1,
        }
    }
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART follows)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RELOAD_A::B_0x0
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RELOAD_A::B_0x1
    }
}
#[doc = "Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software."]
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG, RELOAD_A>;
impl<'a, REG> RELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART follows)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD_A::B_0x0)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD_A::B_0x1)
    }
}
#[doc = "Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND_A {
    #[doc = "0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low."]
    B_0x0 = 0,
    #[doc = "1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    B_0x1 = 1,
}
impl From<AUTOEND_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
pub type AUTOEND_R = crate::BitReader<AUTOEND_A>;
impl AUTOEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOEND_A {
        match self.bits {
            false => AUTOEND_A::B_0x0,
            true => AUTOEND_A::B_0x1,
        }
    }
    #[doc = "software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AUTOEND_A::B_0x0
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AUTOEND_A::B_0x1
    }
}
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG, AUTOEND_A>;
impl<'a, REG> AUTOEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND_A::B_0x0)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND_A::B_0x1)
    }
}
#[doc = "Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing '0' to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTE_A {
    #[doc = "0: No PEC transfer."]
    B_0x0 = 0,
    #[doc = "1: PEC transmission/reception is requested"]
    B_0x1 = 1,
}
impl From<PECBYTE_A> for bool {
    #[inline(always)]
    fn from(variant: PECBYTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECBYTE` reader - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing '0' to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PECBYTE_R = crate::BitReader<PECBYTE_A>;
impl PECBYTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PECBYTE_A {
        match self.bits {
            false => PECBYTE_A::B_0x0,
            true => PECBYTE_A::B_0x1,
        }
    }
    #[doc = "No PEC transfer."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PECBYTE_A::B_0x0
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PECBYTE_A::B_0x1
    }
}
#[doc = "Field `PECBYTE` writer - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing '0' to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG, PECBYTE_A>;
impl<'a, REG> PECBYTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PEC transfer."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PECBYTE_A::B_0x0)
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PECBYTE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\] should be written with the 7-bit slave address to be sent. The bits SADD\\[9\\], SADD\\[8\\] and SADD\\[0\\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\] should be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn SADD(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn RD_WRN(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn ADD10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn HEAD10R(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit generates a START condition once the bus is free. Note: Writing '0' to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
    #[inline(always)]
    pub fn START(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: Note: Writing '0' to this bit has no effect."]
    #[inline(always)]
    pub fn STOP(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing '0' to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
    #[inline(always)]
    pub fn NACK(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don't care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn NBYTES(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn RELOAD(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
    #[inline(always)]
    pub fn AUTOEND(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing '0' to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn PECBYTE(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\\[7:1\\] should be written with the 7-bit slave address to be sent. The bits SADD\\[9\\], SADD\\[8\\] and SADD\\[0\\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\\[9:0\\] should be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn SADD(&mut self) -> SADD_W<'_, CR2_SPEC> {
        SADD_W::new(self, 0)
    }
    #[doc = "Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn RD_WRN(&mut self) -> RD_WRN_W<'_, CR2_SPEC> {
        RD_WRN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn ADD10(&mut self) -> ADD10_W<'_, CR2_SPEC> {
        ADD10_W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn HEAD10R(&mut self) -> HEAD10R_W<'_, CR2_SPEC> {
        HEAD10R_W::new(self, 12)
    }
    #[doc = "Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated Start condition when RELOAD=0, after the end of the NBYTES transfer. Otherwise setting this bit generates a START condition once the bus is free. Note: Writing '0' to this bit has no effect. The START bit can be set even if the bus is BUSY or I2C is in slave mode. This bit has no effect when RELOAD is set."]
    #[inline(always)]
    pub fn START(&mut self) -> START_W<'_, CR2_SPEC> {
        START_W::new(self, 13)
    }
    #[doc = "Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In Master Mode: Note: Writing '0' to this bit has no effect."]
    #[inline(always)]
    pub fn STOP(&mut self) -> STOP_W<'_, CR2_SPEC> {
        STOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE=0. Note: Writing '0' to this bit has no effect. This bit is used in slave mode only: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated whatever the NACK bit value. When hardware PEC checking is enabled (PECBYTE=1), the PEC acknowledge value does not depend on the NACK value."]
    #[inline(always)]
    pub fn NACK(&mut self) -> NACK_W<'_, CR2_SPEC> {
        NACK_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don't care in slave mode with SBC=0. Note: Changing these bits when the START bit is set is not allowed."]
    #[inline(always)]
    pub fn NBYTES(&mut self) -> NBYTES_W<'_, CR2_SPEC> {
        NBYTES_W::new(self, 16)
    }
    #[doc = "Bit 24 - NBYTES reload mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn RELOAD(&mut self) -> RELOAD_W<'_, CR2_SPEC> {
        RELOAD_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set."]
    #[inline(always)]
    pub fn AUTOEND(&mut self) -> AUTOEND_W<'_, CR2_SPEC> {
        AUTOEND_W::new(self, 25)
    }
    #[doc = "Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE=0. Note: Writing '0' to this bit has no effect. This bit has no effect when RELOAD is set. This bit has no effect is slave mode when SBC=0. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0'. Refer to ."]
    #[inline(always)]
    pub fn PECBYTE(&mut self) -> PECBYTE_W<'_, CR2_SPEC> {
        PECBYTE_W::new(self, 26)
    }
}
#[doc = "I2C control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {}
