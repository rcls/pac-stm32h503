#[doc = "Register `APB1HRSTR` reader"]
pub type R = crate::R<APB1HRSTR_SPEC>;
#[doc = "Register `APB1HRSTR` writer"]
pub type W = crate::W<APB1HRSTR_SPEC>;
#[doc = "DTS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSRST_A {
    #[doc = "0: does not reset the DTS block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the DTS block"]
    B_0x1 = 1,
}
impl From<DTSRST_A> for bool {
    #[inline(always)]
    fn from(variant: DTSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSRST` reader - DTS block reset Set and reset by software."]
pub type DTSRST_R = crate::BitReader<DTSRST_A>;
impl DTSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTSRST_A {
        match self.bits {
            false => DTSRST_A::B_0x0,
            true => DTSRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the DTS block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTSRST_A::B_0x0
    }
    #[doc = "resets the DTS block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTSRST_A::B_0x1
    }
}
#[doc = "Field `DTSRST` writer - DTS block reset Set and reset by software."]
pub type DTSRST_W<'a, REG> = crate::BitWriter<'a, REG, DTSRST_A>;
impl<'a, REG> DTSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the DTS block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTSRST_A::B_0x0)
    }
    #[doc = "resets the DTS block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTSRST_A::B_0x1)
    }
}
#[doc = "LPTIM2 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2RST_A {
    #[doc = "0: does not reset the LPTIM2 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the LPTIM2 block"]
    B_0x1 = 1,
}
impl From<LPTIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software."]
pub type LPTIM2RST_R = crate::BitReader<LPTIM2RST_A>;
impl LPTIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2RST_A {
        match self.bits {
            false => LPTIM2RST_A::B_0x0,
            true => LPTIM2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the LPTIM2 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM2RST_A::B_0x0
    }
    #[doc = "resets the LPTIM2 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM2RST_A::B_0x1
    }
}
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software."]
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2RST_A>;
impl<'a, REG> LPTIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the LPTIM2 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2RST_A::B_0x0)
    }
    #[doc = "resets the LPTIM2 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2RST_A::B_0x1)
    }
}
#[doc = "FDCAN1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCAN1RST_A {
    #[doc = "0: does not reset the FDCAN1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the FDCAN1 block"]
    B_0x1 = 1,
}
impl From<FDCAN1RST_A> for bool {
    #[inline(always)]
    fn from(variant: FDCAN1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCAN1RST` reader - FDCAN1 block reset Set and reset by software."]
pub type FDCAN1RST_R = crate::BitReader<FDCAN1RST_A>;
impl FDCAN1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDCAN1RST_A {
        match self.bits {
            false => FDCAN1RST_A::B_0x0,
            true => FDCAN1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the FDCAN1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FDCAN1RST_A::B_0x0
    }
    #[doc = "resets the FDCAN1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FDCAN1RST_A::B_0x1
    }
}
#[doc = "Field `FDCAN1RST` writer - FDCAN1 block reset Set and reset by software."]
pub type FDCAN1RST_W<'a, REG> = crate::BitWriter<'a, REG, FDCAN1RST_A>;
impl<'a, REG> FDCAN1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the FDCAN1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1RST_A::B_0x0)
    }
    #[doc = "resets the FDCAN1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1RST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 3 - DTS block reset Set and reset by software."]
    #[inline(always)]
    pub fn DTSRST(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM2RST(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn FDCAN1RST(&self) -> FDCAN1RST_R {
        FDCAN1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DTS block reset Set and reset by software."]
    #[inline(always)]
    pub fn DTSRST(&mut self) -> DTSRST_W<'_, APB1HRSTR_SPEC> {
        DTSRST_W::new(self, 3)
    }
    #[doc = "Bit 5 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM2RST(&mut self) -> LPTIM2RST_W<'_, APB1HRSTR_SPEC> {
        LPTIM2RST_W::new(self, 5)
    }
    #[doc = "Bit 9 - FDCAN1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn FDCAN1RST(&mut self) -> FDCAN1RST_W<'_, APB1HRSTR_SPEC> {
        FDCAN1RST_W::new(self, 9)
    }
}
#[doc = "RCC APB1 peripheral high reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HRSTR_SPEC;
impl crate::RegisterSpec for APB1HRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hrstr::R`](R) reader structure"]
impl crate::Readable for APB1HRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure"]
impl crate::Writable for APB1HRSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for APB1HRSTR_SPEC {}
