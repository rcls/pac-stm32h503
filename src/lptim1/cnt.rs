#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Field `CNT` reader - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    #[inline(always)]
    pub fn CNT(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LPTIM counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {}
