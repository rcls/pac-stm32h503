#[doc = "Register `DTR2` reader"]
pub type R = crate::R<DTR2_SPEC>;
#[doc = "Register `DTR2` writer"]
pub type W = crate::W<DTR2_SPEC>;
#[doc = "Field `DTGF` reader - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx = DTF=DTGF\\[7:0\\]x t dtg with t dtg =t DTS . DTGF\\[7:5\\]=10x = DTF=(64+DTGF\\[5:0\\])xt dtg with T dtg =2xt DTS . DTGF\\[7:5\\]=110 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =8xt DTS . DTGF\\[7:5\\]=111 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTGF_R = crate::FieldReader;
#[doc = "Field `DTGF` writer - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx = DTF=DTGF\\[7:0\\]x t dtg with t dtg =t DTS . DTGF\\[7:5\\]=10x = DTF=(64+DTGF\\[5:0\\])xt dtg with T dtg =2xt DTS . DTGF\\[7:5\\]=110 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =8xt DTS . DTGF\\[7:5\\]=111 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTGF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTAE_A {
    #[doc = "0: Deadtime on rising and falling edges are identical, and defined with DTG\\[7:0\\] register"]
    B_0x0 = 0,
    #[doc = "1: Deadtime on rising edge is defined with DTG\\[7:0\\] register and deadtime on falling edge is defined with DTGF\\[7:0\\] bits."]
    B_0x1 = 1,
}
impl From<DTAE_A> for bool {
    #[inline(always)]
    fn from(variant: DTAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTAE` reader - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTAE_R = crate::BitReader<DTAE_A>;
impl DTAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTAE_A {
        match self.bits {
            false => DTAE_A::B_0x0,
            true => DTAE_A::B_0x1,
        }
    }
    #[doc = "Deadtime on rising and falling edges are identical, and defined with DTG\\[7:0\\] register"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTAE_A::B_0x0
    }
    #[doc = "Deadtime on rising edge is defined with DTG\\[7:0\\] register and deadtime on falling edge is defined with DTGF\\[7:0\\] bits."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTAE_A::B_0x1
    }
}
#[doc = "Field `DTAE` writer - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTAE_W<'a, REG> = crate::BitWriter<'a, REG, DTAE_A>;
impl<'a, REG> DTAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deadtime on rising and falling edges are identical, and defined with DTG\\[7:0\\] register"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTAE_A::B_0x0)
    }
    #[doc = "Deadtime on rising edge is defined with DTG\\[7:0\\] register and deadtime on falling edge is defined with DTGF\\[7:0\\] bits."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTAE_A::B_0x1)
    }
}
#[doc = "Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTPE_A {
    #[doc = "0: Deadtime value is not preloaded"]
    B_0x0 = 0,
    #[doc = "1: Deadtime value preload is enabled"]
    B_0x1 = 1,
}
impl From<DTPE_A> for bool {
    #[inline(always)]
    fn from(variant: DTPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTPE` reader - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTPE_R = crate::BitReader<DTPE_A>;
impl DTPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTPE_A {
        match self.bits {
            false => DTPE_A::B_0x0,
            true => DTPE_A::B_0x1,
        }
    }
    #[doc = "Deadtime value is not preloaded"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTPE_A::B_0x0
    }
    #[doc = "Deadtime value preload is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTPE_A::B_0x1
    }
}
#[doc = "Field `DTPE` writer - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTPE_W<'a, REG> = crate::BitWriter<'a, REG, DTPE_A>;
impl<'a, REG> DTPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deadtime value is not preloaded"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTPE_A::B_0x0)
    }
    #[doc = "Deadtime value preload is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx = DTF=DTGF\\[7:0\\]x t dtg with t dtg =t DTS . DTGF\\[7:5\\]=10x = DTF=(64+DTGF\\[5:0\\])xt dtg with T dtg =2xt DTS . DTGF\\[7:5\\]=110 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =8xt DTS . DTGF\\[7:5\\]=111 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTGF(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTAE(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTPE(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\\[7:5\\]=0xx = DTF=DTGF\\[7:0\\]x t dtg with t dtg =t DTS . DTGF\\[7:5\\]=10x = DTF=(64+DTGF\\[5:0\\])xt dtg with T dtg =2xt DTS . DTGF\\[7:5\\]=110 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =8xt DTS . DTGF\\[7:5\\]=111 = DTF=(32+DTGF\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTGF(&mut self) -> DTGF_W<'_, DTR2_SPEC> {
        DTGF_W::new(self, 0)
    }
    #[doc = "Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTAE(&mut self) -> DTAE_W<'_, DTR2_SPEC> {
        DTAE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTPE(&mut self) -> DTPE_W<'_, DTR2_SPEC> {
        DTPE_W::new(self, 17)
    }
}
#[doc = "TIM1 timer deadtime register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTR2_SPEC;
impl crate::RegisterSpec for DTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtr2::R`](R) reader structure"]
impl crate::Readable for DTR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtr2::W`](W) writer structure"]
impl crate::Writable for DTR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DTR2 to value 0"]
impl crate::Resettable for DTR2_SPEC {}
