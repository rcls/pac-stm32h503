#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSSI_A {
    #[doc = "0: no extra delay"]
    B_0x0 = 0,
    #[doc = "1: 1 clock cycle period delay added"]
    B_0x1 = 1,
    #[doc = "15: 15 clock cycle periods delay added"]
    B_0xF = 15,
}
impl From<MSSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSSI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSSI_A {
    type Ux = u8;
}
impl crate::IsEnum for MSSI_A {}
#[doc = "Field `MSSI` reader - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
pub type MSSI_R = crate::FieldReader<MSSI_A>;
impl MSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSSI_A> {
        match self.bits {
            0 => Some(MSSI_A::B_0x0),
            1 => Some(MSSI_A::B_0x1),
            15 => Some(MSSI_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "no extra delay"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSSI_A::B_0x0
    }
    #[doc = "1 clock cycle period delay added"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSSI_A::B_0x1
    }
    #[doc = "15 clock cycle periods delay added"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == MSSI_A::B_0xF
    }
}
#[doc = "Field `MSSI` writer - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
pub type MSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSSI_A>;
impl<'a, REG> MSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no extra delay"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSSI_A::B_0x0)
    }
    #[doc = "1 clock cycle period delay added"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSSI_A::B_0x1)
    }
    #[doc = "15 clock cycle periods delay added"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(MSSI_A::B_0xF)
    }
}
#[doc = "master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIDI_A {
    #[doc = "0: no delay"]
    B_0x0 = 0,
    #[doc = "1: 1 clock cycle period delay"]
    B_0x1 = 1,
    #[doc = "15: 15 clock cycle periods delay"]
    B_0xF = 15,
}
impl From<MIDI_A> for u8 {
    #[inline(always)]
    fn from(variant: MIDI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIDI_A {
    type Ux = u8;
}
impl crate::IsEnum for MIDI_A {}
#[doc = "Field `MIDI` reader - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
pub type MIDI_R = crate::FieldReader<MIDI_A>;
impl MIDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MIDI_A> {
        match self.bits {
            0 => Some(MIDI_A::B_0x0),
            1 => Some(MIDI_A::B_0x1),
            15 => Some(MIDI_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "no delay"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIDI_A::B_0x0
    }
    #[doc = "1 clock cycle period delay"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIDI_A::B_0x1
    }
    #[doc = "15 clock cycle periods delay"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == MIDI_A::B_0xF
    }
}
#[doc = "Field `MIDI` writer - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
pub type MIDI_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MIDI_A>;
impl<'a, REG> MIDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no delay"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MIDI_A::B_0x0)
    }
    #[doc = "1 clock cycle period delay"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MIDI_A::B_0x1)
    }
    #[doc = "15 clock cycle periods delay"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(MIDI_A::B_0xF)
    }
}
#[doc = "RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIOM_A {
    #[doc = "0: RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)"]
    B_0x0 = 0,
    #[doc = "1: RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)"]
    B_0x1 = 1,
}
impl From<RDIOM_A> for bool {
    #[inline(always)]
    fn from(variant: RDIOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIOM` reader - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
pub type RDIOM_R = crate::BitReader<RDIOM_A>;
impl RDIOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDIOM_A {
        match self.bits {
            false => RDIOM_A::B_0x0,
            true => RDIOM_A::B_0x1,
        }
    }
    #[doc = "RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RDIOM_A::B_0x0
    }
    #[doc = "RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RDIOM_A::B_0x1
    }
}
#[doc = "Field `RDIOM` writer - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
pub type RDIOM_W<'a, REG> = crate::BitWriter<'a, REG, RDIOM_A>;
impl<'a, REG> RDIOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOM_A::B_0x0)
    }
    #[doc = "RDY signal is overtaken from alternate function input (at master case) or output (at slave case) of the dedicated pin (RDIOP setting takes effect)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOM_A::B_0x1)
    }
}
#[doc = "RDY signal input/output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIOP_A {
    #[doc = "0: high level of the signal means the slave is ready for communication"]
    B_0x0 = 0,
    #[doc = "1: low level of the signal means the slave is ready for communication"]
    B_0x1 = 1,
}
impl From<RDIOP_A> for bool {
    #[inline(always)]
    fn from(variant: RDIOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIOP` reader - RDY signal input/output polarity"]
pub type RDIOP_R = crate::BitReader<RDIOP_A>;
impl RDIOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDIOP_A {
        match self.bits {
            false => RDIOP_A::B_0x0,
            true => RDIOP_A::B_0x1,
        }
    }
    #[doc = "high level of the signal means the slave is ready for communication"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RDIOP_A::B_0x0
    }
    #[doc = "low level of the signal means the slave is ready for communication"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RDIOP_A::B_0x1
    }
}
#[doc = "Field `RDIOP` writer - RDY signal input/output polarity"]
pub type RDIOP_W<'a, REG> = crate::BitWriter<'a, REG, RDIOP_A>;
impl<'a, REG> RDIOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "high level of the signal means the slave is ready for communication"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOP_A::B_0x0)
    }
    #[doc = "low level of the signal means the slave is ready for communication"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RDIOP_A::B_0x1)
    }
}
#[doc = "swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSWP_A {
    #[doc = "0: no swap"]
    B_0x0 = 0,
    #[doc = "1: MOSI and MISO are swapped"]
    B_0x1 = 1,
}
impl From<IOSWP_A> for bool {
    #[inline(always)]
    fn from(variant: IOSWP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOSWP` reader - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
pub type IOSWP_R = crate::BitReader<IOSWP_A>;
impl IOSWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOSWP_A {
        match self.bits {
            false => IOSWP_A::B_0x0,
            true => IOSWP_A::B_0x1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IOSWP_A::B_0x0
    }
    #[doc = "MOSI and MISO are swapped"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IOSWP_A::B_0x1
    }
}
#[doc = "Field `IOSWP` writer - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG, IOSWP_A>;
impl<'a, REG> IOSWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IOSWP_A::B_0x0)
    }
    #[doc = "MOSI and MISO are swapped"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IOSWP_A::B_0x1)
    }
}
#[doc = "SPI Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMM_A {
    #[doc = "0: full-duplex"]
    B_0x0 = 0,
    #[doc = "1: simplex transmitter"]
    B_0x1 = 1,
    #[doc = "2: simplex receiver"]
    B_0x2 = 2,
    #[doc = "3: half-duplex"]
    B_0x3 = 3,
}
impl From<COMM_A> for u8 {
    #[inline(always)]
    fn from(variant: COMM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMM_A {
    type Ux = u8;
}
impl crate::IsEnum for COMM_A {}
#[doc = "Field `COMM` reader - SPI Communication Mode"]
pub type COMM_R = crate::FieldReader<COMM_A>;
impl COMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMM_A {
        match self.bits {
            0 => COMM_A::B_0x0,
            1 => COMM_A::B_0x1,
            2 => COMM_A::B_0x2,
            3 => COMM_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "full-duplex"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COMM_A::B_0x0
    }
    #[doc = "simplex transmitter"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COMM_A::B_0x1
    }
    #[doc = "simplex receiver"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == COMM_A::B_0x2
    }
    #[doc = "half-duplex"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == COMM_A::B_0x3
    }
}
#[doc = "Field `COMM` writer - SPI Communication Mode"]
pub type COMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COMM_A, crate::Safe>;
impl<'a, REG> COMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "full-duplex"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMM_A::B_0x0)
    }
    #[doc = "simplex transmitter"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMM_A::B_0x1)
    }
    #[doc = "simplex receiver"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(COMM_A::B_0x2)
    }
    #[doc = "half-duplex"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(COMM_A::B_0x3)
    }
}
#[doc = "serial protocol others: reserved, must not be used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SP_A {
    #[doc = "0: SPI Motorola"]
    B_0x0 = 0,
    #[doc = "1: SPI TI"]
    B_0x1 = 1,
}
impl From<SP_A> for u8 {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SP_A {
    type Ux = u8;
}
impl crate::IsEnum for SP_A {}
#[doc = "Field `SP` reader - serial protocol others: reserved, must not be used"]
pub type SP_R = crate::FieldReader<SP_A>;
impl SP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SP_A> {
        match self.bits {
            0 => Some(SP_A::B_0x0),
            1 => Some(SP_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "SPI Motorola"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SP_A::B_0x0
    }
    #[doc = "SPI TI"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SP_A::B_0x1
    }
}
#[doc = "Field `SP` writer - serial protocol others: reserved, must not be used"]
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SP_A>;
impl<'a, REG> SP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Motorola"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SP_A::B_0x0)
    }
    #[doc = "SPI TI"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SP_A::B_0x1)
    }
}
#[doc = "SPI Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: SPI Slave"]
    B_0x0 = 0,
    #[doc = "1: SPI Master"]
    B_0x1 = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - SPI Master"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::B_0x0,
            true => MASTER_A::B_0x1,
        }
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MASTER_A::B_0x0
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MASTER_A::B_0x1
    }
}
#[doc = "Field `MASTER` writer - SPI Master"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG, MASTER_A>;
impl<'a, REG> MASTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER_A::B_0x0)
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER_A::B_0x1)
    }
}
#[doc = "data frame format Note: This bit can be also used in PCM and I2S modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFRST_A {
    #[doc = "0: MSB transmitted first"]
    B_0x0 = 0,
    #[doc = "1: LSB transmitted first"]
    B_0x1 = 1,
}
impl From<LSBFRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFRST` reader - data frame format Note: This bit can be also used in PCM and I2S modes."]
pub type LSBFRST_R = crate::BitReader<LSBFRST_A>;
impl LSBFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSBFRST_A {
        match self.bits {
            false => LSBFRST_A::B_0x0,
            true => LSBFRST_A::B_0x1,
        }
    }
    #[doc = "MSB transmitted first"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSBFRST_A::B_0x0
    }
    #[doc = "LSB transmitted first"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSBFRST_A::B_0x1
    }
}
#[doc = "Field `LSBFRST` writer - data frame format Note: This bit can be also used in PCM and I2S modes."]
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFRST_A>;
impl<'a, REG> LSBFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB transmitted first"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFRST_A::B_0x0)
    }
    #[doc = "LSB transmitted first"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFRST_A::B_0x1)
    }
}
#[doc = "clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: the first clock transition is the first data capture edge"]
    B_0x0 = 0,
    #[doc = "1: the second clock transition is the first data capture edge"]
    B_0x1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - clock phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::B_0x0,
            true => CPHA_A::B_0x1,
        }
    }
    #[doc = "the first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CPHA_A::B_0x0
    }
    #[doc = "the second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CPHA_A::B_0x1
    }
}
#[doc = "Field `CPHA` writer - clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA_A>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::B_0x0)
    }
    #[doc = "the second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::B_0x1)
    }
}
#[doc = "clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: SCK signal is at 0 when idle"]
    B_0x0 = 0,
    #[doc = "1: SCK signal is at 1 when idle"]
    B_0x1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - clock polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::B_0x0,
            true => CPOL_A::B_0x1,
        }
    }
    #[doc = "SCK signal is at 0 when idle"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CPOL_A::B_0x0
    }
    #[doc = "SCK signal is at 1 when idle"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CPOL_A::B_0x1
    }
}
#[doc = "Field `CPOL` writer - clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL_A>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCK signal is at 0 when idle"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::B_0x0)
    }
    #[doc = "SCK signal is at 1 when idle"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::B_0x1)
    }
}
#[doc = "software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM_A {
    #[doc = "0: SS input value is determined by the SS PAD"]
    B_0x0 = 0,
    #[doc = "1: SS input value is determined by the SSI bit"]
    B_0x1 = 1,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSM` reader - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
pub type SSM_R = crate::BitReader<SSM_A>;
impl SSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::B_0x0,
            true => SSM_A::B_0x1,
        }
    }
    #[doc = "SS input value is determined by the SS PAD"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SSM_A::B_0x0
    }
    #[doc = "SS input value is determined by the SSI bit"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SSM_A::B_0x1
    }
}
#[doc = "Field `SSM` writer - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG, SSM_A>;
impl<'a, REG> SSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS input value is determined by the SS PAD"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSM_A::B_0x0)
    }
    #[doc = "SS input value is determined by the SSI bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSM_A::B_0x1)
    }
}
#[doc = "SS input/output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIOP_A {
    #[doc = "0: low level is active for SS signal"]
    B_0x0 = 0,
    #[doc = "1: high level is active for SS signal"]
    B_0x1 = 1,
}
impl From<SSIOP_A> for bool {
    #[inline(always)]
    fn from(variant: SSIOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSIOP` reader - SS input/output polarity"]
pub type SSIOP_R = crate::BitReader<SSIOP_A>;
impl SSIOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSIOP_A {
        match self.bits {
            false => SSIOP_A::B_0x0,
            true => SSIOP_A::B_0x1,
        }
    }
    #[doc = "low level is active for SS signal"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SSIOP_A::B_0x0
    }
    #[doc = "high level is active for SS signal"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SSIOP_A::B_0x1
    }
}
#[doc = "Field `SSIOP` writer - SS input/output polarity"]
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG, SSIOP_A>;
impl<'a, REG> SSIOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "low level is active for SS signal"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSIOP_A::B_0x0)
    }
    #[doc = "high level is active for SS signal"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSIOP_A::B_0x1)
    }
}
#[doc = "SS output enable This bit is taken into account in Master mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled and the SPI can work in multi-master configuration"]
    B_0x0 = 0,
    #[doc = "1: SS output is enabled. The SPI cannot work in a multi-master environment. It forces the SS pin at inactive level after the transfer is completed or SPI is disabled with respect to SSOM, MIDI, MSSI, SSIOP bits setting"]
    B_0x1 = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable This bit is taken into account in Master mode only"]
pub type SSOE_R = crate::BitReader<SSOE_A>;
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::B_0x0,
            true => SSOE_A::B_0x1,
        }
    }
    #[doc = "SS output is disabled and the SPI can work in multi-master configuration"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SSOE_A::B_0x0
    }
    #[doc = "SS output is enabled. The SPI cannot work in a multi-master environment. It forces the SS pin at inactive level after the transfer is completed or SPI is disabled with respect to SSOM, MIDI, MSSI, SSIOP bits setting"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SSOE_A::B_0x1
    }
}
#[doc = "Field `SSOE` writer - SS output enable This bit is taken into account in Master mode only"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE_A>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS output is disabled and the SPI can work in multi-master configuration"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE_A::B_0x0)
    }
    #[doc = "SS output is enabled. The SPI cannot work in a multi-master environment. It forces the SS pin at inactive level after the transfer is completed or SPI is disabled with respect to SSOM, MIDI, MSSI, SSIOP bits setting"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE_A::B_0x1)
    }
}
#[doc = "SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOM_A {
    #[doc = "0: SS is kept at active level till data transfer is completed, it becomes inactive with EOT flag"]
    B_0x0 = 0,
    #[doc = "1: SPI data frames are interleaved with SS non active pulses when MIDI\\[3:0\\]1"]
    B_0x1 = 1,
}
impl From<SSOM_A> for bool {
    #[inline(always)]
    fn from(variant: SSOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOM` reader - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
pub type SSOM_R = crate::BitReader<SSOM_A>;
impl SSOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSOM_A {
        match self.bits {
            false => SSOM_A::B_0x0,
            true => SSOM_A::B_0x1,
        }
    }
    #[doc = "SS is kept at active level till data transfer is completed, it becomes inactive with EOT flag"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SSOM_A::B_0x0
    }
    #[doc = "SPI data frames are interleaved with SS non active pulses when MIDI\\[3:0\\]1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SSOM_A::B_0x1
    }
}
#[doc = "Field `SSOM` writer - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG, SSOM_A>;
impl<'a, REG> SSOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS is kept at active level till data transfer is completed, it becomes inactive with EOT flag"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSOM_A::B_0x0)
    }
    #[doc = "SPI data frames are interleaved with SS non active pulses when MIDI\\[3:0\\]1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSOM_A::B_0x1)
    }
}
#[doc = "alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFCNTR_A {
    #[doc = "0: The peripheral takes no control of GPIOs while it is disabled"]
    B_0x0 = 0,
    #[doc = "1: The peripheral keeps always control of all associated GPIOs"]
    B_0x1 = 1,
}
impl From<AFCNTR_A> for bool {
    #[inline(always)]
    fn from(variant: AFCNTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFCNTR` reader - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
pub type AFCNTR_R = crate::BitReader<AFCNTR_A>;
impl AFCNTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFCNTR_A {
        match self.bits {
            false => AFCNTR_A::B_0x0,
            true => AFCNTR_A::B_0x1,
        }
    }
    #[doc = "The peripheral takes no control of GPIOs while it is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFCNTR_A::B_0x0
    }
    #[doc = "The peripheral keeps always control of all associated GPIOs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFCNTR_A::B_0x1
    }
}
#[doc = "Field `AFCNTR` writer - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG, AFCNTR_A>;
impl<'a, REG> AFCNTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral takes no control of GPIOs while it is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFCNTR_A::B_0x0)
    }
    #[doc = "The peripheral keeps always control of all associated GPIOs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFCNTR_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
    #[inline(always)]
    pub fn MSSI(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
    #[inline(always)]
    pub fn MIDI(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
    #[inline(always)]
    pub fn RDIOM(&self) -> RDIOM_R {
        RDIOM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RDY signal input/output polarity"]
    #[inline(always)]
    pub fn RDIOP(&self) -> RDIOP_R {
        RDIOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
    #[inline(always)]
    pub fn IOSWP(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn COMM(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - serial protocol others: reserved, must not be used"]
    #[inline(always)]
    pub fn SP(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn MASTER(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - data frame format Note: This bit can be also used in PCM and I2S modes."]
    #[inline(always)]
    pub fn LSBFRST(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - clock phase"]
    #[inline(always)]
    pub fn CPHA(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - clock polarity"]
    #[inline(always)]
    pub fn CPOL(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
    #[inline(always)]
    pub fn SSM(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn SSIOP(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SS output enable This bit is taken into account in Master mode only"]
    #[inline(always)]
    pub fn SSOE(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
    #[inline(always)]
    pub fn SSOM(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
    #[inline(always)]
    pub fn AFCNTR(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
    #[inline(always)]
    pub fn MSSI(&mut self) -> MSSI_W<'_, CFG2_SPEC> {
        MSSI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
    #[inline(always)]
    pub fn MIDI(&mut self) -> MIDI_W<'_, CFG2_SPEC> {
        MIDI_W::new(self, 4)
    }
    #[doc = "Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
    #[inline(always)]
    pub fn RDIOM(&mut self) -> RDIOM_W<'_, CFG2_SPEC> {
        RDIOM_W::new(self, 13)
    }
    #[doc = "Bit 14 - RDY signal input/output polarity"]
    #[inline(always)]
    pub fn RDIOP(&mut self) -> RDIOP_W<'_, CFG2_SPEC> {
        RDIOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
    #[inline(always)]
    pub fn IOSWP(&mut self) -> IOSWP_W<'_, CFG2_SPEC> {
        IOSWP_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn COMM(&mut self) -> COMM_W<'_, CFG2_SPEC> {
        COMM_W::new(self, 17)
    }
    #[doc = "Bits 19:21 - serial protocol others: reserved, must not be used"]
    #[inline(always)]
    pub fn SP(&mut self) -> SP_W<'_, CFG2_SPEC> {
        SP_W::new(self, 19)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn MASTER(&mut self) -> MASTER_W<'_, CFG2_SPEC> {
        MASTER_W::new(self, 22)
    }
    #[doc = "Bit 23 - data frame format Note: This bit can be also used in PCM and I2S modes."]
    #[inline(always)]
    pub fn LSBFRST(&mut self) -> LSBFRST_W<'_, CFG2_SPEC> {
        LSBFRST_W::new(self, 23)
    }
    #[doc = "Bit 24 - clock phase"]
    #[inline(always)]
    pub fn CPHA(&mut self) -> CPHA_W<'_, CFG2_SPEC> {
        CPHA_W::new(self, 24)
    }
    #[doc = "Bit 25 - clock polarity"]
    #[inline(always)]
    pub fn CPOL(&mut self) -> CPOL_W<'_, CFG2_SPEC> {
        CPOL_W::new(self, 25)
    }
    #[doc = "Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
    #[inline(always)]
    pub fn SSM(&mut self) -> SSM_W<'_, CFG2_SPEC> {
        SSM_W::new(self, 26)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn SSIOP(&mut self) -> SSIOP_W<'_, CFG2_SPEC> {
        SSIOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - SS output enable This bit is taken into account in Master mode only"]
    #[inline(always)]
    pub fn SSOE(&mut self) -> SSOE_W<'_, CFG2_SPEC> {
        SSOE_W::new(self, 29)
    }
    #[doc = "Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
    #[inline(always)]
    pub fn SSOM(&mut self) -> SSOM_W<'_, CFG2_SPEC> {
        SSOM_W::new(self, 30)
    }
    #[doc = "Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
    #[inline(always)]
    pub fn AFCNTR(&mut self) -> AFCNTR_W<'_, CFG2_SPEC> {
        AFCNTR_W::new(self, 31)
    }
}
#[doc = "SPI/I2S configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {}
