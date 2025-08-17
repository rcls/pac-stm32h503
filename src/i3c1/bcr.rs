#[doc = "Register `BCR` reader"]
pub type R = crate::R<BCR_SPEC>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BCR_SPEC>;
#[doc = "max data speed limitation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCR0_A {
    #[doc = "0: no limitation"]
    B_0x0 = 0,
    #[doc = "1: limitation, as described by I3C_GETMXDSR."]
    B_0x1 = 1,
}
impl From<BCR0_A> for bool {
    #[inline(always)]
    fn from(variant: BCR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCR0` reader - max data speed limitation"]
pub type BCR0_R = crate::BitReader<BCR0_A>;
impl BCR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCR0_A {
        match self.bits {
            false => BCR0_A::B_0x0,
            true => BCR0_A::B_0x1,
        }
    }
    #[doc = "no limitation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BCR0_A::B_0x0
    }
    #[doc = "limitation, as described by I3C_GETMXDSR."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BCR0_A::B_0x1
    }
}
#[doc = "Field `BCR0` writer - max data speed limitation"]
pub type BCR0_W<'a, REG> = crate::BitWriter<'a, REG, BCR0_A>;
impl<'a, REG> BCR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no limitation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BCR0_A::B_0x0)
    }
    #[doc = "limitation, as described by I3C_GETMXDSR."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BCR0_A::B_0x1)
    }
}
#[doc = "in-band interrupt (IBI) payload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCR2_A {
    #[doc = "0: no data byte follows the accepted IBI"]
    B_0x0 = 0,
    #[doc = "1: at least one mandatory data byte follows the accepted IBI (and at most 4 data bytes)"]
    B_0x1 = 1,
}
impl From<BCR2_A> for bool {
    #[inline(always)]
    fn from(variant: BCR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCR2` reader - in-band interrupt (IBI) payload"]
pub type BCR2_R = crate::BitReader<BCR2_A>;
impl BCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCR2_A {
        match self.bits {
            false => BCR2_A::B_0x0,
            true => BCR2_A::B_0x1,
        }
    }
    #[doc = "no data byte follows the accepted IBI"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BCR2_A::B_0x0
    }
    #[doc = "at least one mandatory data byte follows the accepted IBI (and at most 4 data bytes)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BCR2_A::B_0x1
    }
}
#[doc = "Field `BCR2` writer - in-band interrupt (IBI) payload"]
pub type BCR2_W<'a, REG> = crate::BitWriter<'a, REG, BCR2_A>;
impl<'a, REG> BCR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no data byte follows the accepted IBI"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BCR2_A::B_0x0)
    }
    #[doc = "at least one mandatory data byte follows the accepted IBI (and at most 4 data bytes)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BCR2_A::B_0x1)
    }
}
#[doc = "controller capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCR6_A {
    #[doc = "0: I3C target (no controller capable)"]
    B_0x0 = 0,
    #[doc = "1: I3C controller capable"]
    B_0x1 = 1,
}
impl From<BCR6_A> for bool {
    #[inline(always)]
    fn from(variant: BCR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCR6` reader - controller capable"]
pub type BCR6_R = crate::BitReader<BCR6_A>;
impl BCR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCR6_A {
        match self.bits {
            false => BCR6_A::B_0x0,
            true => BCR6_A::B_0x1,
        }
    }
    #[doc = "I3C target (no controller capable)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BCR6_A::B_0x0
    }
    #[doc = "I3C controller capable"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BCR6_A::B_0x1
    }
}
#[doc = "Field `BCR6` writer - controller capable"]
pub type BCR6_W<'a, REG> = crate::BitWriter<'a, REG, BCR6_A>;
impl<'a, REG> BCR6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C target (no controller capable)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BCR6_A::B_0x0)
    }
    #[doc = "I3C controller capable"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BCR6_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - max data speed limitation"]
    #[inline(always)]
    pub fn BCR0(&self) -> BCR0_R {
        BCR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - in-band interrupt (IBI) payload"]
    #[inline(always)]
    pub fn BCR2(&self) -> BCR2_R {
        BCR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - controller capable"]
    #[inline(always)]
    pub fn BCR6(&self) -> BCR6_R {
        BCR6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - max data speed limitation"]
    #[inline(always)]
    pub fn BCR0(&mut self) -> BCR0_W<'_, BCR_SPEC> {
        BCR0_W::new(self, 0)
    }
    #[doc = "Bit 2 - in-band interrupt (IBI) payload"]
    #[inline(always)]
    pub fn BCR2(&mut self) -> BCR2_W<'_, BCR_SPEC> {
        BCR2_W::new(self, 2)
    }
    #[doc = "Bit 6 - controller capable"]
    #[inline(always)]
    pub fn BCR6(&mut self) -> BCR6_W<'_, BCR_SPEC> {
        BCR6_W::new(self, 6)
    }
}
#[doc = "I3C bus characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {}
