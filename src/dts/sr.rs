#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_ITEF_A {
    #[doc = "0: No end of measurement detected on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: End of measure detected on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_ITEF_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_ITEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_ITEF` reader - Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set"]
pub type TS1_ITEF_R = crate::BitReader<TS1_ITEF_A>;
impl TS1_ITEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_ITEF_A {
        match self.bits {
            false => TS1_ITEF_A::B_0x0,
            true => TS1_ITEF_A::B_0x1,
        }
    }
    #[doc = "No end of measurement detected on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_ITEF_A::B_0x0
    }
    #[doc = "End of measure detected on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_ITEF_A::B_0x1
    }
}
#[doc = "Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_ITLF_A {
    #[doc = "0: Low threshold not reached on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Low threshold reached on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_ITLF_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_ITLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_ITLF` reader - Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set"]
pub type TS1_ITLF_R = crate::BitReader<TS1_ITLF_A>;
impl TS1_ITLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_ITLF_A {
        match self.bits {
            false => TS1_ITLF_A::B_0x0,
            true => TS1_ITLF_A::B_0x1,
        }
    }
    #[doc = "Low threshold not reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_ITLF_A::B_0x0
    }
    #[doc = "Low threshold reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_ITLF_A::B_0x1
    }
}
#[doc = "Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_ITHF_A {
    #[doc = "0: High threshold not reached on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: High threshold reached on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_ITHF_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_ITHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_ITHF` reader - Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set"]
pub type TS1_ITHF_R = crate::BitReader<TS1_ITHF_A>;
impl TS1_ITHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_ITHF_A {
        match self.bits {
            false => TS1_ITHF_A::B_0x0,
            true => TS1_ITHF_A::B_0x1,
        }
    }
    #[doc = "High threshold not reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_ITHF_A::B_0x0
    }
    #[doc = "High threshold reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_ITHF_A::B_0x1
    }
}
#[doc = "Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_AITEF_A {
    #[doc = "0: End of measure not detected on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: End of measure detected on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_AITEF_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_AITEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_AITEF` reader - Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set"]
pub type TS1_AITEF_R = crate::BitReader<TS1_AITEF_A>;
impl TS1_AITEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_AITEF_A {
        match self.bits {
            false => TS1_AITEF_A::B_0x0,
            true => TS1_AITEF_A::B_0x1,
        }
    }
    #[doc = "End of measure not detected on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_AITEF_A::B_0x0
    }
    #[doc = "End of measure detected on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_AITEF_A::B_0x1
    }
}
#[doc = "Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_AITLF_A {
    #[doc = "0: Low threshold not reached on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: Low threshold reached on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_AITLF_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_AITLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_AITLF` reader - Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set"]
pub type TS1_AITLF_R = crate::BitReader<TS1_AITLF_A>;
impl TS1_AITLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_AITLF_A {
        match self.bits {
            false => TS1_AITLF_A::B_0x0,
            true => TS1_AITLF_A::B_0x1,
        }
    }
    #[doc = "Low threshold not reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_AITLF_A::B_0x0
    }
    #[doc = "Low threshold reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_AITLF_A::B_0x1
    }
}
#[doc = "Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_AITHF_A {
    #[doc = "0: High threshold not reached on temperature sensor 1"]
    B_0x0 = 0,
    #[doc = "1: High threshold reached on temperature sensor 1"]
    B_0x1 = 1,
}
impl From<TS1_AITHF_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_AITHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_AITHF` reader - Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set"]
pub type TS1_AITHF_R = crate::BitReader<TS1_AITHF_A>;
impl TS1_AITHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_AITHF_A {
        match self.bits {
            false => TS1_AITHF_A::B_0x0,
            true => TS1_AITHF_A::B_0x1,
        }
    }
    #[doc = "High threshold not reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_AITHF_A::B_0x0
    }
    #[doc = "High threshold reached on temperature sensor 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_AITHF_A::B_0x1
    }
}
#[doc = "Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_RDY_A {
    #[doc = "0: Temperature sensor 1 busy"]
    B_0x0 = 0,
    #[doc = "1: Temperature sensor 1 ready"]
    B_0x1 = 1,
}
impl From<TS1_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_RDY` reader - Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing."]
pub type TS1_RDY_R = crate::BitReader<TS1_RDY_A>;
impl TS1_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_RDY_A {
        match self.bits {
            false => TS1_RDY_A::B_0x0,
            true => TS1_RDY_A::B_0x1,
        }
    }
    #[doc = "Temperature sensor 1 busy"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_RDY_A::B_0x0
    }
    #[doc = "Temperature sensor 1 ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_RDY_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set"]
    #[inline(always)]
    pub fn TS1_ITEF(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set"]
    #[inline(always)]
    pub fn TS1_ITLF(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set"]
    #[inline(always)]
    pub fn TS1_ITHF(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set"]
    #[inline(always)]
    pub fn TS1_AITEF(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set"]
    #[inline(always)]
    pub fn TS1_AITLF(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set"]
    #[inline(always)]
    pub fn TS1_AITHF(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing."]
    #[inline(always)]
    pub fn TS1_RDY(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Temperature sensor status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
