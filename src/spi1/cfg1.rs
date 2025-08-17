#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\] bits are reserved and must be kept at reset state. DSIZE\\[4:3\\] bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSIZE_A {
    #[doc = "0: not used"]
    B_0x0 = 0,
    #[doc = "1: not used"]
    B_0x1 = 1,
    #[doc = "2: not used"]
    B_0x2 = 2,
    #[doc = "3: 4-bits"]
    B_0x3 = 3,
    #[doc = "4: 5-bits"]
    B_0x4 = 4,
    #[doc = "5: 6-bits"]
    B_0x5 = 5,
    #[doc = "6: 7-bits"]
    B_0x6 = 6,
    #[doc = "7: 8-bits"]
    B_0x7 = 7,
    #[doc = "29: 30-bits"]
    B_0x1D = 29,
    #[doc = "30: 31-bits"]
    B_0x1E = 30,
    #[doc = "31: 32-bits"]
    B_0x1F = 31,
}
impl From<DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for DSIZE_A {}
#[doc = "Field `DSIZE` reader - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\] bits are reserved and must be kept at reset state. DSIZE\\[4:3\\] bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
pub type DSIZE_R = crate::FieldReader<DSIZE_A>;
impl DSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DSIZE_A> {
        match self.bits {
            0 => Some(DSIZE_A::B_0x0),
            1 => Some(DSIZE_A::B_0x1),
            2 => Some(DSIZE_A::B_0x2),
            3 => Some(DSIZE_A::B_0x3),
            4 => Some(DSIZE_A::B_0x4),
            5 => Some(DSIZE_A::B_0x5),
            6 => Some(DSIZE_A::B_0x6),
            7 => Some(DSIZE_A::B_0x7),
            29 => Some(DSIZE_A::B_0x1D),
            30 => Some(DSIZE_A::B_0x1E),
            31 => Some(DSIZE_A::B_0x1F),
            _ => None,
        }
    }
    #[doc = "not used"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DSIZE_A::B_0x0
    }
    #[doc = "not used"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DSIZE_A::B_0x1
    }
    #[doc = "not used"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DSIZE_A::B_0x2
    }
    #[doc = "4-bits"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == DSIZE_A::B_0x3
    }
    #[doc = "5-bits"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == DSIZE_A::B_0x4
    }
    #[doc = "6-bits"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == DSIZE_A::B_0x5
    }
    #[doc = "7-bits"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == DSIZE_A::B_0x6
    }
    #[doc = "8-bits"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == DSIZE_A::B_0x7
    }
    #[doc = "30-bits"]
    #[inline(always)]
    pub fn is_B_0x1D(&self) -> bool {
        *self == DSIZE_A::B_0x1D
    }
    #[doc = "31-bits"]
    #[inline(always)]
    pub fn is_B_0x1E(&self) -> bool {
        *self == DSIZE_A::B_0x1E
    }
    #[doc = "32-bits"]
    #[inline(always)]
    pub fn is_B_0x1F(&self) -> bool {
        *self == DSIZE_A::B_0x1F
    }
}
#[doc = "Field `DSIZE` writer - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\] bits are reserved and must be kept at reset state. DSIZE\\[4:3\\] bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DSIZE_A>;
impl<'a, REG> DSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not used"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x0)
    }
    #[doc = "not used"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x1)
    }
    #[doc = "not used"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x2)
    }
    #[doc = "4-bits"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x3)
    }
    #[doc = "5-bits"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x4)
    }
    #[doc = "6-bits"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x5)
    }
    #[doc = "7-bits"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x6)
    }
    #[doc = "8-bits"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x7)
    }
    #[doc = "30-bits"]
    #[inline(always)]
    pub fn B_0x1D(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x1D)
    }
    #[doc = "31-bits"]
    #[inline(always)]
    pub fn B_0x1E(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x1E)
    }
    #[doc = "32-bits"]
    #[inline(always)]
    pub fn B_0x1F(self) -> &'a mut crate::W<REG> {
        self.variant(DSIZE_A::B_0x1F)
    }
}
#[doc = "FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTHLV_A {
    #[doc = "0: 1-data"]
    B_0x0 = 0,
    #[doc = "1: 2-data"]
    B_0x1 = 1,
    #[doc = "2: 3-data"]
    B_0x2 = 2,
    #[doc = "3: 4-data"]
    B_0x3 = 3,
    #[doc = "4: 5-data"]
    B_0x4 = 4,
    #[doc = "5: 6-data"]
    B_0x5 = 5,
    #[doc = "6: 7-data"]
    B_0x6 = 6,
    #[doc = "7: 8-data"]
    B_0x7 = 7,
    #[doc = "8: 9-data"]
    B_0x8 = 8,
    #[doc = "9: 10-data"]
    B_0x9 = 9,
    #[doc = "10: 11-data"]
    B_0xA = 10,
    #[doc = "11: 12-data"]
    B_0xB = 11,
    #[doc = "12: 13-data"]
    B_0xC = 12,
    #[doc = "13: 14-data"]
    B_0xD = 13,
    #[doc = "14: 15-data"]
    B_0xE = 14,
    #[doc = "15: 16-data"]
    B_0xF = 15,
}
impl From<FTHLV_A> for u8 {
    #[inline(always)]
    fn from(variant: FTHLV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTHLV_A {
    type Ux = u8;
}
impl crate::IsEnum for FTHLV_A {}
#[doc = "Field `FTHLV` reader - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space."]
pub type FTHLV_R = crate::FieldReader<FTHLV_A>;
impl FTHLV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTHLV_A {
        match self.bits {
            0 => FTHLV_A::B_0x0,
            1 => FTHLV_A::B_0x1,
            2 => FTHLV_A::B_0x2,
            3 => FTHLV_A::B_0x3,
            4 => FTHLV_A::B_0x4,
            5 => FTHLV_A::B_0x5,
            6 => FTHLV_A::B_0x6,
            7 => FTHLV_A::B_0x7,
            8 => FTHLV_A::B_0x8,
            9 => FTHLV_A::B_0x9,
            10 => FTHLV_A::B_0xA,
            11 => FTHLV_A::B_0xB,
            12 => FTHLV_A::B_0xC,
            13 => FTHLV_A::B_0xD,
            14 => FTHLV_A::B_0xE,
            15 => FTHLV_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "1-data"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FTHLV_A::B_0x0
    }
    #[doc = "2-data"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FTHLV_A::B_0x1
    }
    #[doc = "3-data"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == FTHLV_A::B_0x2
    }
    #[doc = "4-data"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == FTHLV_A::B_0x3
    }
    #[doc = "5-data"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == FTHLV_A::B_0x4
    }
    #[doc = "6-data"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == FTHLV_A::B_0x5
    }
    #[doc = "7-data"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == FTHLV_A::B_0x6
    }
    #[doc = "8-data"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == FTHLV_A::B_0x7
    }
    #[doc = "9-data"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == FTHLV_A::B_0x8
    }
    #[doc = "10-data"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == FTHLV_A::B_0x9
    }
    #[doc = "11-data"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == FTHLV_A::B_0xA
    }
    #[doc = "12-data"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == FTHLV_A::B_0xB
    }
    #[doc = "13-data"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == FTHLV_A::B_0xC
    }
    #[doc = "14-data"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == FTHLV_A::B_0xD
    }
    #[doc = "15-data"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == FTHLV_A::B_0xE
    }
    #[doc = "16-data"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == FTHLV_A::B_0xF
    }
}
#[doc = "Field `FTHLV` writer - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space."]
pub type FTHLV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FTHLV_A, crate::Safe>;
impl<'a, REG> FTHLV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-data"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x0)
    }
    #[doc = "2-data"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x1)
    }
    #[doc = "3-data"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x2)
    }
    #[doc = "4-data"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x3)
    }
    #[doc = "5-data"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x4)
    }
    #[doc = "6-data"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x5)
    }
    #[doc = "7-data"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x6)
    }
    #[doc = "8-data"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x7)
    }
    #[doc = "9-data"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x8)
    }
    #[doc = "10-data"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0x9)
    }
    #[doc = "11-data"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0xA)
    }
    #[doc = "12-data"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0xB)
    }
    #[doc = "13-data"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0xC)
    }
    #[doc = "14-data"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0xD)
    }
    #[doc = "15-data"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0xE)
    }
    #[doc = "16-data"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(FTHLV_A::B_0xF)
    }
}
#[doc = "behavior of slave transmitter at underrun condition For more details see underrun condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRCFG_A {
    #[doc = "0: slave sends a constant pattern defined by the user at the SPI_UDRDR register"]
    B_0x0 = 0,
    #[doc = "1: Slave repeats lastly received data from master. When slave is configured at transmit only mode (COMM\\[1:0\\]=01), all zeros pattern is repeated."]
    B_0x1 = 1,
}
impl From<UDRCFG_A> for bool {
    #[inline(always)]
    fn from(variant: UDRCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDRCFG` reader - behavior of slave transmitter at underrun condition For more details see underrun condition."]
pub type UDRCFG_R = crate::BitReader<UDRCFG_A>;
impl UDRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDRCFG_A {
        match self.bits {
            false => UDRCFG_A::B_0x0,
            true => UDRCFG_A::B_0x1,
        }
    }
    #[doc = "slave sends a constant pattern defined by the user at the SPI_UDRDR register"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UDRCFG_A::B_0x0
    }
    #[doc = "Slave repeats lastly received data from master. When slave is configured at transmit only mode (COMM\\[1:0\\]=01), all zeros pattern is repeated."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UDRCFG_A::B_0x1
    }
}
#[doc = "Field `UDRCFG` writer - behavior of slave transmitter at underrun condition For more details see underrun condition."]
pub type UDRCFG_W<'a, REG> = crate::BitWriter<'a, REG, UDRCFG_A>;
impl<'a, REG> UDRCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "slave sends a constant pattern defined by the user at the SPI_UDRDR register"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG_A::B_0x0)
    }
    #[doc = "Slave repeats lastly received data from master. When slave is configured at transmit only mode (COMM\\[1:0\\]=01), all zeros pattern is repeated."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UDRCFG_A::B_0x1)
    }
}
#[doc = "Rx DMA stream enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx-DMA disabled"]
    B_0x0 = 0,
    #[doc = "1: Rx-DMA enabled"]
    B_0x1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::B_0x0,
            true => RXDMAEN_A::B_0x1,
        }
    }
    #[doc = "Rx-DMA disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXDMAEN_A::B_0x0
    }
    #[doc = "Rx-DMA enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXDMAEN_A::B_0x1
    }
}
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN_A>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx-DMA disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0x0)
    }
    #[doc = "Rx-DMA enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0x1)
    }
}
#[doc = "Tx DMA stream enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx DMA disabled"]
    B_0x0 = 0,
    #[doc = "1: Tx DMA enabled"]
    B_0x1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::B_0x0,
            true => TXDMAEN_A::B_0x1,
        }
    }
    #[doc = "Tx DMA disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXDMAEN_A::B_0x0
    }
    #[doc = "Tx DMA enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXDMAEN_A::B_0x1
    }
}
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN_A>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx DMA disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0x0)
    }
    #[doc = "Tx DMA enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0x1)
    }
}
#[doc = "length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRCSIZE_A {
    #[doc = "3: 4-bits"]
    B_0x3 = 3,
    #[doc = "4: 5-bits"]
    B_0x4 = 4,
    #[doc = "5: 6-bits"]
    B_0x5 = 5,
    #[doc = "6: 7-bits"]
    B_0x6 = 6,
    #[doc = "7: 8-bits"]
    B_0x7 = 7,
    #[doc = "29: 30-bits"]
    B_0x1D = 29,
    #[doc = "30: 31-bits"]
    B_0x1E = 30,
    #[doc = "31: 32-bits"]
    B_0x1F = 31,
}
impl From<CRCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRCSIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for CRCSIZE_A {}
#[doc = "Field `CRCSIZE` reader - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
pub type CRCSIZE_R = crate::FieldReader<CRCSIZE_A>;
impl CRCSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRCSIZE_A> {
        match self.bits {
            3 => Some(CRCSIZE_A::B_0x3),
            4 => Some(CRCSIZE_A::B_0x4),
            5 => Some(CRCSIZE_A::B_0x5),
            6 => Some(CRCSIZE_A::B_0x6),
            7 => Some(CRCSIZE_A::B_0x7),
            29 => Some(CRCSIZE_A::B_0x1D),
            30 => Some(CRCSIZE_A::B_0x1E),
            31 => Some(CRCSIZE_A::B_0x1F),
            _ => None,
        }
    }
    #[doc = "4-bits"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CRCSIZE_A::B_0x3
    }
    #[doc = "5-bits"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == CRCSIZE_A::B_0x4
    }
    #[doc = "6-bits"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == CRCSIZE_A::B_0x5
    }
    #[doc = "7-bits"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == CRCSIZE_A::B_0x6
    }
    #[doc = "8-bits"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == CRCSIZE_A::B_0x7
    }
    #[doc = "30-bits"]
    #[inline(always)]
    pub fn is_B_0x1D(&self) -> bool {
        *self == CRCSIZE_A::B_0x1D
    }
    #[doc = "31-bits"]
    #[inline(always)]
    pub fn is_B_0x1E(&self) -> bool {
        *self == CRCSIZE_A::B_0x1E
    }
    #[doc = "32-bits"]
    #[inline(always)]
    pub fn is_B_0x1F(&self) -> bool {
        *self == CRCSIZE_A::B_0x1F
    }
}
#[doc = "Field `CRCSIZE` writer - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
pub type CRCSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CRCSIZE_A>;
impl<'a, REG> CRCSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bits"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x3)
    }
    #[doc = "5-bits"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x4)
    }
    #[doc = "6-bits"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x5)
    }
    #[doc = "7-bits"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x6)
    }
    #[doc = "8-bits"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x7)
    }
    #[doc = "30-bits"]
    #[inline(always)]
    pub fn B_0x1D(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x1D)
    }
    #[doc = "31-bits"]
    #[inline(always)]
    pub fn B_0x1E(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x1E)
    }
    #[doc = "32-bits"]
    #[inline(always)]
    pub fn B_0x1F(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSIZE_A::B_0x1F)
    }
}
#[doc = "hardware CRC computation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    B_0x0 = 0,
    #[doc = "1: CRC calculation enabled"]
    B_0x1 = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - hardware CRC computation enable"]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::B_0x0,
            true => CRCEN_A::B_0x1,
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRCEN_A::B_0x0
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRCEN_A::B_0x1
    }
}
#[doc = "Field `CRCEN` writer - hardware CRC computation enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN_A>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::B_0x0)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::B_0x1)
    }
}
#[doc = "master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBR_A {
    #[doc = "0: SPI master clock/2"]
    B_0x0 = 0,
    #[doc = "1: SPI master clock/4"]
    B_0x1 = 1,
    #[doc = "2: SPI master clock/8"]
    B_0x2 = 2,
    #[doc = "3: SPI master clock/16"]
    B_0x3 = 3,
    #[doc = "4: SPI master clock/32"]
    B_0x4 = 4,
    #[doc = "5: SPI master clock/64"]
    B_0x5 = 5,
    #[doc = "6: SPI master clock/128"]
    B_0x6 = 6,
    #[doc = "7: SPI master clock/256"]
    B_0x7 = 7,
}
impl From<MBR_A> for u8 {
    #[inline(always)]
    fn from(variant: MBR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBR_A {
    type Ux = u8;
}
impl crate::IsEnum for MBR_A {}
#[doc = "Field `MBR` reader - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
pub type MBR_R = crate::FieldReader<MBR_A>;
impl MBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBR_A {
        match self.bits {
            0 => MBR_A::B_0x0,
            1 => MBR_A::B_0x1,
            2 => MBR_A::B_0x2,
            3 => MBR_A::B_0x3,
            4 => MBR_A::B_0x4,
            5 => MBR_A::B_0x5,
            6 => MBR_A::B_0x6,
            7 => MBR_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI master clock/2"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MBR_A::B_0x0
    }
    #[doc = "SPI master clock/4"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MBR_A::B_0x1
    }
    #[doc = "SPI master clock/8"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MBR_A::B_0x2
    }
    #[doc = "SPI master clock/16"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MBR_A::B_0x3
    }
    #[doc = "SPI master clock/32"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MBR_A::B_0x4
    }
    #[doc = "SPI master clock/64"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MBR_A::B_0x5
    }
    #[doc = "SPI master clock/128"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MBR_A::B_0x6
    }
    #[doc = "SPI master clock/256"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == MBR_A::B_0x7
    }
}
#[doc = "Field `MBR` writer - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
pub type MBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MBR_A, crate::Safe>;
impl<'a, REG> MBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI master clock/2"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x0)
    }
    #[doc = "SPI master clock/4"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x1)
    }
    #[doc = "SPI master clock/8"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x2)
    }
    #[doc = "SPI master clock/16"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x3)
    }
    #[doc = "SPI master clock/32"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x4)
    }
    #[doc = "SPI master clock/64"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x5)
    }
    #[doc = "SPI master clock/128"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x6)
    }
    #[doc = "SPI master clock/256"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MBR_A::B_0x7)
    }
}
#[doc = "bypass of the prescaler at master baud rate clock generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPASS_A {
    #[doc = "0: bypass is disabled"]
    B_0x0 = 0,
    #[doc = "1: bypass is enabled"]
    B_0x1 = 1,
}
impl From<BPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPASS` reader - bypass of the prescaler at master baud rate clock generator"]
pub type BPASS_R = crate::BitReader<BPASS_A>;
impl BPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BPASS_A {
        match self.bits {
            false => BPASS_A::B_0x0,
            true => BPASS_A::B_0x1,
        }
    }
    #[doc = "bypass is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BPASS_A::B_0x0
    }
    #[doc = "bypass is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BPASS_A::B_0x1
    }
}
#[doc = "Field `BPASS` writer - bypass of the prescaler at master baud rate clock generator"]
pub type BPASS_W<'a, REG> = crate::BitWriter<'a, REG, BPASS_A>;
impl<'a, REG> BPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bypass is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BPASS_A::B_0x0)
    }
    #[doc = "bypass is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BPASS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:4 - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\] bits are reserved and must be kept at reset state. DSIZE\\[4:3\\] bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
    #[inline(always)]
    pub fn DSIZE(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space."]
    #[inline(always)]
    pub fn FTHLV(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - behavior of slave transmitter at underrun condition For more details see underrun condition."]
    #[inline(always)]
    pub fn UDRCFG(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn RXDMAEN(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn TXDMAEN(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
    #[inline(always)]
    pub fn CRCSIZE(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - hardware CRC computation enable"]
    #[inline(always)]
    pub fn CRCEN(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
    #[inline(always)]
    pub fn MBR(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    pub fn BPASS(&self) -> BPASS_R {
        BPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - number of bits in at single SPI data frame ..... Note: Maximum data size can be limited up to 16-bits at some instances. At instances with limited set of features, DSIZE2:0\\] bits are reserved and must be kept at reset state. DSIZE\\[4:3\\] bits then control next settings of data size: 00xxx: 8-bits 01xxx: 16-bits 10xxx: 24-bits 11xxx: 32-bits."]
    #[inline(always)]
    pub fn DSIZE(&mut self) -> DSIZE_W<'_, CFG1_SPEC> {
        DSIZE_W::new(self, 0)
    }
    #[doc = "Bits 5:8 - FIFO threshold level Defines number of data frames at single data packet. Size of the packet should not exceed 1/2 of FIFO space."]
    #[inline(always)]
    pub fn FTHLV(&mut self) -> FTHLV_W<'_, CFG1_SPEC> {
        FTHLV_W::new(self, 5)
    }
    #[doc = "Bit 9 - behavior of slave transmitter at underrun condition For more details see underrun condition."]
    #[inline(always)]
    pub fn UDRCFG(&mut self) -> UDRCFG_W<'_, CFG1_SPEC> {
        UDRCFG_W::new(self, 9)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn RXDMAEN(&mut self) -> RXDMAEN_W<'_, CFG1_SPEC> {
        RXDMAEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn TXDMAEN(&mut self) -> TXDMAEN_W<'_, CFG1_SPEC> {
        TXDMAEN_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - length of CRC frame to be transacted and compared Most significant bits are taken into account from polynomial calculation when CRC result is transacted or compared. The length of the polynomial is not affected by this setting. ..... The value must be set equal or multiply of data size (DSIZE\\[4:0\\]). Its maximum size corresponds to DSIZE maximum at the instance. Note: The most significant bit at CRCSIZE bit field is reserved at the peripheral instances where data size is limited to 16-bit."]
    #[inline(always)]
    pub fn CRCSIZE(&mut self) -> CRCSIZE_W<'_, CFG1_SPEC> {
        CRCSIZE_W::new(self, 16)
    }
    #[doc = "Bit 22 - hardware CRC computation enable"]
    #[inline(always)]
    pub fn CRCEN(&mut self) -> CRCEN_W<'_, CFG1_SPEC> {
        CRCEN_W::new(self, 22)
    }
    #[doc = "Bits 28:30 - master baud rate prescaler setting Note: MBR setting is considered at slave working at TI mode, too (see mode)."]
    #[inline(always)]
    pub fn MBR(&mut self) -> MBR_W<'_, CFG1_SPEC> {
        MBR_W::new(self, 28)
    }
    #[doc = "Bit 31 - bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    pub fn BPASS(&mut self) -> BPASS_W<'_, CFG1_SPEC> {
        BPASS_W::new(self, 31)
    }
}
#[doc = "SPI/I2S configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0007_0007;
}
