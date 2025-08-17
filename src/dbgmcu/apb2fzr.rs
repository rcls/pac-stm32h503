#[doc = "Register `APB2FZR` reader"]
pub type R = crate::R<APB2FZR_SPEC>;
#[doc = "Register `APB2FZR` writer"]
pub type W = crate::W<APB2FZR_SPEC>;
#[doc = "TIM1 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM1_STOP_A {
    #[doc = "0: normal operation. TIM1 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. TIM1 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_TIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 stop in debug"]
pub type DBG_TIM1_STOP_R = crate::BitReader<DBG_TIM1_STOP_A>;
impl DBG_TIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM1_STOP_A {
        match self.bits {
            false => DBG_TIM1_STOP_A::B_0x0,
            true => DBG_TIM1_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. TIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_TIM1_STOP_A::B_0x0
    }
    #[doc = "stop in debug. TIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_TIM1_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 stop in debug"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM1_STOP_A>;
impl<'a, REG> DBG_TIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. TIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM1_STOP(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM1_STOP(&mut self) -> DBG_TIM1_STOP_W<'_, APB2FZR_SPEC> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2FZR_SPEC;
impl crate::RegisterSpec for APB2FZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2fzr::R`](R) reader structure"]
impl crate::Readable for APB2FZR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2fzr::W`](W) writer structure"]
impl crate::Writable for APB2FZR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB2FZR to value 0"]
impl crate::Resettable for APB2FZR_SPEC {}
