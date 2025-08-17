#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISR_SPEC>;
#[doc = "ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDY_A {
    #[doc = "0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: ADC is ready to start conversion"]
    B_0x1 = 1,
}
impl From<ADRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type ADRDY_R = crate::BitReader<ADRDY_A>;
impl ADRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRDY_A {
        match self.bits {
            false => ADRDY_A::B_0x0,
            true => ADRDY_A::B_0x1,
        }
    }
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADRDY_A::B_0x0
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADRDY_A::B_0x1
    }
}
#[doc = "Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG, ADRDY_A>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDY_A::B_0x0)
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDY_A::B_0x1)
    }
}
#[doc = "End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMP_A {
    #[doc = "0: not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: End of sampling phase reached"]
    B_0x1 = 1,
}
impl From<EOSMP_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
pub type EOSMP_R = crate::BitReader<EOSMP_A>;
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMP_A {
        match self.bits {
            false => EOSMP_A::B_0x0,
            true => EOSMP_A::B_0x1,
        }
    }
    #[doc = "not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOSMP_A::B_0x0
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOSMP_A::B_0x1
    }
}
#[doc = "Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG, EOSMP_A>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMP_A::B_0x0)
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMP_A::B_0x1)
    }
}
#[doc = "End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC_A {
    #[doc = "0: Regular channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Regular channel conversion complete"]
    B_0x1 = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
pub type EOC_R = crate::BitReader<EOC_A>;
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::B_0x0,
            true => EOC_A::B_0x1,
        }
    }
    #[doc = "Regular channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOC_A::B_0x0
    }
    #[doc = "Regular channel conversion complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOC_A::B_0x1
    }
}
#[doc = "Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG, EOC_A>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOC_A::B_0x0)
    }
    #[doc = "Regular channel conversion complete"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOC_A::B_0x1)
    }
}
#[doc = "End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOS_A {
    #[doc = "0: Regular Conversions sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Regular Conversions sequence complete"]
    B_0x1 = 1,
}
impl From<EOS_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` reader - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
pub type EOS_R = crate::BitReader<EOS_A>;
impl EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOS_A {
        match self.bits {
            false => EOS_A::B_0x0,
            true => EOS_A::B_0x1,
        }
    }
    #[doc = "Regular Conversions sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOS_A::B_0x0
    }
    #[doc = "Regular Conversions sequence complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOS_A::B_0x1
    }
}
#[doc = "Field `EOS` writer - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG, EOS_A>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular Conversions sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOS_A::B_0x0)
    }
    #[doc = "Regular Conversions sequence complete"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOS_A::B_0x1)
    }
}
#[doc = "ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Overrun has occurred"]
    B_0x1 = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OVR_R = crate::BitReader<OVR_A>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::B_0x0,
            true => OVR_A::B_0x1,
        }
    }
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVR_A::B_0x0
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVR_A::B_0x1
    }
}
#[doc = "Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG, OVR_A>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::B_0x0)
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::B_0x1)
    }
}
#[doc = "Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC_A {
    #[doc = "0: Injected channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Injected channel conversion complete"]
    B_0x1 = 1,
}
impl From<JEOC_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` reader - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
pub type JEOC_R = crate::BitReader<JEOC_A>;
impl JEOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOC_A {
        match self.bits {
            false => JEOC_A::B_0x0,
            true => JEOC_A::B_0x1,
        }
    }
    #[doc = "Injected channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEOC_A::B_0x0
    }
    #[doc = "Injected channel conversion complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEOC_A::B_0x1
    }
}
#[doc = "Field `JEOC` writer - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG, JEOC_A>;
impl<'a, REG> JEOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JEOC_A::B_0x0)
    }
    #[doc = "Injected channel conversion complete"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JEOC_A::B_0x1)
    }
}
#[doc = "Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOS_A {
    #[doc = "0: Injected conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Injected conversions complete"]
    B_0x1 = 1,
}
impl From<JEOS_A> for bool {
    #[inline(always)]
    fn from(variant: JEOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOS` reader - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
pub type JEOS_R = crate::BitReader<JEOS_A>;
impl JEOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOS_A {
        match self.bits {
            false => JEOS_A::B_0x0,
            true => JEOS_A::B_0x1,
        }
    }
    #[doc = "Injected conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEOS_A::B_0x0
    }
    #[doc = "Injected conversions complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEOS_A::B_0x1
    }
}
#[doc = "Field `JEOS` writer - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
pub type JEOS_W<'a, REG> = crate::BitWriter<'a, REG, JEOS_A>;
impl<'a, REG> JEOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JEOS_A::B_0x0)
    }
    #[doc = "Injected conversions complete"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JEOS_A::B_0x1)
    }
}
#[doc = "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\] and HT1\\[11:0\\] of ADC_TR1 register. It is cleared by software. writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1_A {
    #[doc = "0: No analog watchdog 1 event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 1 event occurred"]
    B_0x1 = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\] and HT1\\[11:0\\] of ADC_TR1 register. It is cleared by software. writing 1 to it."]
pub type AWD1_R = crate::BitReader<AWD1_A>;
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::B_0x0,
            true => AWD1_A::B_0x1,
        }
    }
    #[doc = "No analog watchdog 1 event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD1_A::B_0x0
    }
    #[doc = "Analog watchdog 1 event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD1_A::B_0x1
    }
}
#[doc = "Field `AWD1` writer - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\] and HT1\\[11:0\\] of ADC_TR1 register. It is cleared by software. writing 1 to it."]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG, AWD1_A>;
impl<'a, REG> AWD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog 1 event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1_A::B_0x0)
    }
    #[doc = "Analog watchdog 1 event occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1_A::B_0x1)
    }
}
#[doc = "Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\] and HT2\\[7:0\\] of ADC_TR2 register. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2_A {
    #[doc = "0: No analog watchdog 2 event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 2 event occurred"]
    B_0x1 = 1,
}
impl From<AWD2_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2` reader - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\] and HT2\\[7:0\\] of ADC_TR2 register. It is cleared by software writing 1 to it."]
pub type AWD2_R = crate::BitReader<AWD2_A>;
impl AWD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD2_A {
        match self.bits {
            false => AWD2_A::B_0x0,
            true => AWD2_A::B_0x1,
        }
    }
    #[doc = "No analog watchdog 2 event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD2_A::B_0x0
    }
    #[doc = "Analog watchdog 2 event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD2_A::B_0x1
    }
}
#[doc = "Field `AWD2` writer - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\] and HT2\\[7:0\\] of ADC_TR2 register. It is cleared by software writing 1 to it."]
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG, AWD2_A>;
impl<'a, REG> AWD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog 2 event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2_A::B_0x0)
    }
    #[doc = "Analog watchdog 2 event occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2_A::B_0x1)
    }
}
#[doc = "Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\] and HT3\\[7:0\\] of ADC_TR3 register. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3_A {
    #[doc = "0: No analog watchdog 3 event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 3 event occurred"]
    B_0x1 = 1,
}
impl From<AWD3_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3` reader - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\] and HT3\\[7:0\\] of ADC_TR3 register. It is cleared by software writing 1 to it."]
pub type AWD3_R = crate::BitReader<AWD3_A>;
impl AWD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD3_A {
        match self.bits {
            false => AWD3_A::B_0x0,
            true => AWD3_A::B_0x1,
        }
    }
    #[doc = "No analog watchdog 3 event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD3_A::B_0x0
    }
    #[doc = "Analog watchdog 3 event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD3_A::B_0x1
    }
}
#[doc = "Field `AWD3` writer - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\] and HT3\\[7:0\\] of ADC_TR3 register. It is cleared by software writing 1 to it."]
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG, AWD3_A>;
impl<'a, REG> AWD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog 3 event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3_A::B_0x0)
    }
    #[doc = "Analog watchdog 3 event occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3_A::B_0x1)
    }
}
#[doc = "Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVF_A {
    #[doc = "0: No injected context queue overflow occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0x0 = 0,
    #[doc = "1: Injected context queue overflow has occurred"]
    B_0x1 = 1,
}
impl From<JQOVF_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVF` reader - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
pub type JQOVF_R = crate::BitReader<JQOVF_A>;
impl JQOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQOVF_A {
        match self.bits {
            false => JQOVF_A::B_0x0,
            true => JQOVF_A::B_0x1,
        }
    }
    #[doc = "No injected context queue overflow occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JQOVF_A::B_0x0
    }
    #[doc = "Injected context queue overflow has occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JQOVF_A::B_0x1
    }
}
#[doc = "Field `JQOVF` writer - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
pub type JQOVF_W<'a, REG> = crate::BitWriter<'a, REG, JQOVF_A>;
impl<'a, REG> JQOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No injected context queue overflow occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVF_A::B_0x0)
    }
    #[doc = "Injected context queue overflow has occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVF_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ADRDY(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
    #[inline(always)]
    pub fn EOSMP(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
    #[inline(always)]
    pub fn EOC(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn EOS(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn OVR(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
    #[inline(always)]
    pub fn JEOC(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn JEOS(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\] and HT1\\[11:0\\] of ADC_TR1 register. It is cleared by software. writing 1 to it."]
    #[inline(always)]
    pub fn AWD1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\] and HT2\\[7:0\\] of ADC_TR2 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn AWD2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\] and HT3\\[7:0\\] of ADC_TR3 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn AWD3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
    #[inline(always)]
    pub fn JQOVF(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN = 1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ADRDY(&mut self) -> ADRDY_W<'_, ISR_SPEC> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase."]
    #[inline(always)]
    pub fn EOSMP(&mut self) -> EOSMP_W<'_, ISR_SPEC> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register"]
    #[inline(always)]
    pub fn EOC(&mut self) -> EOC_W<'_, ISR_SPEC> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn EOS(&mut self) -> EOS_W<'_, ISR_SPEC> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn OVR(&mut self) -> OVR_W<'_, ISR_SPEC> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by reading the corresponding ADC_JDRy register"]
    #[inline(always)]
    pub fn JEOC(&mut self) -> JEOC_W<'_, ISR_SPEC> {
        JEOC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn JEOS(&mut self) -> JEOS_W<'_, ISR_SPEC> {
        JEOS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1\\[11:0\\] and HT1\\[11:0\\] of ADC_TR1 register. It is cleared by software. writing 1 to it."]
    #[inline(always)]
    pub fn AWD1(&mut self) -> AWD1_W<'_, ISR_SPEC> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2\\[7:0\\] and HT2\\[7:0\\] of ADC_TR2 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn AWD2(&mut self) -> AWD2_W<'_, ISR_SPEC> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3\\[7:0\\] and HT3\\[7:0\\] of ADC_TR3 register. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn AWD3(&mut self) -> AWD3_W<'_, ISR_SPEC> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 10 - Injected context queue overflow This bit is set by hardware when an Overflow of the Injected Queue of Context occurs. It is cleared by software writing 1 to it. Refer to for more information."]
    #[inline(always)]
    pub fn JQOVF(&mut self) -> JQOVF_W<'_, ISR_SPEC> {
        JQOVF_W::new(self, 10)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {}
