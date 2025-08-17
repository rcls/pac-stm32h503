#[doc = "Register `ATCR2` reader"]
pub type R = crate::R<ATCR2_SPEC>;
#[doc = "Register `ATCR2` writer"]
pub type W = crate::W<ATCR2_SPEC>;
#[doc = "Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL1_A {
    #[doc = "0: TAMPOUTSEL1 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL1 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL1 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL1 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL1 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL1 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL1 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL1 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL1_A {}
#[doc = "Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL1_R = crate::FieldReader<ATOSEL1_A>;
impl ATOSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL1_A {
        match self.bits {
            0 => ATOSEL1_A::B_0x0,
            1 => ATOSEL1_A::B_0x1,
            2 => ATOSEL1_A::B_0x2,
            3 => ATOSEL1_A::B_0x3,
            4 => ATOSEL1_A::B_0x4,
            5 => ATOSEL1_A::B_0x5,
            6 => ATOSEL1_A::B_0x6,
            7 => ATOSEL1_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL1_A::B_0x0
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL1_A::B_0x1
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL1_A::B_0x2
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL1_A::B_0x3
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL1_A::B_0x4
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL1_A::B_0x5
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL1_A::B_0x6
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL1_A::B_0x7
    }
}
#[doc = "Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL1_A, crate::Safe>;
impl<'a, REG> ATOSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL1 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL1 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL1_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL2_A {
    #[doc = "0: TAMPOUTSEL2 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL2 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL2 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL2 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL2 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL2 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL2 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL2 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL2_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL2_A {}
#[doc = "Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL2_R = crate::FieldReader<ATOSEL2_A>;
impl ATOSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL2_A {
        match self.bits {
            0 => ATOSEL2_A::B_0x0,
            1 => ATOSEL2_A::B_0x1,
            2 => ATOSEL2_A::B_0x2,
            3 => ATOSEL2_A::B_0x3,
            4 => ATOSEL2_A::B_0x4,
            5 => ATOSEL2_A::B_0x5,
            6 => ATOSEL2_A::B_0x6,
            7 => ATOSEL2_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL2_A::B_0x0
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL2_A::B_0x1
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL2_A::B_0x2
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL2_A::B_0x3
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL2_A::B_0x4
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL2_A::B_0x5
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL2_A::B_0x6
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL2_A::B_0x7
    }
}
#[doc = "Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL2_A, crate::Safe>;
impl<'a, REG> ATOSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL2 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL2 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL2_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL3_A {
    #[doc = "0: TAMPOUTSEL3 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL3 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL3 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL3 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL3 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL3 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL3 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL3 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL3_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL3_A {}
#[doc = "Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL3_R = crate::FieldReader<ATOSEL3_A>;
impl ATOSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL3_A {
        match self.bits {
            0 => ATOSEL3_A::B_0x0,
            1 => ATOSEL3_A::B_0x1,
            2 => ATOSEL3_A::B_0x2,
            3 => ATOSEL3_A::B_0x3,
            4 => ATOSEL3_A::B_0x4,
            5 => ATOSEL3_A::B_0x5,
            6 => ATOSEL3_A::B_0x6,
            7 => ATOSEL3_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL3_A::B_0x0
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL3_A::B_0x1
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL3_A::B_0x2
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL3_A::B_0x3
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL3_A::B_0x4
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL3_A::B_0x5
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL3_A::B_0x6
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL3_A::B_0x7
    }
}
#[doc = "Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL3_A, crate::Safe>;
impl<'a, REG> ATOSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL3 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL3 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL3_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL4_A {
    #[doc = "0: TAMPOUTSEL4 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL4 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL4 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL4 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL4 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL4 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL4 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL4 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL4_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL4_A {}
#[doc = "Field `ATOSEL4` reader - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL4_R = crate::FieldReader<ATOSEL4_A>;
impl ATOSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL4_A {
        match self.bits {
            0 => ATOSEL4_A::B_0x0,
            1 => ATOSEL4_A::B_0x1,
            2 => ATOSEL4_A::B_0x2,
            3 => ATOSEL4_A::B_0x3,
            4 => ATOSEL4_A::B_0x4,
            5 => ATOSEL4_A::B_0x5,
            6 => ATOSEL4_A::B_0x6,
            7 => ATOSEL4_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL4_A::B_0x0
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL4_A::B_0x1
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL4_A::B_0x2
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL4_A::B_0x3
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL4_A::B_0x4
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL4_A::B_0x5
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL4_A::B_0x6
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL4_A::B_0x7
    }
}
#[doc = "Field `ATOSEL4` writer - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
pub type ATOSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL4_A, crate::Safe>;
impl<'a, REG> ATOSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL4 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL4 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL4_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 5 selection The selected output must be available in the package pinout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL5_A {
    #[doc = "0: TAMPOUTSEL5 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL5 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL5 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL5 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL5 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL5 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL5 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL5 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL5_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL5_A {}
#[doc = "Field `ATOSEL5` reader - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
pub type ATOSEL5_R = crate::FieldReader<ATOSEL5_A>;
impl ATOSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL5_A {
        match self.bits {
            0 => ATOSEL5_A::B_0x0,
            1 => ATOSEL5_A::B_0x1,
            2 => ATOSEL5_A::B_0x2,
            3 => ATOSEL5_A::B_0x3,
            4 => ATOSEL5_A::B_0x4,
            5 => ATOSEL5_A::B_0x5,
            6 => ATOSEL5_A::B_0x6,
            7 => ATOSEL5_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL5_A::B_0x0
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL5_A::B_0x1
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL5_A::B_0x2
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL5_A::B_0x3
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL5_A::B_0x4
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL5_A::B_0x5
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL5_A::B_0x6
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL5_A::B_0x7
    }
}
#[doc = "Field `ATOSEL5` writer - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
pub type ATOSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL5_A, crate::Safe>;
impl<'a, REG> ATOSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL5 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL5 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL5_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 6 selection The selected output must be available in the package pinout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL6_A {
    #[doc = "0: TAMPOUTSEL6 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL6 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL6 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL6 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL6 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL6 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL6 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL6 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL6_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL6_A {}
#[doc = "Field `ATOSEL6` reader - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
pub type ATOSEL6_R = crate::FieldReader<ATOSEL6_A>;
impl ATOSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL6_A {
        match self.bits {
            0 => ATOSEL6_A::B_0x0,
            1 => ATOSEL6_A::B_0x1,
            2 => ATOSEL6_A::B_0x2,
            3 => ATOSEL6_A::B_0x3,
            4 => ATOSEL6_A::B_0x4,
            5 => ATOSEL6_A::B_0x5,
            6 => ATOSEL6_A::B_0x6,
            7 => ATOSEL6_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL6_A::B_0x0
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL6_A::B_0x1
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL6_A::B_0x2
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL6_A::B_0x3
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL6_A::B_0x4
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL6_A::B_0x5
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL6_A::B_0x6
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL6_A::B_0x7
    }
}
#[doc = "Field `ATOSEL6` writer - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
pub type ATOSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL6_A, crate::Safe>;
impl<'a, REG> ATOSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL6 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL6 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL6_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 7 selection The selected output must be available in the package pinout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL7_A {
    #[doc = "0: TAMPOUTSEL7 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL7 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL7 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL7 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL7 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL7 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL7 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL7 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL7_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL7_A {}
#[doc = "Field `ATOSEL7` reader - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
pub type ATOSEL7_R = crate::FieldReader<ATOSEL7_A>;
impl ATOSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL7_A {
        match self.bits {
            0 => ATOSEL7_A::B_0x0,
            1 => ATOSEL7_A::B_0x1,
            2 => ATOSEL7_A::B_0x2,
            3 => ATOSEL7_A::B_0x3,
            4 => ATOSEL7_A::B_0x4,
            5 => ATOSEL7_A::B_0x5,
            6 => ATOSEL7_A::B_0x6,
            7 => ATOSEL7_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL7_A::B_0x0
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL7_A::B_0x1
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL7_A::B_0x2
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL7_A::B_0x3
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL7_A::B_0x4
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL7_A::B_0x5
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL7_A::B_0x6
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL7_A::B_0x7
    }
}
#[doc = "Field `ATOSEL7` writer - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
pub type ATOSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL7_A, crate::Safe>;
impl<'a, REG> ATOSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL7 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL7 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL7_A::B_0x7)
    }
}
#[doc = "Active tamper shared output 8 selection The selected output must be available in the package pinout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATOSEL8_A {
    #[doc = "0: TAMPOUTSEL8 = TAMP_OUT1"]
    B_0x0 = 0,
    #[doc = "1: TAMPOUTSEL8 = TAMP_OUT2"]
    B_0x1 = 1,
    #[doc = "2: TAMPOUTSEL8 = TAMP_OUT3"]
    B_0x2 = 2,
    #[doc = "3: TAMPOUTSEL8 = TAMP_OUT4"]
    B_0x3 = 3,
    #[doc = "4: TAMPOUTSEL8 = TAMP_OUT5"]
    B_0x4 = 4,
    #[doc = "5: TAMPOUTSEL8 = TAMP_OUT6"]
    B_0x5 = 5,
    #[doc = "6: TAMPOUTSEL8 = TAMP_OUT7"]
    B_0x6 = 6,
    #[doc = "7: TAMPOUTSEL8 = TAMP_OUT8"]
    B_0x7 = 7,
}
impl From<ATOSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: ATOSEL8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ATOSEL8_A {
    type Ux = u8;
}
impl crate::IsEnum for ATOSEL8_A {}
#[doc = "Field `ATOSEL8` reader - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
pub type ATOSEL8_R = crate::FieldReader<ATOSEL8_A>;
impl ATOSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATOSEL8_A {
        match self.bits {
            0 => ATOSEL8_A::B_0x0,
            1 => ATOSEL8_A::B_0x1,
            2 => ATOSEL8_A::B_0x2,
            3 => ATOSEL8_A::B_0x3,
            4 => ATOSEL8_A::B_0x4,
            5 => ATOSEL8_A::B_0x5,
            6 => ATOSEL8_A::B_0x6,
            7 => ATOSEL8_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ATOSEL8_A::B_0x0
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ATOSEL8_A::B_0x1
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ATOSEL8_A::B_0x2
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ATOSEL8_A::B_0x3
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ATOSEL8_A::B_0x4
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ATOSEL8_A::B_0x5
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT7"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == ATOSEL8_A::B_0x6
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == ATOSEL8_A::B_0x7
    }
}
#[doc = "Field `ATOSEL8` writer - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
pub type ATOSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATOSEL8_A, crate::Safe>;
impl<'a, REG> ATOSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAMPOUTSEL8 = TAMP_OUT1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x0)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x1)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x2)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x3)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x4)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x5)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT7"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x6)
    }
    #[doc = "TAMPOUTSEL8 = TAMP_OUT8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ATOSEL8_A::B_0x7)
    }
}
impl R {
    #[doc = "Bits 8:10 - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL1(&mut self) -> ATOSEL1_W<'_, ATCR2_SPEC> {
        ATOSEL1_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL2(&mut self) -> ATOSEL2_W<'_, ATCR2_SPEC> {
        ATOSEL2_W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL3(&mut self) -> ATOSEL3_W<'_, ATCR2_SPEC> {
        ATOSEL3_W::new(self, 14)
    }
    #[doc = "Bits 17:19 - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\\[1:0\\] in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1."]
    #[inline(always)]
    pub fn ATOSEL4(&mut self) -> ATOSEL4_W<'_, ATCR2_SPEC> {
        ATOSEL4_W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Active tamper shared output 5 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL5(&mut self) -> ATOSEL5_W<'_, ATCR2_SPEC> {
        ATOSEL5_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - Active tamper shared output 6 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL6(&mut self) -> ATOSEL6_W<'_, ATCR2_SPEC> {
        ATOSEL6_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - Active tamper shared output 7 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL7(&mut self) -> ATOSEL7_W<'_, ATCR2_SPEC> {
        ATOSEL7_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - Active tamper shared output 8 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn ATOSEL8(&mut self) -> ATOSEL8_W<'_, ATCR2_SPEC> {
        ATOSEL8_W::new(self, 29)
    }
}
#[doc = "TAMP active tamper control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`atcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATCR2_SPEC;
impl crate::RegisterSpec for ATCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atcr2::R`](R) reader structure"]
impl crate::Readable for ATCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atcr2::W`](W) writer structure"]
impl crate::Writable for ATCR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ATCR2 to value 0"]
impl crate::Resettable for ATCR2_SPEC {}
