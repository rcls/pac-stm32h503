#[doc = "Register `WUSCR` writer"]
pub type W = crate::W<WUSCR_SPEC>;
#[doc = "clear wakeup pin flag for WUFx These bits are always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    B_0x1 = 1,
}
impl From<CWUF1_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF1` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1_A>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1_A::B_0x0)
    }
    #[doc = "writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1_A::B_0x1)
    }
}
#[doc = "clear wakeup pin flag for WUFx These bits are always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF2_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    B_0x1 = 1,
}
impl From<CWUF2_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF2` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG, CWUF2_A>;
impl<'a, REG> CWUF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF2_A::B_0x0)
    }
    #[doc = "writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF2_A::B_0x1)
    }
}
#[doc = "clear wakeup pin flag for WUFx These bits are always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF3_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    B_0x1 = 1,
}
impl From<CWUF3_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF3` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG, CWUF3_A>;
impl<'a, REG> CWUF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF3_A::B_0x0)
    }
    #[doc = "writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF3_A::B_0x1)
    }
}
#[doc = "clear wakeup pin flag for WUFx These bits are always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF4_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    B_0x1 = 1,
}
impl From<CWUF4_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF4` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG, CWUF4_A>;
impl<'a, REG> CWUF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF4_A::B_0x0)
    }
    #[doc = "writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF4_A::B_0x1)
    }
}
#[doc = "clear wakeup pin flag for WUFx These bits are always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF5_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    B_0x1 = 1,
}
impl From<CWUF5_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF5` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF5_W<'a, REG> = crate::BitWriter<'a, REG, CWUF5_A>;
impl<'a, REG> CWUF5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF5_A::B_0x0)
    }
    #[doc = "writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF5_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 0 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    pub fn CWUF1(&mut self) -> CWUF1_W<'_, WUSCR_SPEC> {
        CWUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    pub fn CWUF2(&mut self) -> CWUF2_W<'_, WUSCR_SPEC> {
        CWUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    pub fn CWUF3(&mut self) -> CWUF3_W<'_, WUSCR_SPEC> {
        CWUF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    pub fn CWUF4(&mut self) -> CWUF4_W<'_, WUSCR_SPEC> {
        CWUF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    pub fn CWUF5(&mut self) -> CWUF5_W<'_, WUSCR_SPEC> {
        CWUF5_W::new(self, 4)
    }
}
#[doc = "PWR wakeup status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUSCR_SPEC;
impl crate::RegisterSpec for WUSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wuscr::W`](W) writer structure"]
impl crate::Writable for WUSCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WUSCR to value 0"]
impl crate::Resettable for WUSCR_SPEC {}
