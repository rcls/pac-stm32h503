#[doc = "Register `C4FCR` writer"]
pub type W = crate::W<C4FCR_SPEC>;
#[doc = "transfer complete flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding TCF flag cleared"]
    B_0x1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` writer - transfer complete flag clear"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG, TCF_A>;
impl<'a, REG> TCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCF_A::B_0x0)
    }
    #[doc = "corresponding TCF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCF_A::B_0x1)
    }
}
#[doc = "half transfer flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding HTF flag cleared"]
    B_0x1 = 1,
}
impl From<HTF_A> for bool {
    #[inline(always)]
    fn from(variant: HTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTF` writer - half transfer flag clear"]
pub type HTF_W<'a, REG> = crate::BitWriter<'a, REG, HTF_A>;
impl<'a, REG> HTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HTF_A::B_0x0)
    }
    #[doc = "corresponding HTF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HTF_A::B_0x1)
    }
}
#[doc = "data transfer error flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding DTEF flag cleared"]
    B_0x1 = 1,
}
impl From<DTEF_A> for bool {
    #[inline(always)]
    fn from(variant: DTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEF` writer - data transfer error flag clear"]
pub type DTEF_W<'a, REG> = crate::BitWriter<'a, REG, DTEF_A>;
impl<'a, REG> DTEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTEF_A::B_0x0)
    }
    #[doc = "corresponding DTEF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTEF_A::B_0x1)
    }
}
#[doc = "update link transfer error flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULEF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding ULEF flag cleared"]
    B_0x1 = 1,
}
impl From<ULEF_A> for bool {
    #[inline(always)]
    fn from(variant: ULEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULEF` writer - update link transfer error flag clear"]
pub type ULEF_W<'a, REG> = crate::BitWriter<'a, REG, ULEF_A>;
impl<'a, REG> ULEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ULEF_A::B_0x0)
    }
    #[doc = "corresponding ULEF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ULEF_A::B_0x1)
    }
}
#[doc = "user setting error flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding USEF flag cleared"]
    B_0x1 = 1,
}
impl From<USEF_A> for bool {
    #[inline(always)]
    fn from(variant: USEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEF` writer - user setting error flag clear"]
pub type USEF_W<'a, REG> = crate::BitWriter<'a, REG, USEF_A>;
impl<'a, REG> USEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USEF_A::B_0x0)
    }
    #[doc = "corresponding USEF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USEF_A::B_0x1)
    }
}
#[doc = "completed suspension flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding SUSPF flag cleared"]
    B_0x1 = 1,
}
impl From<SUSPF_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPF` writer - completed suspension flag clear"]
pub type SUSPF_W<'a, REG> = crate::BitWriter<'a, REG, SUSPF_A>;
impl<'a, REG> SUSPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPF_A::B_0x0)
    }
    #[doc = "corresponding SUSPF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPF_A::B_0x1)
    }
}
#[doc = "trigger overrun flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: corresponding TOF flag cleared"]
    B_0x1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` writer - trigger overrun flag clear"]
pub type TOF_W<'a, REG> = crate::BitWriter<'a, REG, TOF_A>;
impl<'a, REG> TOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOF_A::B_0x0)
    }
    #[doc = "corresponding TOF flag cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOF_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 8 - transfer complete flag clear"]
    #[inline(always)]
    pub fn TCF(&mut self) -> TCF_W<'_, C4FCR_SPEC> {
        TCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - half transfer flag clear"]
    #[inline(always)]
    pub fn HTF(&mut self) -> HTF_W<'_, C4FCR_SPEC> {
        HTF_W::new(self, 9)
    }
    #[doc = "Bit 10 - data transfer error flag clear"]
    #[inline(always)]
    pub fn DTEF(&mut self) -> DTEF_W<'_, C4FCR_SPEC> {
        DTEF_W::new(self, 10)
    }
    #[doc = "Bit 11 - update link transfer error flag clear"]
    #[inline(always)]
    pub fn ULEF(&mut self) -> ULEF_W<'_, C4FCR_SPEC> {
        ULEF_W::new(self, 11)
    }
    #[doc = "Bit 12 - user setting error flag clear"]
    #[inline(always)]
    pub fn USEF(&mut self) -> USEF_W<'_, C4FCR_SPEC> {
        USEF_W::new(self, 12)
    }
    #[doc = "Bit 13 - completed suspension flag clear"]
    #[inline(always)]
    pub fn SUSPF(&mut self) -> SUSPF_W<'_, C4FCR_SPEC> {
        SUSPF_W::new(self, 13)
    }
    #[doc = "Bit 14 - trigger overrun flag clear"]
    #[inline(always)]
    pub fn TOF(&mut self) -> TOF_W<'_, C4FCR_SPEC> {
        TOF_W::new(self, 14)
    }
}
#[doc = "GPDMA channel 4 flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4FCR_SPEC;
impl crate::RegisterSpec for C4FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`c4fcr::W`](W) writer structure"]
impl crate::Writable for C4FCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C4FCR to value 0"]
impl crate::Resettable for C4FCR_SPEC {}
