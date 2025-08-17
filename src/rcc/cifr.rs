#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFR_SPEC>;
#[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF_A {
    #[doc = "0: no clock ready interrupt caused by the LSI (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by the LSI"]
    B_0x1 = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
pub type LSIRDYF_R = crate::BitReader<LSIRDYF_A>;
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::B_0x0,
            true => LSIRDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by the LSI (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSIRDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by the LSI"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSIRDYF_A::B_0x1
    }
}
#[doc = "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF_A {
    #[doc = "0: no clock ready interrupt caused by the LSE (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by the LSE"]
    B_0x1 = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
pub type LSERDYF_R = crate::BitReader<LSERDYF_A>;
impl LSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::B_0x0,
            true => LSERDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by the LSE (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSERDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by the LSE"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSERDYF_A::B_0x1
    }
}
#[doc = "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIRDYF_A {
    #[doc = "0: no clock ready interrupt caused by the CSI (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by the CSI"]
    B_0x1 = 1,
}
impl From<CSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSIRDYF` reader - CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
pub type CSIRDYF_R = crate::BitReader<CSIRDYF_A>;
impl CSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSIRDYF_A {
        match self.bits {
            false => CSIRDYF_A::B_0x0,
            true => CSIRDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by the CSI (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSIRDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by the CSI"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSIRDYF_A::B_0x1
    }
}
#[doc = "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF_A {
    #[doc = "0: no clock ready interrupt caused by the HSI (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by the HSI"]
    B_0x1 = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
pub type HSIRDYF_R = crate::BitReader<HSIRDYF_A>;
impl HSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::B_0x0,
            true => HSIRDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by the HSI (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIRDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by the HSI"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIRDYF_A::B_0x1
    }
}
#[doc = "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF_A {
    #[doc = "0: no clock ready interrupt caused by the HSE (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by the HSE"]
    B_0x1 = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
pub type HSERDYF_R = crate::BitReader<HSERDYF_A>;
impl HSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::B_0x0,
            true => HSERDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by the HSE (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSERDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by the HSE"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSERDYF_A::B_0x1
    }
}
#[doc = "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYF_A {
    #[doc = "0: no clock ready interrupt caused by the HSI48 oscillator (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by the HSI48 oscillator"]
    B_0x1 = 1,
}
impl From<HSI48RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
pub type HSI48RDYF_R = crate::BitReader<HSI48RDYF_A>;
impl HSI48RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYF_A {
        match self.bits {
            false => HSI48RDYF_A::B_0x0,
            true => HSI48RDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by the HSI48 oscillator (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSI48RDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by the HSI48 oscillator"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSI48RDYF_A::B_0x1
    }
}
#[doc = "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1RDYF_A {
    #[doc = "0: no clock ready interrupt caused by PLL1 lock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by PLL1 lock"]
    B_0x1 = 1,
}
impl From<PLL1RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1RDYF` reader - PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
pub type PLL1RDYF_R = crate::BitReader<PLL1RDYF_A>;
impl PLL1RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RDYF_A {
        match self.bits {
            false => PLL1RDYF_A::B_0x0,
            true => PLL1RDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by PLL1 lock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1RDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by PLL1 lock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1RDYF_A::B_0x1
    }
}
#[doc = "PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2RDYF_A {
    #[doc = "0: no clock ready interrupt caused by PLL2 lock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock ready interrupt caused by PLL2 lock"]
    B_0x1 = 1,
}
impl From<PLL2RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2RDYF` reader - PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set."]
pub type PLL2RDYF_R = crate::BitReader<PLL2RDYF_A>;
impl PLL2RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RDYF_A {
        match self.bits {
            false => PLL2RDYF_A::B_0x0,
            true => PLL2RDYF_A::B_0x1,
        }
    }
    #[doc = "no clock ready interrupt caused by PLL2 lock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2RDYF_A::B_0x0
    }
    #[doc = "clock ready interrupt caused by PLL2 lock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2RDYF_A::B_0x1
    }
}
#[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSECSSF_A {
    #[doc = "0: no clock security interrupt caused by HSE clock failure (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: clock security interrupt caused by HSE clock failure"]
    B_0x1 = 1,
}
impl From<HSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: HSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSECSSF` reader - HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
pub type HSECSSF_R = crate::BitReader<HSECSSF_A>;
impl HSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSECSSF_A {
        match self.bits {
            false => HSECSSF_A::B_0x0,
            true => HSECSSF_A::B_0x1,
        }
    }
    #[doc = "no clock security interrupt caused by HSE clock failure (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSECSSF_A::B_0x0
    }
    #[doc = "clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSECSSF_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
    #[inline(always)]
    pub fn LSIRDYF(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
    #[inline(always)]
    pub fn LSERDYF(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
    #[inline(always)]
    pub fn CSIRDYF(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
    #[inline(always)]
    pub fn HSIRDYF(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
    #[inline(always)]
    pub fn HSERDYF(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
    #[inline(always)]
    pub fn HSI48RDYF(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
    #[inline(always)]
    pub fn PLL1RDYF(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set."]
    #[inline(always)]
    pub fn PLL2RDYF(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
    #[inline(always)]
    pub fn HSECSSF(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "RCC clock source interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CIFR_SPEC {}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {}
