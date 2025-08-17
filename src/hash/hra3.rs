#[doc = "Register `HRA3` reader"]
pub type R = crate::R<HRA3_SPEC>;
#[doc = "Field `H3` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hra3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA3_SPEC;
impl crate::RegisterSpec for HRA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra3::R`](R) reader structure"]
impl crate::Readable for HRA3_SPEC {}
#[doc = "`reset()` method sets HRA3 to value 0"]
impl crate::Resettable for HRA3_SPEC {}
