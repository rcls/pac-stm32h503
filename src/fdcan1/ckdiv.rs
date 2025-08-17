#[doc = "Register `CKDIV` reader"]
pub type R = crate::R<CKDIV_SPEC>;
#[doc = "Register `CKDIV` writer"]
pub type W = crate::W<CKDIV_SPEC>;
#[doc = "input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDIV_A {
    #[doc = "0: Divide by 1"]
    B_0x0 = 0,
    #[doc = "1: Divide by 2"]
    B_0x1 = 1,
    #[doc = "2: Divide by 4"]
    B_0x2 = 2,
    #[doc = "3: Divide by 6"]
    B_0x3 = 3,
    #[doc = "4: Divide by 8"]
    B_0x4 = 4,
    #[doc = "5: Divide by 10"]
    B_0x5 = 5,
    #[doc = "6: Divide by 12"]
    B_0x6 = 6,
    #[doc = "7: Divide by 14"]
    B_0x7 = 7,
    #[doc = "8: Divide by 16"]
    B_0x8 = 8,
    #[doc = "9: Divide by 18"]
    B_0x9 = 9,
    #[doc = "10: Divide by 20"]
    B_0xA = 10,
    #[doc = "11: Divide by 22"]
    B_0xB = 11,
    #[doc = "12: Divide by 24"]
    B_0xC = 12,
    #[doc = "13: Divide by 26"]
    B_0xD = 13,
    #[doc = "14: Divide by 28"]
    B_0xE = 14,
    #[doc = "15: Divide by 30"]
    B_0xF = 15,
}
impl From<PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for PDIV_A {}
#[doc = "Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type PDIV_R = crate::FieldReader<PDIV_A>;
impl PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDIV_A {
        match self.bits {
            0 => PDIV_A::B_0x0,
            1 => PDIV_A::B_0x1,
            2 => PDIV_A::B_0x2,
            3 => PDIV_A::B_0x3,
            4 => PDIV_A::B_0x4,
            5 => PDIV_A::B_0x5,
            6 => PDIV_A::B_0x6,
            7 => PDIV_A::B_0x7,
            8 => PDIV_A::B_0x8,
            9 => PDIV_A::B_0x9,
            10 => PDIV_A::B_0xA,
            11 => PDIV_A::B_0xB,
            12 => PDIV_A::B_0xC,
            13 => PDIV_A::B_0xD,
            14 => PDIV_A::B_0xE,
            15 => PDIV_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PDIV_A::B_0x0
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PDIV_A::B_0x1
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PDIV_A::B_0x2
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PDIV_A::B_0x3
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PDIV_A::B_0x4
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PDIV_A::B_0x5
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PDIV_A::B_0x6
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PDIV_A::B_0x7
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == PDIV_A::B_0x8
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == PDIV_A::B_0x9
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == PDIV_A::B_0xA
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == PDIV_A::B_0xB
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == PDIV_A::B_0xC
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == PDIV_A::B_0xD
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == PDIV_A::B_0xE
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == PDIV_A::B_0xF
    }
}
#[doc = "Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PDIV_A, crate::Safe>;
impl<'a, REG> PDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x1)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x2)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x3)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x4)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x5)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x6)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x7)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x8)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0x9)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0xA)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0xB)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0xC)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0xD)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0xE)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::B_0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn PDIV(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn PDIV(&mut self) -> PDIV_W<'_, CKDIV_SPEC> {
        PDIV_W::new(self, 0)
    }
}
#[doc = "FDCAN CFG clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKDIV_SPEC;
impl crate::RegisterSpec for CKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckdiv::R`](R) reader structure"]
impl crate::Readable for CKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckdiv::W`](W) writer structure"]
impl crate::Writable for CKDIV_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CKDIV to value 0"]
impl crate::Resettable for CKDIV_SPEC {}
