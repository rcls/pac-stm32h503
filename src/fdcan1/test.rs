#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCK_A {
    #[doc = "0: Reset value, Loop Back mode is disabled"]
    B_0x0 = 0,
    #[doc = "1: Loop Back mode is enabled (see Power down (Sleep mode))"]
    B_0x1 = 1,
}
impl From<LBCK_A> for bool {
    #[inline(always)]
    fn from(variant: LBCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCK` reader - Loop back mode"]
pub type LBCK_R = crate::BitReader<LBCK_A>;
impl LBCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBCK_A {
        match self.bits {
            false => LBCK_A::B_0x0,
            true => LBCK_A::B_0x1,
        }
    }
    #[doc = "Reset value, Loop Back mode is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LBCK_A::B_0x0
    }
    #[doc = "Loop Back mode is enabled (see Power down (Sleep mode))"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LBCK_A::B_0x1
    }
}
#[doc = "Field `LBCK` writer - Loop back mode"]
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG, LBCK_A>;
impl<'a, REG> LBCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset value, Loop Back mode is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LBCK_A::B_0x0)
    }
    #[doc = "Loop Back mode is enabled (see Power down (Sleep mode))"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LBCK_A::B_0x1)
    }
}
#[doc = "Control of transmit pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    B_0x0 = 0,
    #[doc = "1: Sample point can be monitored at pin FDCANx_TX"]
    B_0x1 = 1,
    #[doc = "2: Dominant (0) level at pin FDCANx_TX"]
    B_0x2 = 2,
    #[doc = "3: Recessive (1) at pin FDCANx_TX"]
    B_0x3 = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_A {
    type Ux = u8;
}
impl crate::IsEnum for TX_A {}
#[doc = "Field `TX` reader - Control of transmit pin"]
pub type TX_R = crate::FieldReader<TX_A>;
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::B_0x0,
            1 => TX_A::B_0x1,
            2 => TX_A::B_0x2,
            3 => TX_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TX_A::B_0x0
    }
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TX_A::B_0x1
    }
    #[doc = "Dominant (0) level at pin FDCANx_TX"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TX_A::B_0x2
    }
    #[doc = "Recessive (1) at pin FDCANx_TX"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TX_A::B_0x3
    }
}
#[doc = "Field `TX` writer - Control of transmit pin"]
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TX_A, crate::Safe>;
impl<'a, REG> TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::B_0x0)
    }
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::B_0x1)
    }
    #[doc = "Dominant (0) level at pin FDCANx_TX"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::B_0x2)
    }
    #[doc = "Recessive (1) at pin FDCANx_TX"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TX_A::B_0x3)
    }
}
#[doc = "Receive pin Monitors the actual value of pin FDCANx_RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_A {
    #[doc = "0: The CAN bus is dominant (FDCANx_RX = 0)"]
    B_0x0 = 0,
    #[doc = "1: The CAN bus is recessive (FDCANx_RX = 1)"]
    B_0x1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX"]
pub type RX_R = crate::BitReader<RX_A>;
impl RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::B_0x0,
            true => RX_A::B_0x1,
        }
    }
    #[doc = "The CAN bus is dominant (FDCANx_RX = 0)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RX_A::B_0x0
    }
    #[doc = "The CAN bus is recessive (FDCANx_RX = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RX_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    pub fn LBCK(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    pub fn TX(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX"]
    #[inline(always)]
    pub fn RX(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop back mode"]
    #[inline(always)]
    pub fn LBCK(&mut self) -> LBCK_W<'_, TEST_SPEC> {
        LBCK_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of transmit pin"]
    #[inline(always)]
    pub fn TX(&mut self) -> TX_W<'_, TEST_SPEC> {
        TX_W::new(self, 5)
    }
}
#[doc = "FDCAN test register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {}
