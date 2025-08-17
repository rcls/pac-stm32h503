#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE_A {
    #[doc = "0: Regular Oversampling disabled"]
    B_0x0 = 0,
    #[doc = "1: Regular Oversampling enabled"]
    B_0x1 = 1,
}
impl From<ROVSE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSE` reader - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type ROVSE_R = crate::BitReader<ROVSE_A>;
impl ROVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVSE_A {
        match self.bits {
            false => ROVSE_A::B_0x0,
            true => ROVSE_A::B_0x1,
        }
    }
    #[doc = "Regular Oversampling disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ROVSE_A::B_0x0
    }
    #[doc = "Regular Oversampling enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ROVSE_A::B_0x1
    }
}
#[doc = "Field `ROVSE` writer - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE_A>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular Oversampling disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE_A::B_0x0)
    }
    #[doc = "Regular Oversampling enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE_A::B_0x1)
    }
}
#[doc = "Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE_A {
    #[doc = "0: Injected Oversampling disabled"]
    B_0x0 = 0,
    #[doc = "1: Injected Oversampling enabled"]
    B_0x1 = 1,
}
impl From<JOVSE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVSE` reader - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type JOVSE_R = crate::BitReader<JOVSE_A>;
impl JOVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JOVSE_A {
        match self.bits {
            false => JOVSE_A::B_0x0,
            true => JOVSE_A::B_0x1,
        }
    }
    #[doc = "Injected Oversampling disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JOVSE_A::B_0x0
    }
    #[doc = "Injected Oversampling enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JOVSE_A::B_0x1
    }
}
#[doc = "Field `JOVSE` writer - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE_A>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected Oversampling disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE_A::B_0x0)
    }
    #[doc = "Injected Oversampling enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE_A::B_0x1)
    }
}
#[doc = "Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    B_0x0 = 0,
    #[doc = "1: 4x"]
    B_0x1 = 1,
    #[doc = "2: 8x"]
    B_0x2 = 2,
    #[doc = "3: 16x"]
    B_0x3 = 3,
    #[doc = "4: 32x"]
    B_0x4 = 4,
    #[doc = "5: 64x"]
    B_0x5 = 5,
    #[doc = "6: 128x"]
    B_0x6 = 6,
    #[doc = "7: 256x"]
    B_0x7 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSR_A {
    type Ux = u8;
}
impl crate::IsEnum for OVSR_A {}
#[doc = "Field `OVSR` reader - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSR_R = crate::FieldReader<OVSR_A>;
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::B_0x0,
            1 => OVSR_A::B_0x1,
            2 => OVSR_A::B_0x2,
            3 => OVSR_A::B_0x3,
            4 => OVSR_A::B_0x4,
            5 => OVSR_A::B_0x5,
            6 => OVSR_A::B_0x6,
            7 => OVSR_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVSR_A::B_0x0
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVSR_A::B_0x1
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OVSR_A::B_0x2
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OVSR_A::B_0x3
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == OVSR_A::B_0x4
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == OVSR_A::B_0x5
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == OVSR_A::B_0x6
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == OVSR_A::B_0x7
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OVSR_A, crate::Safe>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x0)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x1)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x2)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x3)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x4)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x5)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x6)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0x7)
    }
}
#[doc = "Oversampling shift This bitfield is set and cleared by software to define the right shifting applied to the raw oversampling result. Other codes reserved Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No shift"]
    B_0x0 = 0,
    #[doc = "1: Shift 1-bit"]
    B_0x1 = 1,
    #[doc = "2: Shift 2-bits"]
    B_0x2 = 2,
    #[doc = "3: Shift 3-bits"]
    B_0x3 = 3,
    #[doc = "4: Shift 4-bits"]
    B_0x4 = 4,
    #[doc = "5: Shift 5-bits"]
    B_0x5 = 5,
    #[doc = "6: Shift 6-bits"]
    B_0x6 = 6,
    #[doc = "7: Shift 7-bits"]
    B_0x7 = 7,
    #[doc = "8: Shift 8-bits"]
    B_0x8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSS_A {
    type Ux = u8;
}
impl crate::IsEnum for OVSS_A {}
#[doc = "Field `OVSS` reader - Oversampling shift This bitfield is set and cleared by software to define the right shifting applied to the raw oversampling result. Other codes reserved Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSS_R = crate::FieldReader<OVSS_A>;
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::B_0x0),
            1 => Some(OVSS_A::B_0x1),
            2 => Some(OVSS_A::B_0x2),
            3 => Some(OVSS_A::B_0x3),
            4 => Some(OVSS_A::B_0x4),
            5 => Some(OVSS_A::B_0x5),
            6 => Some(OVSS_A::B_0x6),
            7 => Some(OVSS_A::B_0x7),
            8 => Some(OVSS_A::B_0x8),
            _ => None,
        }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVSS_A::B_0x0
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVSS_A::B_0x1
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OVSS_A::B_0x2
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OVSS_A::B_0x3
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == OVSS_A::B_0x4
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == OVSS_A::B_0x5
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == OVSS_A::B_0x6
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == OVSS_A::B_0x7
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == OVSS_A::B_0x8
    }
}
#[doc = "Field `OVSS` writer - Oversampling shift This bitfield is set and cleared by software to define the right shifting applied to the raw oversampling result. Other codes reserved Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS_A>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No shift"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x0)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x1)
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x2)
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x3)
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x4)
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x5)
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x6)
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x7)
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0x8)
    }
}
#[doc = "Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS_A {
    #[doc = "0: All oversampled conversions for a channel are done consecutively following a trigger"]
    B_0x0 = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    B_0x1 = 1,
}
impl From<TROVS_A> for bool {
    #[inline(always)]
    fn from(variant: TROVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type TROVS_R = crate::BitReader<TROVS_A>;
impl TROVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TROVS_A {
        match self.bits {
            false => TROVS_A::B_0x0,
            true => TROVS_A::B_0x1,
        }
    }
    #[doc = "All oversampled conversions for a channel are done consecutively following a trigger"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TROVS_A::B_0x0
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TROVS_A::B_0x1
    }
}
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS_A>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are done consecutively following a trigger"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS_A::B_0x0)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS_A::B_0x1)
    }
}
#[doc = "Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM_A {
    #[doc = "0: Continued mode: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    B_0x0 = 0,
    #[doc = "1: Resumed mode: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    B_0x1 = 1,
}
impl From<ROVSM_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ROVSM_R = crate::BitReader<ROVSM_A>;
impl ROVSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM_A {
        match self.bits {
            false => ROVSM_A::B_0x0,
            true => ROVSM_A::B_0x1,
        }
    }
    #[doc = "Continued mode: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ROVSM_A::B_0x0
    }
    #[doc = "Resumed mode: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ROVSM_A::B_0x1
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM_A>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continued mode: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM_A::B_0x0)
    }
    #[doc = "Resumed mode: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM_A::B_0x1)
    }
}
#[doc = "Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG_A {
    #[doc = "0: Software trigger starts the conversion for sampling time control trigger mode"]
    B_0x0 = 0,
    #[doc = "1: Software trigger starts the sampling for sampling time control trigger mode"]
    B_0x1 = 1,
}
impl From<SWTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SWTRIG_R = crate::BitReader<SWTRIG_A>;
impl SWTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWTRIG_A {
        match self.bits {
            false => SWTRIG_A::B_0x0,
            true => SWTRIG_A::B_0x1,
        }
    }
    #[doc = "Software trigger starts the conversion for sampling time control trigger mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWTRIG_A::B_0x0
    }
    #[doc = "Software trigger starts the sampling for sampling time control trigger mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWTRIG_A::B_0x1
    }
}
#[doc = "Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG_A>;
impl<'a, REG> SWTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger starts the conversion for sampling time control trigger mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG_A::B_0x0)
    }
    #[doc = "Software trigger starts the sampling for sampling time control trigger mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG_A::B_0x1)
    }
}
#[doc = "Bulb sampling mode This bit is set and cleared by software to enable the bulb sampling mode. SAMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB_A {
    #[doc = "0: Bulb sampling mode disabled"]
    B_0x0 = 0,
    #[doc = "1: Bulb sampling mode enabled. The sampling period starts just after the previous end of conversion."]
    B_0x1 = 1,
}
impl From<BULB_A> for bool {
    #[inline(always)]
    fn from(variant: BULB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BULB` reader - Bulb sampling mode This bit is set and cleared by software to enable the bulb sampling mode. SAMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type BULB_R = crate::BitReader<BULB_A>;
impl BULB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BULB_A {
        match self.bits {
            false => BULB_A::B_0x0,
            true => BULB_A::B_0x1,
        }
    }
    #[doc = "Bulb sampling mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BULB_A::B_0x0
    }
    #[doc = "Bulb sampling mode enabled. The sampling period starts just after the previous end of conversion."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BULB_A::B_0x1
    }
}
#[doc = "Field `BULB` writer - Bulb sampling mode This bit is set and cleared by software to enable the bulb sampling mode. SAMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type BULB_W<'a, REG> = crate::BitWriter<'a, REG, BULB_A>;
impl<'a, REG> BULB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bulb sampling mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BULB_A::B_0x0)
    }
    #[doc = "Bulb sampling mode enabled. The sampling period starts just after the previous end of conversion."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BULB_A::B_0x1)
    }
}
#[doc = "Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN bit should be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN bit is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPTRIG_A {
    #[doc = "0: Sampling time control trigger mode disabled"]
    B_0x0 = 0,
    #[doc = "1: Sampling time control trigger mode enabled"]
    B_0x1 = 1,
}
impl From<SMPTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPTRIG` reader - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN bit should be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN bit is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SMPTRIG_R = crate::BitReader<SMPTRIG_A>;
impl SMPTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPTRIG_A {
        match self.bits {
            false => SMPTRIG_A::B_0x0,
            true => SMPTRIG_A::B_0x1,
        }
    }
    #[doc = "Sampling time control trigger mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMPTRIG_A::B_0x0
    }
    #[doc = "Sampling time control trigger mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMPTRIG_A::B_0x1
    }
}
#[doc = "Field `SMPTRIG` writer - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN bit should be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN bit is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SMPTRIG_W<'a, REG> = crate::BitWriter<'a, REG, SMPTRIG_A>;
impl<'a, REG> SMPTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time control trigger mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG_A::B_0x0)
    }
    #[doc = "Sampling time control trigger mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPTRIG_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn ROVSE(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn JOVSE(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OVSR(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bitfield is set and cleared by software to define the right shifting applied to the raw oversampling result. Other codes reserved Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OVSS(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn TROVS(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ROVSM(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn SWTRIG(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bulb sampling mode This bit is set and cleared by software to enable the bulb sampling mode. SAMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn BULB(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN bit should be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN bit is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn SMPTRIG(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn ROVSE(&mut self) -> ROVSE_W<'_, CFGR2_SPEC> {
        ROVSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn JOVSE(&mut self) -> JOVSE_W<'_, CFGR2_SPEC> {
        JOVSE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OVSR(&mut self) -> OVSR_W<'_, CFGR2_SPEC> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bitfield is set and cleared by software to define the right shifting applied to the raw oversampling result. Other codes reserved Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn OVSS(&mut self) -> OVSS_W<'_, CFGR2_SPEC> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn TROVS(&mut self) -> TROVS_W<'_, CFGR2_SPEC> {
        TROVS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ROVSM(&mut self) -> ROVSM_W<'_, CFGR2_SPEC> {
        ROVSM_W::new(self, 10)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn SWTRIG(&mut self) -> SWTRIG_W<'_, CFGR2_SPEC> {
        SWTRIG_W::new(self, 25)
    }
    #[doc = "Bit 26 - Bulb sampling mode This bit is set and cleared by software to enable the bulb sampling mode. SAMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn BULB(&mut self) -> BULB_W<'_, CFGR2_SPEC> {
        BULB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN bit should be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN bit is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn SMPTRIG(&mut self) -> SMPTRIG_W<'_, CFGR2_SPEC> {
        SMPTRIG_W::new(self, 27)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {}
