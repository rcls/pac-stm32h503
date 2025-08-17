#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIER_SPEC>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIER_SPEC>;
#[doc = "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE_A {
    #[doc = "0: LSI ready interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LSI ready interrupt enabled"]
    B_0x1 = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::B_0x0,
            true => LSIRDYIE_A::B_0x1,
        }
    }
    #[doc = "LSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSIRDYIE_A::B_0x0
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSIRDYIE_A::B_0x1
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE_A>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE_A::B_0x0)
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE_A::B_0x1)
    }
}
#[doc = "LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE_A {
    #[doc = "0: LSE ready interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LSE ready interrupt enabled"]
    B_0x1 = 1,
}
impl From<LSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LSERDYIE_R = crate::BitReader<LSERDYIE_A>;
impl LSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYIE_A {
        match self.bits {
            false => LSERDYIE_A::B_0x0,
            true => LSERDYIE_A::B_0x1,
        }
    }
    #[doc = "LSE ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSERDYIE_A::B_0x0
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSERDYIE_A::B_0x1
    }
}
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYIE_A>;
impl<'a, REG> LSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE_A::B_0x0)
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYIE_A::B_0x1)
    }
}
#[doc = "CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIRDYIE_A {
    #[doc = "0: CSI ready interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSI ready interrupt enabled"]
    B_0x1 = 1,
}
impl From<CSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSIRDYIE` reader - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
pub type CSIRDYIE_R = crate::BitReader<CSIRDYIE_A>;
impl CSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSIRDYIE_A {
        match self.bits {
            false => CSIRDYIE_A::B_0x0,
            true => CSIRDYIE_A::B_0x1,
        }
    }
    #[doc = "CSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSIRDYIE_A::B_0x0
    }
    #[doc = "CSI ready interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSIRDYIE_A::B_0x1
    }
}
#[doc = "Field `CSIRDYIE` writer - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
pub type CSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, CSIRDYIE_A>;
impl<'a, REG> CSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSIRDYIE_A::B_0x0)
    }
    #[doc = "CSI ready interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSIRDYIE_A::B_0x1)
    }
}
#[doc = "HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE_A {
    #[doc = "0: HSI ready interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI ready interrupt enabled"]
    B_0x1 = 1,
}
impl From<HSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE_A>;
impl HSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYIE_A {
        match self.bits {
            false => HSIRDYIE_A::B_0x0,
            true => HSIRDYIE_A::B_0x1,
        }
    }
    #[doc = "HSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIRDYIE_A::B_0x0
    }
    #[doc = "HSI ready interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIRDYIE_A::B_0x1
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYIE_A>;
impl<'a, REG> HSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE_A::B_0x0)
    }
    #[doc = "HSI ready interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYIE_A::B_0x1)
    }
}
#[doc = "HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE_A {
    #[doc = "0: HSE ready interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSE ready interrupt enabled"]
    B_0x1 = 1,
}
impl From<HSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HSERDYIE_R = crate::BitReader<HSERDYIE_A>;
impl HSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYIE_A {
        match self.bits {
            false => HSERDYIE_A::B_0x0,
            true => HSERDYIE_A::B_0x1,
        }
    }
    #[doc = "HSE ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSERDYIE_A::B_0x0
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSERDYIE_A::B_0x1
    }
}
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYIE_A>;
impl<'a, REG> HSERDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE_A::B_0x0)
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYIE_A::B_0x1)
    }
}
#[doc = "HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYIE_A {
    #[doc = "0: HSI48 ready interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI48 ready interrupt enabled"]
    B_0x1 = 1,
}
impl From<HSI48RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
pub type HSI48RDYIE_R = crate::BitReader<HSI48RDYIE_A>;
impl HSI48RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYIE_A {
        match self.bits {
            false => HSI48RDYIE_A::B_0x0,
            true => HSI48RDYIE_A::B_0x1,
        }
    }
    #[doc = "HSI48 ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSI48RDYIE_A::B_0x0
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSI48RDYIE_A::B_0x1
    }
}
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, HSI48RDYIE_A>;
impl<'a, REG> HSI48RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYIE_A::B_0x0)
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYIE_A::B_0x1)
    }
}
#[doc = "PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1RDYIE_A {
    #[doc = "0: PLL1 lock interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL1 lock interrupt enabled"]
    B_0x1 = 1,
}
impl From<PLL1RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1RDYIE` reader - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
pub type PLL1RDYIE_R = crate::BitReader<PLL1RDYIE_A>;
impl PLL1RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RDYIE_A {
        match self.bits {
            false => PLL1RDYIE_A::B_0x0,
            true => PLL1RDYIE_A::B_0x1,
        }
    }
    #[doc = "PLL1 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1RDYIE_A::B_0x0
    }
    #[doc = "PLL1 lock interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1RDYIE_A::B_0x1
    }
}
#[doc = "Field `PLL1RDYIE` writer - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
pub type PLL1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLL1RDYIE_A>;
impl<'a, REG> PLL1RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL1 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RDYIE_A::B_0x0)
    }
    #[doc = "PLL1 lock interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RDYIE_A::B_0x1)
    }
}
#[doc = "PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2RDYIE_A {
    #[doc = "0: PLL2 lock interrupt disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL2 lock interrupt enabled"]
    B_0x1 = 1,
}
impl From<PLL2RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2RDYIE` reader - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
pub type PLL2RDYIE_R = crate::BitReader<PLL2RDYIE_A>;
impl PLL2RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RDYIE_A {
        match self.bits {
            false => PLL2RDYIE_A::B_0x0,
            true => PLL2RDYIE_A::B_0x1,
        }
    }
    #[doc = "PLL2 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2RDYIE_A::B_0x0
    }
    #[doc = "PLL2 lock interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2RDYIE_A::B_0x1
    }
}
#[doc = "Field `PLL2RDYIE` writer - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
pub type PLL2RDYIE_W<'a, REG> = crate::BitWriter<'a, REG, PLL2RDYIE_A>;
impl<'a, REG> PLL2RDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL2 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RDYIE_A::B_0x0)
    }
    #[doc = "PLL2 lock interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RDYIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn LSIRDYIE(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn LSERDYIE(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
    #[inline(always)]
    pub fn CSIRDYIE(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
    #[inline(always)]
    pub fn HSIRDYIE(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn HSERDYIE(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
    #[inline(always)]
    pub fn HSI48RDYIE(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
    #[inline(always)]
    pub fn PLL1RDYIE(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
    #[inline(always)]
    pub fn PLL2RDYIE(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn LSIRDYIE(&mut self) -> LSIRDYIE_W<'_, CIER_SPEC> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn LSERDYIE(&mut self) -> LSERDYIE_W<'_, CIER_SPEC> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
    #[inline(always)]
    pub fn CSIRDYIE(&mut self) -> CSIRDYIE_W<'_, CIER_SPEC> {
        CSIRDYIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
    #[inline(always)]
    pub fn HSIRDYIE(&mut self) -> HSIRDYIE_W<'_, CIER_SPEC> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn HSERDYIE(&mut self) -> HSERDYIE_W<'_, CIER_SPEC> {
        HSERDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
    #[inline(always)]
    pub fn HSI48RDYIE(&mut self) -> HSI48RDYIE_W<'_, CIER_SPEC> {
        HSI48RDYIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
    #[inline(always)]
    pub fn PLL1RDYIE(&mut self) -> PLL1RDYIE_W<'_, CIER_SPEC> {
        PLL1RDYIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
    #[inline(always)]
    pub fn PLL2RDYIE(&mut self) -> PLL2RDYIE_W<'_, CIER_SPEC> {
        PLL2RDYIE_W::new(self, 7)
    }
}
#[doc = "RCC clock source interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {}
