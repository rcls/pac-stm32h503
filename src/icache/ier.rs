#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYENDIE_A {
    #[doc = "0: interrupt disabled on busy end"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled on busy end"]
    B_0x1 = 1,
}
impl From<BSYENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: BSYENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSYENDIE` reader - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
pub type BSYENDIE_R = crate::BitReader<BSYENDIE_A>;
impl BSYENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSYENDIE_A {
        match self.bits {
            false => BSYENDIE_A::B_0x0,
            true => BSYENDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled on busy end"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BSYENDIE_A::B_0x0
    }
    #[doc = "interrupt enabled on busy end"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BSYENDIE_A::B_0x1
    }
}
#[doc = "Field `BSYENDIE` writer - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG, BSYENDIE_A>;
impl<'a, REG> BSYENDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled on busy end"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BSYENDIE_A::B_0x0)
    }
    #[doc = "interrupt enabled on busy end"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BSYENDIE_A::B_0x1)
    }
}
#[doc = "interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: interrupt disabled on error"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled on error"]
    B_0x1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0x0,
            true => ERRIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled on error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRIE_A::B_0x0
    }
    #[doc = "interrupt enabled on error"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRIE_A::B_0x1
    }
}
#[doc = "Field `ERRIE` writer - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE_A>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled on error"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0x0)
    }
    #[doc = "interrupt enabled on error"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 1 - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
    #[inline(always)]
    pub fn BSYENDIE(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
    #[inline(always)]
    pub fn ERRIE(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
    #[inline(always)]
    pub fn BSYENDIE(&mut self) -> BSYENDIE_W<'_, IER_SPEC> {
        BSYENDIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
    #[inline(always)]
    pub fn ERRIE(&mut self) -> ERRIE_W<'_, IER_SPEC> {
        ERRIE_W::new(self, 2)
    }
}
#[doc = "ICACHE interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {}
