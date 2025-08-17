#[doc = "Register `NSCCR` writer"]
pub type W = crate::W<NSCCR_SPEC>;
#[doc = "Field `CLR_EOP` writer - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
pub type CLR_EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_WRPERR` writer - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
pub type CLR_WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_PGSERR` writer - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
pub type CLR_PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_STRBERR` writer - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
pub type CLR_STRBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_INCERR` writer - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
pub type CLR_INCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OPTCHANGEERR` writer - Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 16 - EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
    #[inline(always)]
    pub fn CLR_EOP(&mut self) -> CLR_EOP_W<'_, NSCCR_SPEC> {
        CLR_EOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
    #[inline(always)]
    pub fn CLR_WRPERR(&mut self) -> CLR_WRPERR_W<'_, NSCCR_SPEC> {
        CLR_WRPERR_W::new(self, 17)
    }
    #[doc = "Bit 18 - PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
    #[inline(always)]
    pub fn CLR_PGSERR(&mut self) -> CLR_PGSERR_W<'_, NSCCR_SPEC> {
        CLR_PGSERR_W::new(self, 18)
    }
    #[doc = "Bit 19 - STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
    #[inline(always)]
    pub fn CLR_STRBERR(&mut self) -> CLR_STRBERR_W<'_, NSCCR_SPEC> {
        CLR_STRBERR_W::new(self, 19)
    }
    #[doc = "Bit 20 - INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
    #[inline(always)]
    pub fn CLR_INCERR(&mut self) -> CLR_INCERR_W<'_, NSCCR_SPEC> {
        CLR_INCERR_W::new(self, 20)
    }
    #[doc = "Bit 23 - Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
    #[inline(always)]
    pub fn CLR_OPTCHANGEERR(&mut self) -> CLR_OPTCHANGEERR_W<'_, NSCCR_SPEC> {
        CLR_OPTCHANGEERR_W::new(self, 23)
    }
}
#[doc = "FLASH non-secure clear control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSCCR_SPEC;
impl crate::RegisterSpec for NSCCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nsccr::W`](W) writer structure"]
impl crate::Writable for NSCCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets NSCCR to value 0"]
impl crate::Resettable for NSCCR_SPEC {}
