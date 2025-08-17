#[doc = "Register `CRCAPR` reader"]
pub type R = crate::R<CRCAPR_SPEC>;
#[doc = "Register `CRCAPR` writer"]
pub type W = crate::W<CRCAPR_SPEC>;
#[doc = "delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPDHOFF_A {
    #[doc = "0: this I3C does not needs additional time to process a controller-role hand-off"]
    B_0x0 = 0,
    #[doc = "1: this I3C needs additional time to process a controller-role hand-off"]
    B_0x1 = 1,
}
impl From<CAPDHOFF_A> for bool {
    #[inline(always)]
    fn from(variant: CAPDHOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPDHOFF` reader - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
pub type CAPDHOFF_R = crate::BitReader<CAPDHOFF_A>;
impl CAPDHOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAPDHOFF_A {
        match self.bits {
            false => CAPDHOFF_A::B_0x0,
            true => CAPDHOFF_A::B_0x1,
        }
    }
    #[doc = "this I3C does not needs additional time to process a controller-role hand-off"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CAPDHOFF_A::B_0x0
    }
    #[doc = "this I3C needs additional time to process a controller-role hand-off"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CAPDHOFF_A::B_0x1
    }
}
#[doc = "Field `CAPDHOFF` writer - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
pub type CAPDHOFF_W<'a, REG> = crate::BitWriter<'a, REG, CAPDHOFF_A>;
impl<'a, REG> CAPDHOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "this I3C does not needs additional time to process a controller-role hand-off"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CAPDHOFF_A::B_0x0)
    }
    #[doc = "this I3C needs additional time to process a controller-role hand-off"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CAPDHOFF_A::B_0x1)
    }
}
#[doc = "group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPGRP_A {
    #[doc = "0: this I3C does not support group address capabilities"]
    B_0x0 = 0,
    #[doc = "1: this I3C supports group address capabilities (when becoming controller)"]
    B_0x1 = 1,
}
impl From<CAPGRP_A> for bool {
    #[inline(always)]
    fn from(variant: CAPGRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPGRP` reader - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
pub type CAPGRP_R = crate::BitReader<CAPGRP_A>;
impl CAPGRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAPGRP_A {
        match self.bits {
            false => CAPGRP_A::B_0x0,
            true => CAPGRP_A::B_0x1,
        }
    }
    #[doc = "this I3C does not support group address capabilities"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CAPGRP_A::B_0x0
    }
    #[doc = "this I3C supports group address capabilities (when becoming controller)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CAPGRP_A::B_0x1
    }
}
#[doc = "Field `CAPGRP` writer - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
pub type CAPGRP_W<'a, REG> = crate::BitWriter<'a, REG, CAPGRP_A>;
impl<'a, REG> CAPGRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "this I3C does not support group address capabilities"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CAPGRP_A::B_0x0)
    }
    #[doc = "this I3C supports group address capabilities (when becoming controller)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CAPGRP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    pub fn CAPDHOFF(&self) -> CAPDHOFF_R {
        CAPDHOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    pub fn CAPGRP(&self) -> CAPGRP_R {
        CAPGRP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    pub fn CAPDHOFF(&mut self) -> CAPDHOFF_W<'_, CRCAPR_SPEC> {
        CAPDHOFF_W::new(self, 3)
    }
    #[doc = "Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
    #[inline(always)]
    pub fn CAPGRP(&mut self) -> CAPGRP_W<'_, CRCAPR_SPEC> {
        CAPGRP_W::new(self, 9)
    }
}
#[doc = "I3C controller-role capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCAPR_SPEC;
impl crate::RegisterSpec for CRCAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcapr::R`](R) reader structure"]
impl crate::Readable for CRCAPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcapr::W`](W) writer structure"]
impl crate::Writable for CRCAPR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CRCAPR to value 0"]
impl crate::Resettable for CRCAPR_SPEC {}
