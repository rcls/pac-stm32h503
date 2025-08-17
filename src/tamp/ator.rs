#[doc = "Register `ATOR` reader"]
pub type R = crate::R<ATOR_SPEC>;
#[doc = "Field `PRNG` reader - Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value."]
pub type PRNG_R = crate::FieldReader;
#[doc = "Field `SEEDF` reader - Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
pub type SEEDF_R = crate::BitReader;
#[doc = "Field `INITS` reader - Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
pub type INITS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value."]
    #[inline(always)]
    pub fn PRNG(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
    #[inline(always)]
    pub fn SEEDF(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
    #[inline(always)]
    pub fn INITS(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "TAMP active tamper output register\n\nYou can [`read`](crate::Reg::read) this register and get [`ator::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATOR_SPEC;
impl crate::RegisterSpec for ATOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ator::R`](R) reader structure"]
impl crate::Readable for ATOR_SPEC {}
#[doc = "`reset()` method sets ATOR to value 0"]
impl crate::Resettable for ATOR_SPEC {}
