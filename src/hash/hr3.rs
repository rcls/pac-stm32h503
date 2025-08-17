#[doc = "Register `HR3` reader"]
pub type R = crate::R<HR3_SPEC>;
#[doc = "Field `H3` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
#[doc = "HASH digest register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR3_SPEC;
impl crate::RegisterSpec for HR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr3::R`](R) reader structure"]
impl crate::Readable for HR3_SPEC {}
#[doc = "`reset()` method sets HR3 to value 0"]
impl crate::Resettable for HR3_SPEC {}
