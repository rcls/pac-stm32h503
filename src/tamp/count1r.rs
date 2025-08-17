#[doc = "Register `COUNT1R` reader"]
pub type R = crate::R<COUNT1R_SPEC>;
#[doc = "Field `COUNT` reader - This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
    #[inline(always)]
    pub fn COUNT(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "TAMP monotonic counter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`count1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT1R_SPEC;
impl crate::RegisterSpec for COUNT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count1r::R`](R) reader structure"]
impl crate::Readable for COUNT1R_SPEC {}
#[doc = "`reset()` method sets COUNT1R to value 0"]
impl crate::Resettable for COUNT1R_SPEC {}
