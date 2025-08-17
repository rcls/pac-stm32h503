#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE_A {
    #[doc = "0: ADRDY interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    B_0x1 = 1,
}
impl From<ADRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ADRDYIE_R = crate::BitReader<ADRDYIE_A>;
impl ADRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYIE_A {
        match self.bits {
            false => ADRDYIE_A::B_0x0,
            true => ADRDYIE_A::B_0x1,
        }
    }
    #[doc = "ADRDY interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADRDYIE_A::B_0x0
    }
    #[doc = "ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADRDYIE_A::B_0x1
    }
}
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type ADRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, ADRDYIE_A>;
impl<'a, REG> ADRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADRDY interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE_A::B_0x0)
    }
    #[doc = "ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYIE_A::B_0x1)
    }
}
#[doc = "End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE_A {
    #[doc = "0: EOSMP interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    B_0x1 = 1,
}
impl From<EOSMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMPIE` reader - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EOSMPIE_R = crate::BitReader<EOSMPIE_A>;
impl EOSMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPIE_A {
        match self.bits {
            false => EOSMPIE_A::B_0x0,
            true => EOSMPIE_A::B_0x1,
        }
    }
    #[doc = "EOSMP interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOSMPIE_A::B_0x0
    }
    #[doc = "EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOSMPIE_A::B_0x1
    }
}
#[doc = "Field `EOSMPIE` writer - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EOSMPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSMPIE_A>;
impl<'a, REG> EOSMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOSMP interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE_A::B_0x0)
    }
    #[doc = "EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPIE_A::B_0x1)
    }
}
#[doc = "End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    #[doc = "0: EOC interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    B_0x1 = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::B_0x0,
            true => EOCIE_A::B_0x1,
        }
    }
    #[doc = "EOC interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOCIE_A::B_0x0
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOCIE_A::B_0x1
    }
}
#[doc = "Field `EOCIE` writer - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG, EOCIE_A>;
impl<'a, REG> EOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOC interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE_A::B_0x0)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOCIE_A::B_0x1)
    }
}
#[doc = "End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSIE_A {
    #[doc = "0: EOS interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    B_0x1 = 1,
}
impl From<EOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSIE` reader - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EOSIE_R = crate::BitReader<EOSIE_A>;
impl EOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSIE_A {
        match self.bits {
            false => EOSIE_A::B_0x0,
            true => EOSIE_A::B_0x1,
        }
    }
    #[doc = "EOS interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOSIE_A::B_0x0
    }
    #[doc = "EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOSIE_A::B_0x1
    }
}
#[doc = "Field `EOSIE` writer - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type EOSIE_W<'a, REG> = crate::BitWriter<'a, REG, EOSIE_A>;
impl<'a, REG> EOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOS interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE_A::B_0x0)
    }
    #[doc = "EOS interrupt enabled. An interrupt is generated when the EOS bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSIE_A::B_0x1)
    }
}
#[doc = "Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE_A {
    #[doc = "0: Overrun interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    B_0x1 = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRIE` reader - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type OVRIE_R = crate::BitReader<OVRIE_A>;
impl OVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::B_0x0,
            true => OVRIE_A::B_0x1,
        }
    }
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OVRIE_A::B_0x0
    }
    #[doc = "Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OVRIE_A::B_0x1
    }
}
#[doc = "Field `OVRIE` writer - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRIE_A>;
impl<'a, REG> OVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE_A::B_0x0)
    }
    #[doc = "Overrun interrupt enabled. An interrupt is generated when the OVR bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRIE_A::B_0x1)
    }
}
#[doc = "End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE_A {
    #[doc = "0: JEOC interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set."]
    B_0x1 = 1,
}
impl From<JEOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOCIE` reader - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JEOCIE_R = crate::BitReader<JEOCIE_A>;
impl JEOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOCIE_A {
        match self.bits {
            false => JEOCIE_A::B_0x0,
            true => JEOCIE_A::B_0x1,
        }
    }
    #[doc = "JEOC interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEOCIE_A::B_0x0
    }
    #[doc = "JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEOCIE_A::B_0x1
    }
}
#[doc = "Field `JEOCIE` writer - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOCIE_A>;
impl<'a, REG> JEOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JEOC interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE_A::B_0x0)
    }
    #[doc = "JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE_A::B_0x1)
    }
}
#[doc = "End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSIE_A {
    #[doc = "0: JEOS interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: JEOS interrupt enabled. An interrupt is generated when the JEOS bit is set."]
    B_0x1 = 1,
}
impl From<JEOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOSIE` reader - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JEOSIE_R = crate::BitReader<JEOSIE_A>;
impl JEOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOSIE_A {
        match self.bits {
            false => JEOSIE_A::B_0x0,
            true => JEOSIE_A::B_0x1,
        }
    }
    #[doc = "JEOS interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEOSIE_A::B_0x0
    }
    #[doc = "JEOS interrupt enabled. An interrupt is generated when the JEOS bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEOSIE_A::B_0x1
    }
}
#[doc = "Field `JEOSIE` writer - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JEOSIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOSIE_A>;
impl<'a, REG> JEOSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JEOS interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSIE_A::B_0x0)
    }
    #[doc = "JEOS interrupt enabled. An interrupt is generated when the JEOS bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSIE_A::B_0x1)
    }
}
#[doc = "Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1IE_A {
    #[doc = "0: Analog watchdog 1 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 1 interrupt enabled"]
    B_0x1 = 1,
}
impl From<AWD1IE_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1IE` reader - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1IE_R = crate::BitReader<AWD1IE_A>;
impl AWD1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1IE_A {
        match self.bits {
            false => AWD1IE_A::B_0x0,
            true => AWD1IE_A::B_0x1,
        }
    }
    #[doc = "Analog watchdog 1 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD1IE_A::B_0x0
    }
    #[doc = "Analog watchdog 1 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD1IE_A::B_0x1
    }
}
#[doc = "Field `AWD1IE` writer - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD1IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD1IE_A>;
impl<'a, REG> AWD1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE_A::B_0x0)
    }
    #[doc = "Analog watchdog 1 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1IE_A::B_0x1)
    }
}
#[doc = "Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2IE_A {
    #[doc = "0: Analog watchdog 2 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 2 interrupt enabled"]
    B_0x1 = 1,
}
impl From<AWD2IE_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2IE` reader - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD2IE_R = crate::BitReader<AWD2IE_A>;
impl AWD2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD2IE_A {
        match self.bits {
            false => AWD2IE_A::B_0x0,
            true => AWD2IE_A::B_0x1,
        }
    }
    #[doc = "Analog watchdog 2 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD2IE_A::B_0x0
    }
    #[doc = "Analog watchdog 2 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD2IE_A::B_0x1
    }
}
#[doc = "Field `AWD2IE` writer - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD2IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD2IE_A>;
impl<'a, REG> AWD2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 2 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2IE_A::B_0x0)
    }
    #[doc = "Analog watchdog 2 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2IE_A::B_0x1)
    }
}
#[doc = "Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3IE_A {
    #[doc = "0: Analog watchdog 3 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Analog watchdog 3 interrupt enabled"]
    B_0x1 = 1,
}
impl From<AWD3IE_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3IE` reader - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD3IE_R = crate::BitReader<AWD3IE_A>;
impl AWD3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD3IE_A {
        match self.bits {
            false => AWD3IE_A::B_0x0,
            true => AWD3IE_A::B_0x1,
        }
    }
    #[doc = "Analog watchdog 3 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWD3IE_A::B_0x0
    }
    #[doc = "Analog watchdog 3 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWD3IE_A::B_0x1
    }
}
#[doc = "Field `AWD3IE` writer - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWD3IE_W<'a, REG> = crate::BitWriter<'a, REG, AWD3IE_A>;
impl<'a, REG> AWD3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 3 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3IE_A::B_0x0)
    }
    #[doc = "Analog watchdog 3 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3IE_A::B_0x1)
    }
}
#[doc = "Injected context queue overflow interrupt enable This bit is set and cleared by software to enable/disable the Injected Context Queue Overflow interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFIE_A {
    #[doc = "0: Injected Context Queue Overflow interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Injected Context Queue Overflow interrupt enabled. An interrupt is generated when the JQOVF bit is set."]
    B_0x1 = 1,
}
impl From<JQOVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVFIE` reader - Injected context queue overflow interrupt enable This bit is set and cleared by software to enable/disable the Injected Context Queue Overflow interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JQOVFIE_R = crate::BitReader<JQOVFIE_A>;
impl JQOVFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQOVFIE_A {
        match self.bits {
            false => JQOVFIE_A::B_0x0,
            true => JQOVFIE_A::B_0x1,
        }
    }
    #[doc = "Injected Context Queue Overflow interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JQOVFIE_A::B_0x0
    }
    #[doc = "Injected Context Queue Overflow interrupt enabled. An interrupt is generated when the JQOVF bit is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JQOVFIE_A::B_0x1
    }
}
#[doc = "Field `JQOVFIE` writer - Injected context queue overflow interrupt enable This bit is set and cleared by software to enable/disable the Injected Context Queue Overflow interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
pub type JQOVFIE_W<'a, REG> = crate::BitWriter<'a, REG, JQOVFIE_A>;
impl<'a, REG> JQOVFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected Context Queue Overflow interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFIE_A::B_0x0)
    }
    #[doc = "Injected Context Queue Overflow interrupt enabled. An interrupt is generated when the JQOVF bit is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ADRDYIE(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EOSMPIE(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EOCIE(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EOSIE(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn OVRIE(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JEOCIE(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JEOSIE(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1IE(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD2IE(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD3IE(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected context queue overflow interrupt enable This bit is set and cleared by software to enable/disable the Injected Context Queue Overflow interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JQOVFIE(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ADRDYIE(&mut self) -> ADRDYIE_W<'_, IER_SPEC> {
        ADRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EOSMPIE(&mut self) -> EOSMPIE_W<'_, IER_SPEC> {
        EOSMPIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EOCIE(&mut self) -> EOCIE_W<'_, IER_SPEC> {
        EOCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn EOSIE(&mut self) -> EOSIE_W<'_, IER_SPEC> {
        EOSIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn OVRIE(&mut self) -> OVRIE_W<'_, IER_SPEC> {
        OVRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JEOCIE(&mut self) -> JEOCIE_W<'_, IER_SPEC> {
        JEOCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JEOSIE(&mut self) -> JEOSIE_W<'_, IER_SPEC> {
        JEOSIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD1IE(&mut self) -> AWD1IE_W<'_, IER_SPEC> {
        AWD1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD2IE(&mut self) -> AWD2IE_W<'_, IER_SPEC> {
        AWD2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWD3IE(&mut self) -> AWD3IE_W<'_, IER_SPEC> {
        AWD3IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Injected context queue overflow interrupt enable This bit is set and cleared by software to enable/disable the Injected Context Queue Overflow interrupt. Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no injected conversion is ongoing)."]
    #[inline(always)]
    pub fn JQOVFIE(&mut self) -> JQOVFIE_W<'_, IER_SPEC> {
        JQOVFIE_W::new(self, 10)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {}
