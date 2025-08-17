#[doc = "Register `APB1HFZR` reader"]
pub type R = crate::R<APB1HFZR_SPEC>;
#[doc = "Register `APB1HFZR` writer"]
pub type W = crate::W<APB1HFZR_SPEC>;
#[doc = "LPTIM2 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM2_STOP_A {
    #[doc = "0: normal operation. LPTIM2 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_LPTIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 stop in debug"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<DBG_LPTIM2_STOP_A>;
impl DBG_LPTIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM2_STOP_A {
        match self.bits {
            false => DBG_LPTIM2_STOP_A::B_0x0,
            true => DBG_LPTIM2_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. LPTIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::B_0x0
    }
    #[doc = "stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 stop in debug"]
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM2_STOP_A>;
impl<'a, REG> DBG_LPTIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. LPTIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. LPTIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 5 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn DBG_LPTIM2_STOP(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn DBG_LPTIM2_STOP(&mut self) -> DBG_LPTIM2_STOP_W<'_, APB1HFZR_SPEC> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
}
#[doc = "DBGMCU APB1H peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hfzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HFZR_SPEC;
impl crate::RegisterSpec for APB1HFZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hfzr::R`](R) reader structure"]
impl crate::Readable for APB1HFZR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1hfzr::W`](W) writer structure"]
impl crate::Writable for APB1HFZR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1HFZR to value 0"]
impl crate::Resettable for APB1HFZR_SPEC {}
