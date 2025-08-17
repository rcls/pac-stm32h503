#[doc = "Register `HRA4` reader"]
pub type R = crate::R<HRA4_SPEC>;
#[doc = "Field `H4` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H4(&self) -> H4_R {
        H4_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hra4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA4_SPEC;
impl crate::RegisterSpec for HRA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra4::R`](R) reader structure"]
impl crate::Readable for HRA4_SPEC {}
#[doc = "`reset()` method sets HRA4 to value 0"]
impl crate::Resettable for HRA4_SPEC {}
