#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    #[doc = "0: DAC channel1 disabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 enabled"]
    B_0x1 = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type EN1_R = crate::BitReader<EN1_A>;
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::B_0x0,
            true => EN1_A::B_0x1,
        }
    }
    #[doc = "DAC channel1 disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN1_A::B_0x0
    }
    #[doc = "DAC channel1 enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN1_A::B_0x1
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1_A>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN1_A::B_0x0)
    }
    #[doc = "DAC channel1 enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN1_A::B_0x1)
    }
}
#[doc = "DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_hclk clock cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1_A {
    #[doc = "0: DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_hclk clock cycle later to the DAC_DOR1 register"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_hclk clock cycles later to the DAC_DOR1 register"]
    B_0x1 = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_hclk clock cycle."]
pub type TEN1_R = crate::BitReader<TEN1_A>;
impl TEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::B_0x0,
            true => TEN1_A::B_0x1,
        }
    }
    #[doc = "DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_hclk clock cycle later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEN1_A::B_0x0
    }
    #[doc = "DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_hclk clock cycles later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEN1_A::B_0x1
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_hclk clock cycle."]
pub type TEN1_W<'a, REG> = crate::BitWriter<'a, REG, TEN1_A>;
impl<'a, REG> TEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_hclk clock cycle later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1_A::B_0x0)
    }
    #[doc = "DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_hclk clock cycles later to the DAC_DOR1 register"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEN1_A::B_0x1)
    }
}
#[doc = "DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: SWTRIG1"]
    B_0x0 = 0,
    #[doc = "1: dac_ch1_trg1"]
    B_0x1 = 1,
    #[doc = "2: dac_ch1_trg2"]
    B_0x2 = 2,
    #[doc = "15: dac_ch1_trg15"]
    B_0xF = 15,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for TSEL1_A {}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type TSEL1_R = crate::FieldReader<TSEL1_A>;
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::B_0x0),
            1 => Some(TSEL1_A::B_0x1),
            2 => Some(TSEL1_A::B_0x2),
            15 => Some(TSEL1_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSEL1_A::B_0x0
    }
    #[doc = "dac_ch1_trg1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSEL1_A::B_0x1
    }
    #[doc = "dac_ch1_trg2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TSEL1_A::B_0x2
    }
    #[doc = "dac_ch1_trg15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == TSEL1_A::B_0xF
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TSEL1_A>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SWTRIG1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::B_0x0)
    }
    #[doc = "dac_ch1_trg1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::B_0x1)
    }
    #[doc = "dac_ch1_trg2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::B_0x2)
    }
    #[doc = "dac_ch1_trg15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::B_0xF)
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1_A {
    #[doc = "0: wave generation disabled"]
    B_0x0 = 0,
    #[doc = "1: Noise wave generation enabled"]
    B_0x1 = 1,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAVE1_A {
    type Ux = u8;
}
impl crate::IsEnum for WAVE1_A {}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type WAVE1_R = crate::FieldReader<WAVE1_A>;
impl WAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAVE1_A> {
        match self.bits {
            0 => Some(WAVE1_A::B_0x0),
            1 => Some(WAVE1_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WAVE1_A::B_0x0
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WAVE1_A::B_0x1
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
pub type WAVE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAVE1_A>;
impl<'a, REG> WAVE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1_A::B_0x0)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE1_A::B_0x1)
    }
}
#[doc = "DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAMP1_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    B_0x0 = 0,
    #[doc = "1: Unmask bits\\[1:0\\] of LFSR/ triangle amplitude equal to 3"]
    B_0x1 = 1,
    #[doc = "2: Unmask bits\\[2:0\\] of LFSR/ triangle amplitude equal to 7"]
    B_0x2 = 2,
    #[doc = "3: Unmask bits\\[3:0\\] of LFSR/ triangle amplitude equal to 15"]
    B_0x3 = 3,
    #[doc = "4: Unmask bits\\[4:0\\] of LFSR/ triangle amplitude equal to 31"]
    B_0x4 = 4,
    #[doc = "5: Unmask bits\\[5:0\\] of LFSR/ triangle amplitude equal to 63"]
    B_0x5 = 5,
    #[doc = "6: Unmask bits\\[6:0\\] of LFSR/ triangle amplitude equal to 127"]
    B_0x6 = 6,
    #[doc = "7: Unmask bits\\[7:0\\] of LFSR/ triangle amplitude equal to 255"]
    B_0x7 = 7,
    #[doc = "8: Unmask bits\\[8:0\\] of LFSR/ triangle amplitude equal to 511"]
    B_0x8 = 8,
    #[doc = "9: Unmask bits\\[9:0\\] of LFSR/ triangle amplitude equal to 1023"]
    B_0x9 = 9,
    #[doc = "10: Unmask bits\\[10:0\\] of LFSR/ triangle amplitude equal to 2047"]
    B_0xA = 10,
}
impl From<MAMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAMP1_A {
    type Ux = u8;
}
impl crate::IsEnum for MAMP1_A {}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
pub type MAMP1_R = crate::FieldReader<MAMP1_A>;
impl MAMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAMP1_A> {
        match self.bits {
            0 => Some(MAMP1_A::B_0x0),
            1 => Some(MAMP1_A::B_0x1),
            2 => Some(MAMP1_A::B_0x2),
            3 => Some(MAMP1_A::B_0x3),
            4 => Some(MAMP1_A::B_0x4),
            5 => Some(MAMP1_A::B_0x5),
            6 => Some(MAMP1_A::B_0x6),
            7 => Some(MAMP1_A::B_0x7),
            8 => Some(MAMP1_A::B_0x8),
            9 => Some(MAMP1_A::B_0x9),
            10 => Some(MAMP1_A::B_0xA),
            _ => None,
        }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MAMP1_A::B_0x0
    }
    #[doc = "Unmask bits\\[1:0\\] of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MAMP1_A::B_0x1
    }
    #[doc = "Unmask bits\\[2:0\\] of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MAMP1_A::B_0x2
    }
    #[doc = "Unmask bits\\[3:0\\] of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MAMP1_A::B_0x3
    }
    #[doc = "Unmask bits\\[4:0\\] of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MAMP1_A::B_0x4
    }
    #[doc = "Unmask bits\\[5:0\\] of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MAMP1_A::B_0x5
    }
    #[doc = "Unmask bits\\[6:0\\] of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MAMP1_A::B_0x6
    }
    #[doc = "Unmask bits\\[7:0\\] of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == MAMP1_A::B_0x7
    }
    #[doc = "Unmask bits\\[8:0\\] of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == MAMP1_A::B_0x8
    }
    #[doc = "Unmask bits\\[9:0\\] of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == MAMP1_A::B_0x9
    }
    #[doc = "Unmask bits\\[10:0\\] of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == MAMP1_A::B_0xA
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
pub type MAMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAMP1_A>;
impl<'a, REG> MAMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x0)
    }
    #[doc = "Unmask bits\\[1:0\\] of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x1)
    }
    #[doc = "Unmask bits\\[2:0\\] of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x2)
    }
    #[doc = "Unmask bits\\[3:0\\] of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x3)
    }
    #[doc = "Unmask bits\\[4:0\\] of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x4)
    }
    #[doc = "Unmask bits\\[5:0\\] of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x5)
    }
    #[doc = "Unmask bits\\[6:0\\] of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x6)
    }
    #[doc = "Unmask bits\\[7:0\\] of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x7)
    }
    #[doc = "Unmask bits\\[8:0\\] of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x8)
    }
    #[doc = "Unmask bits\\[9:0\\] of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0x9)
    }
    #[doc = "Unmask bits\\[10:0\\] of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP1_A::B_0xA)
    }
}
#[doc = "DAC channel1 DMA enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC channel1 DMA mode disabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 DMA mode enabled"]
    B_0x1 = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type DMAEN1_R = crate::BitReader<DMAEN1_A>;
impl DMAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::B_0x0,
            true => DMAEN1_A::B_0x1,
        }
    }
    #[doc = "DAC channel1 DMA mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAEN1_A::B_0x0
    }
    #[doc = "DAC channel1 DMA mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAEN1_A::B_0x1
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable This bit is set and cleared by software."]
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN1_A>;
impl<'a, REG> DMAEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 DMA mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1_A::B_0x0)
    }
    #[doc = "DAC channel1 DMA mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN1_A::B_0x1)
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC channel1 DMA Underrun Interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 DMA Underrun Interrupt enabled"]
    B_0x1 = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1_A>;
impl DMAUDRIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::B_0x0,
            true => DMAUDRIE1_A::B_0x1,
        }
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAUDRIE1_A::B_0x0
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAUDRIE1_A::B_0x1
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDRIE1_A>;
impl<'a, REG> DMAUDRIE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1_A::B_0x0)
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE1_A::B_0x1)
    }
}
#[doc = "DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1 = 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN1_A {
    #[doc = "0: DAC channel1 in Normal operating mode"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 in calibration mode"]
    B_0x1 = 1,
}
impl From<CEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN1` reader - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1 = 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type CEN1_R = crate::BitReader<CEN1_A>;
impl CEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN1_A {
        match self.bits {
            false => CEN1_A::B_0x0,
            true => CEN1_A::B_0x1,
        }
    }
    #[doc = "DAC channel1 in Normal operating mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CEN1_A::B_0x0
    }
    #[doc = "DAC channel1 in calibration mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CEN1_A::B_0x1
    }
}
#[doc = "Field `CEN1` writer - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1 = 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
pub type CEN1_W<'a, REG> = crate::BitWriter<'a, REG, CEN1_A>;
impl<'a, REG> CEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel1 in Normal operating mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1_A::B_0x0)
    }
    #[doc = "DAC channel1 in calibration mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CEN1_A::B_0x1)
    }
}
#[doc = "DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN2_A {
    #[doc = "0: DAC channel2 disabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel2 enabled"]
    B_0x1 = 1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN2` reader - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation."]
pub type EN2_R = crate::BitReader<EN2_A>;
impl EN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::B_0x0,
            true => EN2_A::B_0x1,
        }
    }
    #[doc = "DAC channel2 disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN2_A::B_0x0
    }
    #[doc = "DAC channel2 enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN2_A::B_0x1
    }
}
#[doc = "Field `EN2` writer - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation."]
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG, EN2_A>;
impl<'a, REG> EN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel2 disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN2_A::B_0x0)
    }
    #[doc = "DAC channel2 enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN2_A::B_0x1)
    }
}
#[doc = "DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_hclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN2_A {
    #[doc = "0: DAC channel2 trigger disabled and data written into the DAC_DHR2 register are transferred one dac_hclk clock cycle later to the DAC_DOR2 register"]
    B_0x0 = 0,
    #[doc = "1: DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred three dac_hclk clock cycles later to the DAC_DOR2 register"]
    B_0x1 = 1,
}
impl From<TEN2_A> for bool {
    #[inline(always)]
    fn from(variant: TEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_hclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation."]
pub type TEN2_R = crate::BitReader<TEN2_A>;
impl TEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEN2_A {
        match self.bits {
            false => TEN2_A::B_0x0,
            true => TEN2_A::B_0x1,
        }
    }
    #[doc = "DAC channel2 trigger disabled and data written into the DAC_DHR2 register are transferred one dac_hclk clock cycle later to the DAC_DOR2 register"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEN2_A::B_0x0
    }
    #[doc = "DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred three dac_hclk clock cycles later to the DAC_DOR2 register"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEN2_A::B_0x1
    }
}
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_hclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation."]
pub type TEN2_W<'a, REG> = crate::BitWriter<'a, REG, TEN2_A>;
impl<'a, REG> TEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel2 trigger disabled and data written into the DAC_DHR2 register are transferred one dac_hclk clock cycle later to the DAC_DOR2 register"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEN2_A::B_0x0)
    }
    #[doc = "DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred three dac_hclk clock cycles later to the DAC_DOR2 register"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEN2_A::B_0x1)
    }
}
#[doc = "DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL2_A {
    #[doc = "0: SWTRIG2"]
    B_0x0 = 0,
    #[doc = "1: dac_ch2_trg1"]
    B_0x1 = 1,
    #[doc = "2: dac_ch2_trg2"]
    B_0x2 = 2,
    #[doc = "15: dac_ch2_trg15"]
    B_0xF = 15,
}
impl From<TSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL2_A {
    type Ux = u8;
}
impl crate::IsEnum for TSEL2_A {}
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation."]
pub type TSEL2_R = crate::FieldReader<TSEL2_A>;
impl TSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL2_A> {
        match self.bits {
            0 => Some(TSEL2_A::B_0x0),
            1 => Some(TSEL2_A::B_0x1),
            2 => Some(TSEL2_A::B_0x2),
            15 => Some(TSEL2_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "SWTRIG2"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSEL2_A::B_0x0
    }
    #[doc = "dac_ch2_trg1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSEL2_A::B_0x1
    }
    #[doc = "dac_ch2_trg2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TSEL2_A::B_0x2
    }
    #[doc = "dac_ch2_trg15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == TSEL2_A::B_0xF
    }
}
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation."]
pub type TSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TSEL2_A>;
impl<'a, REG> TSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SWTRIG2"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2_A::B_0x0)
    }
    #[doc = "dac_ch2_trg1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2_A::B_0x1)
    }
    #[doc = "dac_ch2_trg2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2_A::B_0x2)
    }
    #[doc = "dac_ch2_trg15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL2_A::B_0xF)
    }
}
#[doc = "DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE2_A {
    #[doc = "0: wave generation disabled"]
    B_0x0 = 0,
    #[doc = "1: Noise wave generation enabled"]
    B_0x1 = 1,
}
impl From<WAVE2_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAVE2_A {
    type Ux = u8;
}
impl crate::IsEnum for WAVE2_A {}
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation."]
pub type WAVE2_R = crate::FieldReader<WAVE2_A>;
impl WAVE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WAVE2_A> {
        match self.bits {
            0 => Some(WAVE2_A::B_0x0),
            1 => Some(WAVE2_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WAVE2_A::B_0x0
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WAVE2_A::B_0x1
    }
}
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation."]
pub type WAVE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAVE2_A>;
impl<'a, REG> WAVE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "wave generation disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE2_A::B_0x0)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE2_A::B_0x1)
    }
}
#[doc = "DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAMP2_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    B_0x0 = 0,
    #[doc = "1: Unmask bits\\[1:0\\] of LFSR/ triangle amplitude equal to 3"]
    B_0x1 = 1,
    #[doc = "2: Unmask bits\\[2:0\\] of LFSR/ triangle amplitude equal to 7"]
    B_0x2 = 2,
    #[doc = "3: Unmask bits\\[3:0\\] of LFSR/ triangle amplitude equal to 15"]
    B_0x3 = 3,
    #[doc = "4: Unmask bits\\[4:0\\] of LFSR/ triangle amplitude equal to 31"]
    B_0x4 = 4,
    #[doc = "5: Unmask bits\\[5:0\\] of LFSR/ triangle amplitude equal to 63"]
    B_0x5 = 5,
    #[doc = "6: Unmask bits\\[6:0\\] of LFSR/ triangle amplitude equal to 127"]
    B_0x6 = 6,
    #[doc = "7: Unmask bits\\[7:0\\] of LFSR/ triangle amplitude equal to 255"]
    B_0x7 = 7,
    #[doc = "8: Unmask bits\\[8:0\\] of LFSR/ triangle amplitude equal to 511"]
    B_0x8 = 8,
    #[doc = "9: Unmask bits\\[9:0\\] of LFSR/ triangle amplitude equal to 1023"]
    B_0x9 = 9,
    #[doc = "10: Unmask bits\\[10:0\\] of LFSR/ triangle amplitude equal to 2047"]
    B_0xA = 10,
}
impl From<MAMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAMP2_A {
    type Ux = u8;
}
impl crate::IsEnum for MAMP2_A {}
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation."]
pub type MAMP2_R = crate::FieldReader<MAMP2_A>;
impl MAMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAMP2_A> {
        match self.bits {
            0 => Some(MAMP2_A::B_0x0),
            1 => Some(MAMP2_A::B_0x1),
            2 => Some(MAMP2_A::B_0x2),
            3 => Some(MAMP2_A::B_0x3),
            4 => Some(MAMP2_A::B_0x4),
            5 => Some(MAMP2_A::B_0x5),
            6 => Some(MAMP2_A::B_0x6),
            7 => Some(MAMP2_A::B_0x7),
            8 => Some(MAMP2_A::B_0x8),
            9 => Some(MAMP2_A::B_0x9),
            10 => Some(MAMP2_A::B_0xA),
            _ => None,
        }
    }
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MAMP2_A::B_0x0
    }
    #[doc = "Unmask bits\\[1:0\\] of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MAMP2_A::B_0x1
    }
    #[doc = "Unmask bits\\[2:0\\] of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MAMP2_A::B_0x2
    }
    #[doc = "Unmask bits\\[3:0\\] of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MAMP2_A::B_0x3
    }
    #[doc = "Unmask bits\\[4:0\\] of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MAMP2_A::B_0x4
    }
    #[doc = "Unmask bits\\[5:0\\] of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MAMP2_A::B_0x5
    }
    #[doc = "Unmask bits\\[6:0\\] of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MAMP2_A::B_0x6
    }
    #[doc = "Unmask bits\\[7:0\\] of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == MAMP2_A::B_0x7
    }
    #[doc = "Unmask bits\\[8:0\\] of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == MAMP2_A::B_0x8
    }
    #[doc = "Unmask bits\\[9:0\\] of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == MAMP2_A::B_0x9
    }
    #[doc = "Unmask bits\\[10:0\\] of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == MAMP2_A::B_0xA
    }
}
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation."]
pub type MAMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAMP2_A>;
impl<'a, REG> MAMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x0)
    }
    #[doc = "Unmask bits\\[1:0\\] of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x1)
    }
    #[doc = "Unmask bits\\[2:0\\] of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x2)
    }
    #[doc = "Unmask bits\\[3:0\\] of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x3)
    }
    #[doc = "Unmask bits\\[4:0\\] of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x4)
    }
    #[doc = "Unmask bits\\[5:0\\] of LFSR/ triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x5)
    }
    #[doc = "Unmask bits\\[6:0\\] of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x6)
    }
    #[doc = "Unmask bits\\[7:0\\] of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x7)
    }
    #[doc = "Unmask bits\\[8:0\\] of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x8)
    }
    #[doc = "Unmask bits\\[9:0\\] of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0x9)
    }
    #[doc = "Unmask bits\\[10:0\\] of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(MAMP2_A::B_0xA)
    }
}
#[doc = "DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN2_A {
    #[doc = "0: DAC channel2 DMA mode disabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel2 DMA mode enabled"]
    B_0x1 = 1,
}
impl From<DMAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAEN2_R = crate::BitReader<DMAEN2_A>;
impl DMAEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN2_A {
        match self.bits {
            false => DMAEN2_A::B_0x0,
            true => DMAEN2_A::B_0x1,
        }
    }
    #[doc = "DAC channel2 DMA mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAEN2_A::B_0x0
    }
    #[doc = "DAC channel2 DMA mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAEN2_A::B_0x1
    }
}
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAEN2_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN2_A>;
impl<'a, REG> DMAEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel2 DMA mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN2_A::B_0x0)
    }
    #[doc = "DAC channel2 DMA mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN2_A::B_0x1)
    }
}
#[doc = "DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDRIE2_A {
    #[doc = "0: DAC channel2 DMA underrun interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel2 DMA underrun interrupt enabled"]
    B_0x1 = 1,
}
impl From<DMAUDRIE2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAUDRIE2_R = crate::BitReader<DMAUDRIE2_A>;
impl DMAUDRIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDRIE2_A {
        match self.bits {
            false => DMAUDRIE2_A::B_0x0,
            true => DMAUDRIE2_A::B_0x1,
        }
    }
    #[doc = "DAC channel2 DMA underrun interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAUDRIE2_A::B_0x0
    }
    #[doc = "DAC channel2 DMA underrun interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAUDRIE2_A::B_0x1
    }
}
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMAUDRIE2_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDRIE2_A>;
impl<'a, REG> DMAUDRIE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel2 DMA underrun interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE2_A::B_0x0)
    }
    #[doc = "DAC channel2 DMA underrun interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDRIE2_A::B_0x1)
    }
}
#[doc = "DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN2_A {
    #[doc = "0: DAC channel2 in Normal operating mode"]
    B_0x0 = 0,
    #[doc = "1: DAC channel2 in calibration mode"]
    B_0x1 = 1,
}
impl From<CEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN2` reader - DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type CEN2_R = crate::BitReader<CEN2_A>;
impl CEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN2_A {
        match self.bits {
            false => CEN2_A::B_0x0,
            true => CEN2_A::B_0x1,
        }
    }
    #[doc = "DAC channel2 in Normal operating mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CEN2_A::B_0x0
    }
    #[doc = "DAC channel2 in calibration mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CEN2_A::B_0x1
    }
}
#[doc = "Field `CEN2` writer - DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type CEN2_W<'a, REG> = crate::BitWriter<'a, REG, CEN2_A>;
impl<'a, REG> CEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel2 in Normal operating mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CEN2_A::B_0x0)
    }
    #[doc = "DAC channel2 in calibration mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CEN2_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn EN1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_hclk clock cycle."]
    #[inline(always)]
    pub fn TEN1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn TSEL1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn WAVE1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn MAMP1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn DMAEN1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn DMAUDRIE1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1 = 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn CEN1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn EN2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_hclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn TEN2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn TSEL2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn WAVE2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn MAMP2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMAEN2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMAUDRIE2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn CEN2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn EN1(&mut self) -> EN1_W<'_, CR_SPEC> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_hclk clock cycle."]
    #[inline(always)]
    pub fn TEN1(&mut self) -> TEN1_W<'_, CR_SPEC> {
        TEN1_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn TSEL1(&mut self) -> TSEL1_W<'_, CR_SPEC> {
        TSEL1_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn WAVE1(&mut self) -> WAVE1_W<'_, CR_SPEC> {
        WAVE1_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn MAMP1(&mut self) -> MAMP1_W<'_, CR_SPEC> {
        MAMP1_W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn DMAEN1(&mut self) -> DMAEN1_W<'_, CR_SPEC> {
        DMAEN1_W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn DMAUDRIE1(&mut self) -> DMAUDRIE1_W<'_, CR_SPEC> {
        DMAUDRIE1_W::new(self, 13)
    }
    #[doc = "Bit 14 - DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1 = 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn CEN1(&mut self) -> CEN1_W<'_, CR_SPEC> {
        CEN1_W::new(self, 14)
    }
    #[doc = "Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn EN2(&mut self) -> EN2_W<'_, CR_SPEC> {
        EN2_W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_hclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn TEN2(&mut self) -> TEN2_W<'_, CR_SPEC> {
        TEN2_W::new(self, 17)
    }
    #[doc = "Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn TSEL2(&mut self) -> TSEL2_W<'_, CR_SPEC> {
        TSEL2_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn WAVE2(&mut self) -> WAVE2_W<'_, CR_SPEC> {
        WAVE2_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. greater than or equal 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn MAMP2(&mut self) -> MAMP2_W<'_, CR_SPEC> {
        MAMP2_W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMAEN2(&mut self) -> DMAEN2_W<'_, CR_SPEC> {
        DMAEN2_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMAUDRIE2(&mut self) -> DMAUDRIE2_W<'_, CR_SPEC> {
        DMAUDRIE2_W::new(self, 29)
    }
    #[doc = "Bit 30 - DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn CEN2(&mut self) -> CEN2_W<'_, CR_SPEC> {
        CEN2_W::new(self, 30)
    }
}
#[doc = "DAC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {}
