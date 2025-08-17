#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "busy flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYF_A {
    #[doc = "0: cache not busy on a CACHEINV operation"]
    B_0x0 = 0,
    #[doc = "1: cache executing a full invalidate CACHEINV operation"]
    B_0x1 = 1,
}
impl From<BUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSYF` reader - busy flag"]
pub type BUSYF_R = crate::BitReader<BUSYF_A>;
impl BUSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSYF_A {
        match self.bits {
            false => BUSYF_A::B_0x0,
            true => BUSYF_A::B_0x1,
        }
    }
    #[doc = "cache not busy on a CACHEINV operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BUSYF_A::B_0x0
    }
    #[doc = "cache executing a full invalidate CACHEINV operation"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BUSYF_A::B_0x1
    }
}
#[doc = "busy end flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYENDF_A {
    #[doc = "0: cache busy"]
    B_0x0 = 0,
    #[doc = "1: full invalidate CACHEINV operation finished"]
    B_0x1 = 1,
}
impl From<BSYENDF_A> for bool {
    #[inline(always)]
    fn from(variant: BSYENDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSYENDF` reader - busy end flag"]
pub type BSYENDF_R = crate::BitReader<BSYENDF_A>;
impl BSYENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSYENDF_A {
        match self.bits {
            false => BSYENDF_A::B_0x0,
            true => BSYENDF_A::B_0x1,
        }
    }
    #[doc = "cache busy"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BSYENDF_A::B_0x0
    }
    #[doc = "full invalidate CACHEINV operation finished"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BSYENDF_A::B_0x1
    }
}
#[doc = "cache error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRF_A {
    #[doc = "0: no error"]
    B_0x0 = 0,
    #[doc = "1: an error occurred during the operation (cacheable write)"]
    B_0x1 = 1,
}
impl From<ERRF_A> for bool {
    #[inline(always)]
    fn from(variant: ERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRF` reader - cache error flag"]
pub type ERRF_R = crate::BitReader<ERRF_A>;
impl ERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRF_A {
        match self.bits {
            false => ERRF_A::B_0x0,
            true => ERRF_A::B_0x1,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRF_A::B_0x0
    }
    #[doc = "an error occurred during the operation (cacheable write)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRF_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - busy flag"]
    #[inline(always)]
    pub fn BUSYF(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - busy end flag"]
    #[inline(always)]
    pub fn BSYENDF(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - cache error flag"]
    #[inline(always)]
    pub fn ERRF(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ICACHE status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
