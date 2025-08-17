#[doc = "Register `3ECCKEYR` writer"]
pub type W = crate::W<_3ECCKEYR_SPEC>;
#[doc = "Field `ECCKEY` writer - ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
    #[inline(always)]
    pub fn ECCKEY(&mut self) -> ECCKEY_W<'_, _3ECCKEYR_SPEC> {
        ECCKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG memory 3 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3ECCKEYR_SPEC;
impl crate::RegisterSpec for _3ECCKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3ecckeyr::W`](W) writer structure"]
impl crate::Writable for _3ECCKEYR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets 3ECCKEYR to value 0"]
impl crate::Resettable for _3ECCKEYR_SPEC {}
