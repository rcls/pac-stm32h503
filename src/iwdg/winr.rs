#[doc = "Register `WINR` reader"]
pub type R = crate::R<WINR_SPEC>;
#[doc = "Register `WINR` writer"]
pub type W = crate::W<WINR_SPEC>;
#[doc = "Field `WIN` reader - Watchdog counter window value These bits are write access protected, see , they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]+1 and greater than 1. The WVU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the (IWDG_SR) is reset."]
pub type WIN_R = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Watchdog counter window value These bits are write access protected, see , they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]+1 and greater than 1. The WVU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the (IWDG_SR) is reset."]
pub type WIN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected, see , they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]+1 and greater than 1. The WVU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn WIN(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected, see , they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than WIN\\[11:0\\]+1 and greater than 1. The WVU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VDD voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn WIN(&mut self) -> WIN_W<'_, WINR_SPEC> {
        WIN_W::new(self, 0)
    }
}
#[doc = "IWDG window register\n\nYou can [`read`](crate::Reg::read) this register and get [`winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINR_SPEC;
impl crate::RegisterSpec for WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winr::R`](R) reader structure"]
impl crate::Readable for WINR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`winr::W`](W) writer structure"]
impl crate::Writable for WINR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WINR to value 0x0fff"]
impl crate::Resettable for WINR_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
