#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PRESC_SPEC>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PRESC_SPEC>;
#[doc = "Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: input clock not divided"]
    B_0x0 = 0,
    #[doc = "1: input clock divided by 2"]
    B_0x1 = 1,
    #[doc = "2: input clock divided by 4"]
    B_0x2 = 2,
    #[doc = "3: input clock divided by 6"]
    B_0x3 = 3,
    #[doc = "4: input clock divided by 8"]
    B_0x4 = 4,
    #[doc = "5: input clock divided by 10"]
    B_0x5 = 5,
    #[doc = "6: input clock divided by 12"]
    B_0x6 = 6,
    #[doc = "7: input clock divided by 16"]
    B_0x7 = 7,
    #[doc = "8: input clock divided by 32"]
    B_0x8 = 8,
    #[doc = "9: input clock divided by 64"]
    B_0x9 = 9,
    #[doc = "10: input clock divided by 128"]
    B_0xA = 10,
    #[doc = "11: input clock divided by 256"]
    B_0xB = 11,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER_A {
    type Ux = u8;
}
impl crate::IsEnum for PRESCALER_A {}
#[doc = "Field `PRESCALER` reader - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
pub type PRESCALER_R = crate::FieldReader<PRESCALER_A>;
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCALER_A> {
        match self.bits {
            0 => Some(PRESCALER_A::B_0x0),
            1 => Some(PRESCALER_A::B_0x1),
            2 => Some(PRESCALER_A::B_0x2),
            3 => Some(PRESCALER_A::B_0x3),
            4 => Some(PRESCALER_A::B_0x4),
            5 => Some(PRESCALER_A::B_0x5),
            6 => Some(PRESCALER_A::B_0x6),
            7 => Some(PRESCALER_A::B_0x7),
            8 => Some(PRESCALER_A::B_0x8),
            9 => Some(PRESCALER_A::B_0x9),
            10 => Some(PRESCALER_A::B_0xA),
            11 => Some(PRESCALER_A::B_0xB),
            _ => None,
        }
    }
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRESCALER_A::B_0x0
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRESCALER_A::B_0x1
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PRESCALER_A::B_0x2
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PRESCALER_A::B_0x3
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PRESCALER_A::B_0x4
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PRESCALER_A::B_0x5
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PRESCALER_A::B_0x6
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PRESCALER_A::B_0x7
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == PRESCALER_A::B_0x8
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == PRESCALER_A::B_0x9
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == PRESCALER_A::B_0xA
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == PRESCALER_A::B_0xB
    }
}
#[doc = "Field `PRESCALER` writer - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESCALER_A>;
impl<'a, REG> PRESCALER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x0)
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x1)
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x2)
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x3)
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x4)
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x5)
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x6)
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x7)
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x8)
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0x9)
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0xA)
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0xB)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn PRESCALER(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn PRESCALER(&mut self) -> PRESCALER_W<'_, PRESC_SPEC> {
        PRESCALER_W::new(self, 0)
    }
}
#[doc = "LPUART prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PRESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PRESC_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESC_SPEC {}
