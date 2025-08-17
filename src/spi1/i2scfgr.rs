#[doc = "Register `I2SCFGR` reader"]
pub type R = crate::R<I2SCFGR_SPEC>;
#[doc = "Register `I2SCFGR` writer"]
pub type W = crate::W<I2SCFGR_SPEC>;
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD_A {
    #[doc = "0: SPI mode is selected"]
    B_0x0 = 0,
    #[doc = "1: I2S/PCM mode is selected"]
    B_0x1 = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader<I2SMOD_A>;
impl I2SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::B_0x0,
            true => I2SMOD_A::B_0x1,
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2SMOD_A::B_0x0
    }
    #[doc = "I2S/PCM mode is selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2SMOD_A::B_0x1
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD_A>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD_A::B_0x0)
    }
    #[doc = "I2S/PCM mode is selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD_A::B_0x1)
    }
}
#[doc = "I2S configuration mode others, not used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG_A {
    #[doc = "0: slave - transmit"]
    B_0x0 = 0,
    #[doc = "1: slave - receive"]
    B_0x1 = 1,
    #[doc = "2: master - transmit"]
    B_0x2 = 2,
    #[doc = "3: master - receive"]
    B_0x3 = 3,
    #[doc = "4: slave - Full Duplex"]
    B_0x4 = 4,
    #[doc = "5: master - Full Duplex"]
    B_0x5 = 5,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SCFG_A {
    type Ux = u8;
}
impl crate::IsEnum for I2SCFG_A {}
#[doc = "Field `I2SCFG` reader - I2S configuration mode others, not used"]
pub type I2SCFG_R = crate::FieldReader<I2SCFG_A>;
impl I2SCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2SCFG_A> {
        match self.bits {
            0 => Some(I2SCFG_A::B_0x0),
            1 => Some(I2SCFG_A::B_0x1),
            2 => Some(I2SCFG_A::B_0x2),
            3 => Some(I2SCFG_A::B_0x3),
            4 => Some(I2SCFG_A::B_0x4),
            5 => Some(I2SCFG_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "slave - transmit"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2SCFG_A::B_0x0
    }
    #[doc = "slave - receive"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2SCFG_A::B_0x1
    }
    #[doc = "master - transmit"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == I2SCFG_A::B_0x2
    }
    #[doc = "master - receive"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == I2SCFG_A::B_0x3
    }
    #[doc = "slave - Full Duplex"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == I2SCFG_A::B_0x4
    }
    #[doc = "master - Full Duplex"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == I2SCFG_A::B_0x5
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode others, not used"]
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, I2SCFG_A>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "slave - transmit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0x0)
    }
    #[doc = "slave - receive"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0x1)
    }
    #[doc = "master - transmit"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0x2)
    }
    #[doc = "master - receive"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0x3)
    }
    #[doc = "slave - Full Duplex"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0x4)
    }
    #[doc = "master - Full Duplex"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0x5)
    }
}
#[doc = "I2S standard selection For more details on I2S standards, refer to\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD_A {
    #[doc = "0: I2S Philips standard."]
    B_0x0 = 0,
    #[doc = "1: MSB justified standard (left justified)"]
    B_0x1 = 1,
    #[doc = "2: LSB justified standard (right justified)"]
    B_0x2 = 2,
    #[doc = "3: PCM standard"]
    B_0x3 = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSTD_A {
    type Ux = u8;
}
impl crate::IsEnum for I2SSTD_A {}
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to"]
pub type I2SSTD_R = crate::FieldReader<I2SSTD_A>;
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::B_0x0,
            1 => I2SSTD_A::B_0x1,
            2 => I2SSTD_A::B_0x2,
            3 => I2SSTD_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S Philips standard."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2SSTD_A::B_0x0
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2SSTD_A::B_0x1
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == I2SSTD_A::B_0x2
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == I2SSTD_A::B_0x3
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to"]
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SSTD_A, crate::Safe>;
impl<'a, REG> I2SSTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips standard."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0x0)
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0x1)
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0x2)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0x3)
    }
}
#[doc = "PCM frame synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC_A {
    #[doc = "0: short frame synchronization"]
    B_0x0 = 0,
    #[doc = "1: long frame synchronization"]
    B_0x1 = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PCMSYNC_R = crate::BitReader<PCMSYNC_A>;
impl PCMSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::B_0x0,
            true => PCMSYNC_A::B_0x1,
        }
    }
    #[doc = "short frame synchronization"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PCMSYNC_A::B_0x0
    }
    #[doc = "long frame synchronization"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PCMSYNC_A::B_0x1
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC_A>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "short frame synchronization"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC_A::B_0x0)
    }
    #[doc = "long frame synchronization"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC_A::B_0x1)
    }
}
#[doc = "data length to be transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: 16-bit data length"]
    B_0x0 = 0,
    #[doc = "1: 24-bit data length"]
    B_0x1 = 1,
    #[doc = "2: 32-bit data length"]
    B_0x2 = 2,
    #[doc = "3: Not allowed"]
    B_0x3 = 3,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATLEN_A {
    type Ux = u8;
}
impl crate::IsEnum for DATLEN_A {}
#[doc = "Field `DATLEN` reader - data length to be transferred"]
pub type DATLEN_R = crate::FieldReader<DATLEN_A>;
impl DATLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATLEN_A {
        match self.bits {
            0 => DATLEN_A::B_0x0,
            1 => DATLEN_A::B_0x1,
            2 => DATLEN_A::B_0x2,
            3 => DATLEN_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DATLEN_A::B_0x0
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DATLEN_A::B_0x1
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DATLEN_A::B_0x2
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == DATLEN_A::B_0x3
    }
}
#[doc = "Field `DATLEN` writer - data length to be transferred"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATLEN_A, crate::Safe>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0x0)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0x1)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0x2)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0x3)
    }
}
#[doc = "channel length (number of bits per audio channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN_A {
    #[doc = "0: 16-bit wide"]
    B_0x0 = 0,
    #[doc = "1: 32-bit wide"]
    B_0x1 = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHLEN` reader - channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::B_0x0,
            true => CHLEN_A::B_0x1,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CHLEN_A::B_0x0
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CHLEN_A::B_0x1
    }
}
#[doc = "Field `CHLEN` writer - channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN_A>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN_A::B_0x0)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN_A::B_0x1)
    }
}
#[doc = "serial audio clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    #[doc = "0: the signals generated by the SPI/I2S (i.e. SDO and WS) are changed on the falling edge of CK and the signals received by the SPI/I2S (i.e. SDI and WS) are read of the rising edge of CK."]
    B_0x0 = 0,
    #[doc = "1: the signals generated by the SPI/I2S (i.e. SDO and WS) are changed on the rising edge of CK and the signals received by the SPI/I2S (i.e. SDI and WS) are read of the falling edge of CK."]
    B_0x1 = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - serial audio clock polarity"]
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::B_0x0,
            true => CKPOL_A::B_0x1,
        }
    }
    #[doc = "the signals generated by the SPI/I2S (i.e. SDO and WS) are changed on the falling edge of CK and the signals received by the SPI/I2S (i.e. SDI and WS) are read of the rising edge of CK."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CKPOL_A::B_0x0
    }
    #[doc = "the signals generated by the SPI/I2S (i.e. SDO and WS) are changed on the rising edge of CK and the signals received by the SPI/I2S (i.e. SDI and WS) are read of the falling edge of CK."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CKPOL_A::B_0x1
    }
}
#[doc = "Field `CKPOL` writer - serial audio clock polarity"]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL_A>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the signals generated by the SPI/I2S (i.e. SDO and WS) are changed on the falling edge of CK and the signals received by the SPI/I2S (i.e. SDI and WS) are read of the rising edge of CK."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0x0)
    }
    #[doc = "the signals generated by the SPI/I2S (i.e. SDO and WS) are changed on the rising edge of CK and the signals received by the SPI/I2S (i.e. SDI and WS) are read of the falling edge of CK."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0x1)
    }
}
#[doc = "fixed channel length in slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXCH_A {
    #[doc = "0: the channel length in Slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    B_0x0 = 0,
    #[doc = "1: the channel length in Slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    B_0x1 = 1,
}
impl From<FIXCH_A> for bool {
    #[inline(always)]
    fn from(variant: FIXCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXCH` reader - fixed channel length in slave"]
pub type FIXCH_R = crate::BitReader<FIXCH_A>;
impl FIXCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIXCH_A {
        match self.bits {
            false => FIXCH_A::B_0x0,
            true => FIXCH_A::B_0x1,
        }
    }
    #[doc = "the channel length in Slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FIXCH_A::B_0x0
    }
    #[doc = "the channel length in Slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FIXCH_A::B_0x1
    }
}
#[doc = "Field `FIXCH` writer - fixed channel length in slave"]
pub type FIXCH_W<'a, REG> = crate::BitWriter<'a, REG, FIXCH_A>;
impl<'a, REG> FIXCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the channel length in Slave mode is different from 16 or 32 bits (CHLEN not taken into account)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FIXCH_A::B_0x0)
    }
    #[doc = "the channel length in Slave mode is supposed to be 16 or 32 bits (according to CHLEN)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FIXCH_A::B_0x1)
    }
}
#[doc = "word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSINV_A {
    #[doc = "0: in I2S Philips standard, Left channel is transfered when WS is LOW, and right channel when WS is HIGH. In MSB or LSB justified mode, Left channel is transfered when WS is HIGH, and right channel when"]
    B_0x0 = 0,
    #[doc = "1: in I2S Philips standard, Left channel is transfered when WS is HIGH, and right channel when WS is LOW.In MSB or LSB justified mode, Left channel is transfered when WS is LOW, and right channel when"]
    B_0x1 = 1,
}
impl From<WSINV_A> for bool {
    #[inline(always)]
    fn from(variant: WSINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSINV` reader - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
pub type WSINV_R = crate::BitReader<WSINV_A>;
impl WSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WSINV_A {
        match self.bits {
            false => WSINV_A::B_0x0,
            true => WSINV_A::B_0x1,
        }
    }
    #[doc = "in I2S Philips standard, Left channel is transfered when WS is LOW, and right channel when WS is HIGH. In MSB or LSB justified mode, Left channel is transfered when WS is HIGH, and right channel when"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WSINV_A::B_0x0
    }
    #[doc = "in I2S Philips standard, Left channel is transfered when WS is HIGH, and right channel when WS is LOW.In MSB or LSB justified mode, Left channel is transfered when WS is LOW, and right channel when"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WSINV_A::B_0x1
    }
}
#[doc = "Field `WSINV` writer - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
pub type WSINV_W<'a, REG> = crate::BitWriter<'a, REG, WSINV_A>;
impl<'a, REG> WSINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "in I2S Philips standard, Left channel is transfered when WS is LOW, and right channel when WS is HIGH. In MSB or LSB justified mode, Left channel is transfered when WS is HIGH, and right channel when"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WSINV_A::B_0x0)
    }
    #[doc = "in I2S Philips standard, Left channel is transfered when WS is HIGH, and right channel when WS is LOW.In MSB or LSB justified mode, Left channel is transfered when WS is LOW, and right channel when"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WSINV_A::B_0x1)
    }
}
#[doc = "data format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATFMT_A {
    #[doc = "0: The data inside the SPI_RXDR or SPI_TXDR are right aligned"]
    B_0x0 = 0,
    #[doc = "1: The data inside the SPI_RXDR or SPI_TXDR are left aligned."]
    B_0x1 = 1,
}
impl From<DATFMT_A> for bool {
    #[inline(always)]
    fn from(variant: DATFMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATFMT` reader - data format"]
pub type DATFMT_R = crate::BitReader<DATFMT_A>;
impl DATFMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATFMT_A {
        match self.bits {
            false => DATFMT_A::B_0x0,
            true => DATFMT_A::B_0x1,
        }
    }
    #[doc = "The data inside the SPI_RXDR or SPI_TXDR are right aligned"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DATFMT_A::B_0x0
    }
    #[doc = "The data inside the SPI_RXDR or SPI_TXDR are left aligned."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DATFMT_A::B_0x1
    }
}
#[doc = "Field `DATFMT` writer - data format"]
pub type DATFMT_W<'a, REG> = crate::BitWriter<'a, REG, DATFMT_A>;
impl<'a, REG> DATFMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data inside the SPI_RXDR or SPI_TXDR are right aligned"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATFMT_A::B_0x0)
    }
    #[doc = "The data inside the SPI_RXDR or SPI_TXDR are left aligned."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATFMT_A::B_0x1)
    }
}
#[doc = "Field `I2SDIV` reader - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "odd factor for the prescaler Refer to for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD_A {
    #[doc = "0: Real divider value is = I2SDIV *2"]
    B_0x0 = 0,
    #[doc = "1: Real divider value is = (I2SDIV * 2) + 1"]
    B_0x1 = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - odd factor for the prescaler Refer to for details"]
pub type ODD_R = crate::BitReader<ODD_A>;
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::B_0x0,
            true => ODD_A::B_0x1,
        }
    }
    #[doc = "Real divider value is = I2SDIV *2"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ODD_A::B_0x0
    }
    #[doc = "Real divider value is = (I2SDIV * 2) + 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ODD_A::B_0x1
    }
}
#[doc = "Field `ODD` writer - odd factor for the prescaler Refer to for details"]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD_A>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real divider value is = I2SDIV *2"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_A::B_0x0)
    }
    #[doc = "Real divider value is = (I2SDIV * 2) + 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_A::B_0x1)
    }
}
#[doc = "master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE_A {
    #[doc = "0: Master clock output is disabled"]
    B_0x0 = 0,
    #[doc = "1: Master clock output is enabled"]
    B_0x1 = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - master clock output enable"]
pub type MCKOE_R = crate::BitReader<MCKOE_A>;
impl MCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::B_0x0,
            true => MCKOE_A::B_0x1,
        }
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MCKOE_A::B_0x0
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MCKOE_A::B_0x1
    }
}
#[doc = "Field `MCKOE` writer - master clock output enable"]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE_A>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE_A::B_0x0)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn I2SMOD(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode others, not used"]
    #[inline(always)]
    pub fn I2SCFG(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to"]
    #[inline(always)]
    pub fn I2SSTD(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn PCMSYNC(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - data length to be transferred"]
    #[inline(always)]
    pub fn DATLEN(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn CHLEN(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - serial audio clock polarity"]
    #[inline(always)]
    pub fn CKPOL(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - fixed channel length in slave"]
    #[inline(always)]
    pub fn FIXCH(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
    #[inline(always)]
    pub fn WSINV(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - data format"]
    #[inline(always)]
    pub fn DATFMT(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
    #[inline(always)]
    pub fn I2SDIV(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - odd factor for the prescaler Refer to for details"]
    #[inline(always)]
    pub fn ODD(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - master clock output enable"]
    #[inline(always)]
    pub fn MCKOE(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn I2SMOD(&mut self) -> I2SMOD_W<'_, I2SCFGR_SPEC> {
        I2SMOD_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode others, not used"]
    #[inline(always)]
    pub fn I2SCFG(&mut self) -> I2SCFG_W<'_, I2SCFGR_SPEC> {
        I2SCFG_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to"]
    #[inline(always)]
    pub fn I2SSTD(&mut self) -> I2SSTD_W<'_, I2SCFGR_SPEC> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn PCMSYNC(&mut self) -> PCMSYNC_W<'_, I2SCFGR_SPEC> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - data length to be transferred"]
    #[inline(always)]
    pub fn DATLEN(&mut self) -> DATLEN_W<'_, I2SCFGR_SPEC> {
        DATLEN_W::new(self, 8)
    }
    #[doc = "Bit 10 - channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn CHLEN(&mut self) -> CHLEN_W<'_, I2SCFGR_SPEC> {
        CHLEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - serial audio clock polarity"]
    #[inline(always)]
    pub fn CKPOL(&mut self) -> CKPOL_W<'_, I2SCFGR_SPEC> {
        CKPOL_W::new(self, 11)
    }
    #[doc = "Bit 12 - fixed channel length in slave"]
    #[inline(always)]
    pub fn FIXCH(&mut self) -> FIXCH_W<'_, I2SCFGR_SPEC> {
        FIXCH_W::new(self, 12)
    }
    #[doc = "Bit 13 - word select inversion This bit is used to invert the default polarity of WS signal. WS is LOW. In PCM mode the start of frame is indicated by a rising edge. WS is HIGH. In PCM mode the start of frame is indicated by a falling edge."]
    #[inline(always)]
    pub fn WSINV(&mut self) -> WSINV_W<'_, I2SCFGR_SPEC> {
        WSINV_W::new(self, 13)
    }
    #[doc = "Bit 14 - data format"]
    #[inline(always)]
    pub fn DATFMT(&mut self) -> DATFMT_W<'_, I2SCFGR_SPEC> {
        DATFMT_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler I2SDIV can take any values except the value 1, when ODD is also equal to 1. Refer to for details"]
    #[inline(always)]
    pub fn I2SDIV(&mut self) -> I2SDIV_W<'_, I2SCFGR_SPEC> {
        I2SDIV_W::new(self, 16)
    }
    #[doc = "Bit 24 - odd factor for the prescaler Refer to for details"]
    #[inline(always)]
    pub fn ODD(&mut self) -> ODD_W<'_, I2SCFGR_SPEC> {
        ODD_W::new(self, 24)
    }
    #[doc = "Bit 25 - master clock output enable"]
    #[inline(always)]
    pub fn MCKOE(&mut self) -> MCKOE_W<'_, I2SCFGR_SPEC> {
        MCKOE_W::new(self, 25)
    }
}
#[doc = "SPI/I2S configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2scfgr::R`](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {}
