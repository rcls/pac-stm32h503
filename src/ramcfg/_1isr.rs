#[doc = "Register `1ISR` reader"]
pub type R = crate::R<_1ISR_SPEC>;
#[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEDC_A {
    #[doc = "0: No single error"]
    B_0x0 = 0,
    #[doc = "1: Single error detected and corrected"]
    B_0x1 = 1,
}
impl From<SEDC_A> for bool {
    #[inline(always)]
    fn from(variant: SEDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEDC` reader - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
pub type SEDC_R = crate::BitReader<SEDC_A>;
impl SEDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEDC_A {
        match self.bits {
            false => SEDC_A::B_0x0,
            true => SEDC_A::B_0x1,
        }
    }
    #[doc = "No single error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SEDC_A::B_0x0
    }
    #[doc = "Single error detected and corrected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SEDC_A::B_0x1
    }
}
#[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DED_A {
    #[doc = "0: No double error"]
    B_0x0 = 0,
    #[doc = "1: Double error detected"]
    B_0x1 = 1,
}
impl From<DED_A> for bool {
    #[inline(always)]
    fn from(variant: DED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DED` reader - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
pub type DED_R = crate::BitReader<DED_A>;
impl DED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DED_A {
        match self.bits {
            false => DED_A::B_0x0,
            true => DED_A::B_0x1,
        }
    }
    #[doc = "No double error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DED_A::B_0x0
    }
    #[doc = "Double error detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DED_A::B_0x1
    }
}
#[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMBUSY_A {
    #[doc = "0: No erase operation on going"]
    B_0x0 = 0,
    #[doc = "1: Erase operation on going"]
    B_0x1 = 1,
}
impl From<SRAMBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMBUSY` reader - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
pub type SRAMBUSY_R = crate::BitReader<SRAMBUSY_A>;
impl SRAMBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAMBUSY_A {
        match self.bits {
            false => SRAMBUSY_A::B_0x0,
            true => SRAMBUSY_A::B_0x1,
        }
    }
    #[doc = "No erase operation on going"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SRAMBUSY_A::B_0x0
    }
    #[doc = "Erase operation on going"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SRAMBUSY_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
    #[inline(always)]
    pub fn SEDC(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
    #[inline(always)]
    pub fn DED(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
    #[inline(always)]
    pub fn SRAMBUSY(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`_1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1ISR_SPEC;
impl crate::RegisterSpec for _1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1isr::R`](R) reader structure"]
impl crate::Readable for _1ISR_SPEC {}
#[doc = "`reset()` method sets 1ISR to value 0"]
impl crate::Resettable for _1ISR_SPEC {}
