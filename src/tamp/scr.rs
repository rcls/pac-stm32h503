#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `CTAMP1F` writer - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` writer - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP1F` writer - Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP2F` writer - Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
pub type CITAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` writer - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP4F` writer - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` writer - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP6F` writer - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP7F` writer - Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
pub type CITAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP8F` writer - Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
pub type CITAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP9F` writer - Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
pub type CITAMP9F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP11F` writer - Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
pub type CITAMP11F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP12F` writer - Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
pub type CITAMP12F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP13F` writer - Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
pub type CITAMP13F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP15F` writer - Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
pub type CITAMP15F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CTAMP1F(&mut self) -> CTAMP1F_W<'_, SCR_SPEC> {
        CTAMP1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CTAMP2F(&mut self) -> CTAMP2F_W<'_, SCR_SPEC> {
        CTAMP2F_W::new(self, 1)
    }
    #[doc = "Bit 16 - Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP1F(&mut self) -> CITAMP1F_W<'_, SCR_SPEC> {
        CITAMP1F_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP2F(&mut self) -> CITAMP2F_W<'_, SCR_SPEC> {
        CITAMP2F_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP3F(&mut self) -> CITAMP3F_W<'_, SCR_SPEC> {
        CITAMP3F_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP4F(&mut self) -> CITAMP4F_W<'_, SCR_SPEC> {
        CITAMP4F_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP5F(&mut self) -> CITAMP5F_W<'_, SCR_SPEC> {
        CITAMP5F_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP6F(&mut self) -> CITAMP6F_W<'_, SCR_SPEC> {
        CITAMP6F_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP7F(&mut self) -> CITAMP7F_W<'_, SCR_SPEC> {
        CITAMP7F_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP8F(&mut self) -> CITAMP8F_W<'_, SCR_SPEC> {
        CITAMP8F_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP9F(&mut self) -> CITAMP9F_W<'_, SCR_SPEC> {
        CITAMP9F_W::new(self, 24)
    }
    #[doc = "Bit 26 - Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP11F(&mut self) -> CITAMP11F_W<'_, SCR_SPEC> {
        CITAMP11F_W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP12F(&mut self) -> CITAMP12F_W<'_, SCR_SPEC> {
        CITAMP12F_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP13F(&mut self) -> CITAMP13F_W<'_, SCR_SPEC> {
        CITAMP13F_W::new(self, 28)
    }
    #[doc = "Bit 30 - Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
    #[inline(always)]
    pub fn CITAMP15F(&mut self) -> CITAMP15F_W<'_, SCR_SPEC> {
        CITAMP15F_W::new(self, 30)
    }
}
#[doc = "TAMP status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {}
