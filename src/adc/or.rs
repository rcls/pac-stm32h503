#[doc = "Register `OR` reader"]
pub type R = crate::R<OR_SPEC>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OR_SPEC>;
#[doc = "Option bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OP0_A {
    #[doc = "0: VDDCORE channel disabled"]
    B_0x0 = 0,
    #[doc = "1: VDDCORE channel enabled"]
    B_0x1 = 1,
}
impl From<OP0_A> for bool {
    #[inline(always)]
    fn from(variant: OP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OP0` reader - Option bit 0"]
pub type OP0_R = crate::BitReader<OP0_A>;
impl OP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OP0_A {
        match self.bits {
            false => OP0_A::B_0x0,
            true => OP0_A::B_0x1,
        }
    }
    #[doc = "VDDCORE channel disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OP0_A::B_0x0
    }
    #[doc = "VDDCORE channel enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OP0_A::B_0x1
    }
}
#[doc = "Field `OP0` writer - Option bit 0"]
pub type OP0_W<'a, REG> = crate::BitWriter<'a, REG, OP0_A>;
impl<'a, REG> OP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDDCORE channel disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OP0_A::B_0x0)
    }
    #[doc = "VDDCORE channel enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OP0_A::B_0x1)
    }
}
#[doc = "Option bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OP1_A {
    #[doc = "0: INP0 GPIO switch control disabled"]
    B_0x0 = 0,
    #[doc = "1: INP0 GPIO switch control enabled"]
    B_0x1 = 1,
}
impl From<OP1_A> for bool {
    #[inline(always)]
    fn from(variant: OP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OP1` reader - Option bit 1"]
pub type OP1_R = crate::BitReader<OP1_A>;
impl OP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OP1_A {
        match self.bits {
            false => OP1_A::B_0x0,
            true => OP1_A::B_0x1,
        }
    }
    #[doc = "INP0 GPIO switch control disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OP1_A::B_0x0
    }
    #[doc = "INP0 GPIO switch control enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OP1_A::B_0x1
    }
}
#[doc = "Field `OP1` writer - Option bit 1"]
pub type OP1_W<'a, REG> = crate::BitWriter<'a, REG, OP1_A>;
impl<'a, REG> OP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INP0 GPIO switch control disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OP1_A::B_0x0)
    }
    #[doc = "INP0 GPIO switch control enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OP1_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Option bit 0"]
    #[inline(always)]
    pub fn OP0(&self) -> OP0_R {
        OP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option bit 1"]
    #[inline(always)]
    pub fn OP1(&self) -> OP1_R {
        OP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option bit 0"]
    #[inline(always)]
    pub fn OP0(&mut self) -> OP0_W<'_, OR_SPEC> {
        OP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Option bit 1"]
    #[inline(always)]
    pub fn OP1(&mut self) -> OP1_W<'_, OR_SPEC> {
        OP1_W::new(self, 1)
    }
}
#[doc = "ADC option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {}
