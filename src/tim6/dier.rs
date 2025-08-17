#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIER_SPEC>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIER_SPEC>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE_A {
    #[doc = "0: Update interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Update interrupt enabled."]
    B_0x1 = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader<UIE_A>;
impl UIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::B_0x0,
            true => UIE_A::B_0x1,
        }
    }
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UIE_A::B_0x0
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UIE_A::B_0x1
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE_A>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIE_A::B_0x0)
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIE_A::B_0x1)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE_A {
    #[doc = "0: Update DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: Update DMA request enabled."]
    B_0x1 = 1,
}
impl From<UDE_A> for bool {
    #[inline(always)]
    fn from(variant: UDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader<UDE_A>;
impl UDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDE_A {
        match self.bits {
            false => UDE_A::B_0x0,
            true => UDE_A::B_0x1,
        }
    }
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UDE_A::B_0x0
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UDE_A::B_0x1
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE_A>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDE_A::B_0x0)
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UDE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn UIE(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn UDE(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn UIE(&mut self) -> UIE_W<'_, DIER_SPEC> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn UDE(&mut self) -> UDE_W<'_, DIER_SPEC> {
        UDE_W::new(self, 8)
    }
}
#[doc = "TIM6 DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DIER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIER_SPEC {}
