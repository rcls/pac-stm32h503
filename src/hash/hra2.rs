#[doc = "Register `HRA2` reader"]
pub type R = crate::R<HRA2_SPEC>;
#[doc = "Field `H2` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hra2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA2_SPEC;
impl crate::RegisterSpec for HRA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra2::R`](R) reader structure"]
impl crate::Readable for HRA2_SPEC {}
#[doc = "`reset()` method sets HRA2 to value 0"]
impl crate::Resettable for HRA2_SPEC {}
