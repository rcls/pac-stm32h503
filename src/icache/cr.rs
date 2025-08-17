#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: cache disabled"]
    B_0x0 = 0,
    #[doc = "1: cache enabled"]
    B_0x1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0x0,
            true => EN_A::B_0x1,
        }
    }
    #[doc = "cache disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN_A::B_0x0
    }
    #[doc = "cache enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN_A::B_0x1
    }
}
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cache disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x0)
    }
    #[doc = "cache enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x1)
    }
}
#[doc = "cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHEINV_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: invalidate entire cache (all cache lines valid bit = 0)"]
    B_0x1 = 1,
}
impl From<CACHEINV_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEINV` writer - cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG, CACHEINV_A>;
impl<'a, REG> CACHEINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CACHEINV_A::B_0x0)
    }
    #[doc = "invalidate entire cache (all cache lines valid bit = 0)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CACHEINV_A::B_0x1)
    }
}
#[doc = "cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAYSEL_A {
    #[doc = "0: direct mapped cache (1-way cache)"]
    B_0x0 = 0,
    #[doc = "1: n-way set associative cache (reset value)"]
    B_0x1 = 1,
}
impl From<WAYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WAYSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAYSEL` reader - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
pub type WAYSEL_R = crate::BitReader<WAYSEL_A>;
impl WAYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAYSEL_A {
        match self.bits {
            false => WAYSEL_A::B_0x0,
            true => WAYSEL_A::B_0x1,
        }
    }
    #[doc = "direct mapped cache (1-way cache)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WAYSEL_A::B_0x0
    }
    #[doc = "n-way set associative cache (reset value)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WAYSEL_A::B_0x1
    }
}
#[doc = "Field `WAYSEL` writer - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
pub type WAYSEL_W<'a, REG> = crate::BitWriter<'a, REG, WAYSEL_A>;
impl<'a, REG> WAYSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "direct mapped cache (1-way cache)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAYSEL_A::B_0x0)
    }
    #[doc = "n-way set associative cache (reset value)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAYSEL_A::B_0x1)
    }
}
#[doc = "hit monitor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HITMEN_A {
    #[doc = "0: cache hit monitor switched off. Stopping the monitor does not reset it."]
    B_0x0 = 0,
    #[doc = "1: cache hit monitor enabled"]
    B_0x1 = 1,
}
impl From<HITMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HITMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HITMEN` reader - hit monitor enable"]
pub type HITMEN_R = crate::BitReader<HITMEN_A>;
impl HITMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HITMEN_A {
        match self.bits {
            false => HITMEN_A::B_0x0,
            true => HITMEN_A::B_0x1,
        }
    }
    #[doc = "cache hit monitor switched off. Stopping the monitor does not reset it."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HITMEN_A::B_0x0
    }
    #[doc = "cache hit monitor enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HITMEN_A::B_0x1
    }
}
#[doc = "Field `HITMEN` writer - hit monitor enable"]
pub type HITMEN_W<'a, REG> = crate::BitWriter<'a, REG, HITMEN_A>;
impl<'a, REG> HITMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cache hit monitor switched off. Stopping the monitor does not reset it."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HITMEN_A::B_0x0)
    }
    #[doc = "cache hit monitor enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HITMEN_A::B_0x1)
    }
}
#[doc = "miss monitor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSMEN_A {
    #[doc = "0: cache miss monitor switched off. Stopping the monitor does not reset it."]
    B_0x0 = 0,
    #[doc = "1: cache miss monitor enabled"]
    B_0x1 = 1,
}
impl From<MISSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: MISSMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISSMEN` reader - miss monitor enable"]
pub type MISSMEN_R = crate::BitReader<MISSMEN_A>;
impl MISSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MISSMEN_A {
        match self.bits {
            false => MISSMEN_A::B_0x0,
            true => MISSMEN_A::B_0x1,
        }
    }
    #[doc = "cache miss monitor switched off. Stopping the monitor does not reset it."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MISSMEN_A::B_0x0
    }
    #[doc = "cache miss monitor enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MISSMEN_A::B_0x1
    }
}
#[doc = "Field `MISSMEN` writer - miss monitor enable"]
pub type MISSMEN_W<'a, REG> = crate::BitWriter<'a, REG, MISSMEN_A>;
impl<'a, REG> MISSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cache miss monitor switched off. Stopping the monitor does not reset it."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MISSMEN_A::B_0x0)
    }
    #[doc = "cache miss monitor enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MISSMEN_A::B_0x1)
    }
}
#[doc = "hit monitor reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HITMRST_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: reset cache hit monitor"]
    B_0x1 = 1,
}
impl From<HITMRST_A> for bool {
    #[inline(always)]
    fn from(variant: HITMRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HITMRST` reader - hit monitor reset"]
pub type HITMRST_R = crate::BitReader<HITMRST_A>;
impl HITMRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HITMRST_A {
        match self.bits {
            false => HITMRST_A::B_0x0,
            true => HITMRST_A::B_0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HITMRST_A::B_0x0
    }
    #[doc = "reset cache hit monitor"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HITMRST_A::B_0x1
    }
}
#[doc = "Field `HITMRST` writer - hit monitor reset"]
pub type HITMRST_W<'a, REG> = crate::BitWriter<'a, REG, HITMRST_A>;
impl<'a, REG> HITMRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HITMRST_A::B_0x0)
    }
    #[doc = "reset cache hit monitor"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HITMRST_A::B_0x1)
    }
}
#[doc = "miss monitor reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSMRST_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: reset cache miss monitor"]
    B_0x1 = 1,
}
impl From<MISSMRST_A> for bool {
    #[inline(always)]
    fn from(variant: MISSMRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISSMRST` reader - miss monitor reset"]
pub type MISSMRST_R = crate::BitReader<MISSMRST_A>;
impl MISSMRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MISSMRST_A {
        match self.bits {
            false => MISSMRST_A::B_0x0,
            true => MISSMRST_A::B_0x1,
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MISSMRST_A::B_0x0
    }
    #[doc = "reset cache miss monitor"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MISSMRST_A::B_0x1
    }
}
#[doc = "Field `MISSMRST` writer - miss monitor reset"]
pub type MISSMRST_W<'a, REG> = crate::BitWriter<'a, REG, MISSMRST_A>;
impl<'a, REG> MISSMRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MISSMRST_A::B_0x0)
    }
    #[doc = "reset cache miss monitor"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MISSMRST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn EN(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
    #[inline(always)]
    pub fn WAYSEL(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - hit monitor enable"]
    #[inline(always)]
    pub fn HITMEN(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - miss monitor enable"]
    #[inline(always)]
    pub fn MISSMEN(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - hit monitor reset"]
    #[inline(always)]
    pub fn HITMRST(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - miss monitor reset"]
    #[inline(always)]
    pub fn MISSMRST(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn EN(&mut self) -> EN_W<'_, CR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
    #[inline(always)]
    pub fn CACHEINV(&mut self) -> CACHEINV_W<'_, CR_SPEC> {
        CACHEINV_W::new(self, 1)
    }
    #[doc = "Bit 2 - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
    #[inline(always)]
    pub fn WAYSEL(&mut self) -> WAYSEL_W<'_, CR_SPEC> {
        WAYSEL_W::new(self, 2)
    }
    #[doc = "Bit 16 - hit monitor enable"]
    #[inline(always)]
    pub fn HITMEN(&mut self) -> HITMEN_W<'_, CR_SPEC> {
        HITMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - miss monitor enable"]
    #[inline(always)]
    pub fn MISSMEN(&mut self) -> MISSMEN_W<'_, CR_SPEC> {
        MISSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - hit monitor reset"]
    #[inline(always)]
    pub fn HITMRST(&mut self) -> HITMRST_W<'_, CR_SPEC> {
        HITMRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - miss monitor reset"]
    #[inline(always)]
    pub fn MISSMRST(&mut self) -> MISSMRST_W<'_, CR_SPEC> {
        MISSMRST_W::new(self, 19)
    }
}
#[doc = "ICACHE control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0x04"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
