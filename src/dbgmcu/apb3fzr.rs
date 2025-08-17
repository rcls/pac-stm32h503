#[doc = "Register `APB3FZR` reader"]
pub type R = crate::R<APB3FZR_SPEC>;
#[doc = "Register `APB3FZR` writer"]
pub type W = crate::W<APB3FZR_SPEC>;
#[doc = "I3C2 SCL stall counter stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I3C2_STOP_A {
    #[doc = "0: normal operation. I3C2 SCL stall timeout counter continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. I3C2 SCL stall timeout counter is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_I3C2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I3C2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I3C2_STOP` reader - I3C2 SCL stall counter stop in debug"]
pub type DBG_I3C2_STOP_R = crate::BitReader<DBG_I3C2_STOP_A>;
impl DBG_I3C2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I3C2_STOP_A {
        match self.bits {
            false => DBG_I3C2_STOP_A::B_0x0,
            true => DBG_I3C2_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. I3C2 SCL stall timeout counter continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_I3C2_STOP_A::B_0x0
    }
    #[doc = "stop in debug. I3C2 SCL stall timeout counter is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_I3C2_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_I3C2_STOP` writer - I3C2 SCL stall counter stop in debug"]
pub type DBG_I3C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I3C2_STOP_A>;
impl<'a, REG> DBG_I3C2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. I3C2 SCL stall timeout counter continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I3C2_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. I3C2 SCL stall timeout counter is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I3C2_STOP_A::B_0x1)
    }
}
#[doc = "LPTIM1 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM1_STOP_A {
    #[doc = "0: normal operation. LPTIM1 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_LPTIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader<DBG_LPTIM1_STOP_A>;
impl DBG_LPTIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM1_STOP_A {
        match self.bits {
            false => DBG_LPTIM1_STOP_A::B_0x0,
            true => DBG_LPTIM1_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. LPTIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_LPTIM1_STOP_A::B_0x0
    }
    #[doc = "stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_LPTIM1_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM1_STOP_A>;
impl<'a, REG> DBG_LPTIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. LPTIM1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. LPTIM1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP_A::B_0x1)
    }
}
#[doc = "RTC stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RTC_STOP_A {
    #[doc = "0: normal operation. RTC continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. RTC is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_RTC_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_RTC_STOP` reader - RTC stop in debug"]
pub type DBG_RTC_STOP_R = crate::BitReader<DBG_RTC_STOP_A>;
impl DBG_RTC_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_RTC_STOP_A {
        match self.bits {
            false => DBG_RTC_STOP_A::B_0x0,
            true => DBG_RTC_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. RTC continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_RTC_STOP_A::B_0x0
    }
    #[doc = "stop in debug. RTC is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_RTC_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - RTC stop in debug"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_RTC_STOP_A>;
impl<'a, REG> DBG_RTC_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. RTC continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. RTC is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 12 - I3C2 SCL stall counter stop in debug"]
    #[inline(always)]
    pub fn DBG_I3C2_STOP(&self) -> DBG_I3C2_STOP_R {
        DBG_I3C2_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn DBG_LPTIM1_STOP(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC stop in debug"]
    #[inline(always)]
    pub fn DBG_RTC_STOP(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - I3C2 SCL stall counter stop in debug"]
    #[inline(always)]
    pub fn DBG_I3C2_STOP(&mut self) -> DBG_I3C2_STOP_W<'_, APB3FZR_SPEC> {
        DBG_I3C2_STOP_W::new(self, 12)
    }
    #[doc = "Bit 17 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn DBG_LPTIM1_STOP(&mut self) -> DBG_LPTIM1_STOP_W<'_, APB3FZR_SPEC> {
        DBG_LPTIM1_STOP_W::new(self, 17)
    }
    #[doc = "Bit 30 - RTC stop in debug"]
    #[inline(always)]
    pub fn DBG_RTC_STOP(&mut self) -> DBG_RTC_STOP_W<'_, APB3FZR_SPEC> {
        DBG_RTC_STOP_W::new(self, 30)
    }
}
#[doc = "DBGMCU APB3 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3FZR_SPEC;
impl crate::RegisterSpec for APB3FZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3fzr::R`](R) reader structure"]
impl crate::Readable for APB3FZR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb3fzr::W`](W) writer structure"]
impl crate::Writable for APB3FZR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB3FZR to value 0"]
impl crate::Resettable for APB3FZR_SPEC {}
