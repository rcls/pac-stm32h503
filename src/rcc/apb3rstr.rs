#[doc = "Register `APB3RSTR` reader"]
pub type R = crate::R<APB3RSTR_SPEC>;
#[doc = "Register `APB3RSTR` writer"]
pub type W = crate::W<APB3RSTR_SPEC>;
#[doc = "SBS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSRST_A {
    #[doc = "0: does not reset the SBS block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the SBS block"]
    B_0x1 = 1,
}
impl From<SBSRST_A> for bool {
    #[inline(always)]
    fn from(variant: SBSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSRST` reader - SBS block reset Set and reset by software."]
pub type SBSRST_R = crate::BitReader<SBSRST_A>;
impl SBSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBSRST_A {
        match self.bits {
            false => SBSRST_A::B_0x0,
            true => SBSRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the SBS block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBSRST_A::B_0x0
    }
    #[doc = "resets the SBS block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBSRST_A::B_0x1
    }
}
#[doc = "Field `SBSRST` writer - SBS block reset Set and reset by software."]
pub type SBSRST_W<'a, REG> = crate::BitWriter<'a, REG, SBSRST_A>;
impl<'a, REG> SBSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the SBS block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBSRST_A::B_0x0)
    }
    #[doc = "resets the SBS block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBSRST_A::B_0x1)
    }
}
#[doc = "LPUART1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST_A {
    #[doc = "0: does not reset the LPUART1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the LPUART1 block"]
    B_0x1 = 1,
}
impl From<LPUART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_R = crate::BitReader<LPUART1RST_A>;
impl LPUART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1RST_A {
        match self.bits {
            false => LPUART1RST_A::B_0x0,
            true => LPUART1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the LPUART1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPUART1RST_A::B_0x0
    }
    #[doc = "resets the LPUART1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPUART1RST_A::B_0x1
    }
}
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1RST_A>;
impl<'a, REG> LPUART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the LPUART1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST_A::B_0x0)
    }
    #[doc = "resets the LPUART1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST_A::B_0x1)
    }
}
#[doc = "I3C2RST block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C2RST_A {
    #[doc = "0: does not reset the I3C2RST block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the I3C2RST block"]
    B_0x1 = 1,
}
impl From<I3C2RST_A> for bool {
    #[inline(always)]
    fn from(variant: I3C2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I3C2RST` reader - I3C2RST block reset Set and reset by software."]
pub type I3C2RST_R = crate::BitReader<I3C2RST_A>;
impl I3C2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C2RST_A {
        match self.bits {
            false => I3C2RST_A::B_0x0,
            true => I3C2RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the I3C2RST block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C2RST_A::B_0x0
    }
    #[doc = "resets the I3C2RST block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C2RST_A::B_0x1
    }
}
#[doc = "Field `I3C2RST` writer - I3C2RST block reset Set and reset by software."]
pub type I3C2RST_W<'a, REG> = crate::BitWriter<'a, REG, I3C2RST_A>;
impl<'a, REG> I3C2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the I3C2RST block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2RST_A::B_0x0)
    }
    #[doc = "resets the I3C2RST block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2RST_A::B_0x1)
    }
}
#[doc = "LPTIM1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1RST_A {
    #[doc = "0: does not reset the LPTIM1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the LPTIM1 block"]
    B_0x1 = 1,
}
impl From<LPTIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1RST` reader - LPTIM1 block reset Set and reset by software."]
pub type LPTIM1RST_R = crate::BitReader<LPTIM1RST_A>;
impl LPTIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1RST_A {
        match self.bits {
            false => LPTIM1RST_A::B_0x0,
            true => LPTIM1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the LPTIM1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM1RST_A::B_0x0
    }
    #[doc = "resets the LPTIM1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM1RST_A::B_0x1
    }
}
#[doc = "Field `LPTIM1RST` writer - LPTIM1 block reset Set and reset by software."]
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1RST_A>;
impl<'a, REG> LPTIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the LPTIM1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1RST_A::B_0x0)
    }
    #[doc = "resets the LPTIM1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1RST_A::B_0x1)
    }
}
#[doc = "VREF block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFRST_A {
    #[doc = "0: does not reset the VREF block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the VREF block"]
    B_0x1 = 1,
}
impl From<VREFRST_A> for bool {
    #[inline(always)]
    fn from(variant: VREFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFRST` reader - VREF block reset Set and reset by software."]
pub type VREFRST_R = crate::BitReader<VREFRST_A>;
impl VREFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFRST_A {
        match self.bits {
            false => VREFRST_A::B_0x0,
            true => VREFRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the VREF block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VREFRST_A::B_0x0
    }
    #[doc = "resets the VREF block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VREFRST_A::B_0x1
    }
}
#[doc = "Field `VREFRST` writer - VREF block reset Set and reset by software."]
pub type VREFRST_W<'a, REG> = crate::BitWriter<'a, REG, VREFRST_A>;
impl<'a, REG> VREFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the VREF block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFRST_A::B_0x0)
    }
    #[doc = "resets the VREF block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFRST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    pub fn SBSRST(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn LPUART1RST(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2RST block reset Set and reset by software."]
    #[inline(always)]
    pub fn I3C2RST(&self) -> I3C2RST_R {
        I3C2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM1RST(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn VREFRST(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    pub fn SBSRST(&mut self) -> SBSRST_W<'_, APB3RSTR_SPEC> {
        SBSRST_W::new(self, 1)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn LPUART1RST(&mut self) -> LPUART1RST_W<'_, APB3RSTR_SPEC> {
        LPUART1RST_W::new(self, 6)
    }
    #[doc = "Bit 9 - I3C2RST block reset Set and reset by software."]
    #[inline(always)]
    pub fn I3C2RST(&mut self) -> I3C2RST_W<'_, APB3RSTR_SPEC> {
        I3C2RST_W::new(self, 9)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM1RST(&mut self) -> LPTIM1RST_W<'_, APB3RSTR_SPEC> {
        LPTIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn VREFRST(&mut self) -> VREFRST_W<'_, APB3RSTR_SPEC> {
        VREFRST_W::new(self, 20)
    }
}
#[doc = "RCC APB3 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3RSTR_SPEC;
impl crate::RegisterSpec for APB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3rstr::R`](R) reader structure"]
impl crate::Readable for APB3RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure"]
impl crate::Writable for APB3RSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTR_SPEC {}
