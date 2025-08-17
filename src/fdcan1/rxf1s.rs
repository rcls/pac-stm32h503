#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<RXF1S_SPEC>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
pub type F1FL_R = crate::FieldReader;
#[doc = "Field `F1GI` reader - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
pub type F1GI_R = crate::FieldReader;
#[doc = "Field `F1PI` reader - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
pub type F1PI_R = crate::FieldReader;
#[doc = "Rx FIFO 1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F1F_A {
    #[doc = "0: Rx FIFO 1 not full"]
    B_0x0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B_0x1 = 1,
}
impl From<F1F_A> for bool {
    #[inline(always)]
    fn from(variant: F1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F1F` reader - Rx FIFO 1 full"]
pub type F1F_R = crate::BitReader<F1F_A>;
impl F1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F1F_A {
        match self.bits {
            false => F1F_A::B_0x0,
            true => F1F_A::B_0x1,
        }
    }
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == F1F_A::B_0x0
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == F1F_A::B_0x1
    }
}
#[doc = "Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\] is reset, this bit is also reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1L_A {
    #[doc = "0: No Rx FIFO 1 message lost"]
    B_0x0 = 0,
    #[doc = "1: Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0"]
    B_0x1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\] is reset, this bit is also reset."]
pub type RF1L_R = crate::BitReader<RF1L_A>;
impl RF1L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0x0,
            true => RF1L_A::B_0x1,
        }
    }
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1L_A::B_0x0
    }
    #[doc = "Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1L_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3."]
    #[inline(always)]
    pub fn F1FL(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn F1GI(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn F1PI(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn F1F(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\\[RF1L\\]. When IR\\[RF1L\\] is reset, this bit is also reset."]
    #[inline(always)]
    pub fn RF1L(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for RXF1S_SPEC {}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for RXF1S_SPEC {}
