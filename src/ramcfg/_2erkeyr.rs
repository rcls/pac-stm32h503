#[doc = "Register `2ERKEYR` writer"]
pub type W = crate::W<_2ERKEYR_SPEC>;
#[doc = "Field `ERASEKEY` writer - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
    #[inline(always)]
    pub fn ERASEKEY(&mut self) -> ERASEKEY_W<'_, _2ERKEYR_SPEC> {
        ERASEKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG memory 2 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2ERKEYR_SPEC;
impl crate::RegisterSpec for _2ERKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_2erkeyr::W`](W) writer structure"]
impl crate::Writable for _2ERKEYR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets 2ERKEYR to value 0"]
impl crate::Resettable for _2ERKEYR_SPEC {}
