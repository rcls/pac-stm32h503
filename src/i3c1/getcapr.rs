#[doc = "Register `GETCAPR` reader"]
pub type R = crate::R<GETCAPR_SPEC>;
#[doc = "Register `GETCAPR` writer"]
pub type W = crate::W<GETCAPR_SPEC>;
#[doc = "IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\] value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPPEND_A {
    #[doc = "0: this I3C when acting as target sends an IBI request without a mandatory data byte value indicating a pending read notification"]
    B_0x0 = 0,
    #[doc = "1: this I3C when acting as target sends an IBI request with a mandatory data byte value (i.e. a MDB\\[7:5\\]=101) indicating a pending read notification"]
    B_0x1 = 1,
}
impl From<CAPPEND_A> for bool {
    #[inline(always)]
    fn from(variant: CAPPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPPEND` reader - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\] value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
pub type CAPPEND_R = crate::BitReader<CAPPEND_A>;
impl CAPPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAPPEND_A {
        match self.bits {
            false => CAPPEND_A::B_0x0,
            true => CAPPEND_A::B_0x1,
        }
    }
    #[doc = "this I3C when acting as target sends an IBI request without a mandatory data byte value indicating a pending read notification"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CAPPEND_A::B_0x0
    }
    #[doc = "this I3C when acting as target sends an IBI request with a mandatory data byte value (i.e. a MDB\\[7:5\\]=101) indicating a pending read notification"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CAPPEND_A::B_0x1
    }
}
#[doc = "Field `CAPPEND` writer - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\] value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
pub type CAPPEND_W<'a, REG> = crate::BitWriter<'a, REG, CAPPEND_A>;
impl<'a, REG> CAPPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "this I3C when acting as target sends an IBI request without a mandatory data byte value indicating a pending read notification"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CAPPEND_A::B_0x0)
    }
    #[doc = "this I3C when acting as target sends an IBI request with a mandatory data byte value (i.e. a MDB\\[7:5\\]=101) indicating a pending read notification"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CAPPEND_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\] value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
    #[inline(always)]
    pub fn CAPPEND(&self) -> CAPPEND_R {
        CAPPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\] value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
    #[inline(always)]
    pub fn CAPPEND(&mut self) -> CAPPEND_W<'_, GETCAPR_SPEC> {
        CAPPEND_W::new(self, 14)
    }
}
#[doc = "I3C get capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`getcapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getcapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GETCAPR_SPEC;
impl crate::RegisterSpec for GETCAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`getcapr::R`](R) reader structure"]
impl crate::Readable for GETCAPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`getcapr::W`](W) writer structure"]
impl crate::Writable for GETCAPR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GETCAPR to value 0"]
impl crate::Resettable for GETCAPR_SPEC {}
