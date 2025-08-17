#[doc = "Register `MAXRLR` reader"]
pub type R = crate::R<MAXRLR_SPEC>;
#[doc = "Register `MAXRLR` writer"]
pub type W = crate::W<MAXRLR_SPEC>;
#[doc = "Field `MRL` reader - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
pub type MRL_R = crate::FieldReader<u16>;
#[doc = "Field `MRL` writer - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
pub type MRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IBIP_A {
    #[doc = "0: null payload data size (only allowed when IC3_BCR.BCR2=0)"]
    B_0x0 = 0,
    #[doc = "1: 1 byte (i.e. mandatory data byte MDB\\[7:0\\]"]
    B_0x1 = 1,
    #[doc = "2: 2 bytes (including first MDB\\[7:0\\])"]
    B_0x2 = 2,
    #[doc = "3: 3 bytes (including first MDB\\[7:0\\])"]
    B_0x3 = 3,
    #[doc = "4: 4 bytes (including first MDB\\[7:0\\])"]
    B_0x4 = 4,
}
impl From<IBIP_A> for u8 {
    #[inline(always)]
    fn from(variant: IBIP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IBIP_A {
    type Ux = u8;
}
impl crate::IsEnum for IBIP_A {}
#[doc = "Field `IBIP` reader - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
pub type IBIP_R = crate::FieldReader<IBIP_A>;
impl IBIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IBIP_A> {
        match self.bits {
            0 => Some(IBIP_A::B_0x0),
            1 => Some(IBIP_A::B_0x1),
            2 => Some(IBIP_A::B_0x2),
            3 => Some(IBIP_A::B_0x3),
            4 => Some(IBIP_A::B_0x4),
            _ => None,
        }
    }
    #[doc = "null payload data size (only allowed when IC3_BCR.BCR2=0)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBIP_A::B_0x0
    }
    #[doc = "1 byte (i.e. mandatory data byte MDB\\[7:0\\]"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBIP_A::B_0x1
    }
    #[doc = "2 bytes (including first MDB\\[7:0\\])"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IBIP_A::B_0x2
    }
    #[doc = "3 bytes (including first MDB\\[7:0\\])"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IBIP_A::B_0x3
    }
    #[doc = "4 bytes (including first MDB\\[7:0\\])"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == IBIP_A::B_0x4
    }
}
#[doc = "Field `IBIP` writer - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
pub type IBIP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, IBIP_A>;
impl<'a, REG> IBIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "null payload data size (only allowed when IC3_BCR.BCR2=0)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IBIP_A::B_0x0)
    }
    #[doc = "1 byte (i.e. mandatory data byte MDB\\[7:0\\]"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IBIP_A::B_0x1)
    }
    #[doc = "2 bytes (including first MDB\\[7:0\\])"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IBIP_A::B_0x2)
    }
    #[doc = "3 bytes (including first MDB\\[7:0\\])"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IBIP_A::B_0x3)
    }
    #[doc = "4 bytes (including first MDB\\[7:0\\])"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(IBIP_A::B_0x4)
    }
}
impl R {
    #[doc = "Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
    #[inline(always)]
    pub fn MRL(&self) -> MRL_R {
        MRL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
    #[inline(always)]
    pub fn IBIP(&self) -> IBIP_R {
        IBIP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
    #[inline(always)]
    pub fn MRL(&mut self) -> MRL_W<'_, MAXRLR_SPEC> {
        MRL_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
    #[inline(always)]
    pub fn IBIP(&mut self) -> IBIP_W<'_, MAXRLR_SPEC> {
        IBIP_W::new(self, 16)
    }
}
#[doc = "I3C maximum read length register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXRLR_SPEC;
impl crate::RegisterSpec for MAXRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxrlr::R`](R) reader structure"]
impl crate::Readable for MAXRLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxrlr::W`](W) writer structure"]
impl crate::Writable for MAXRLR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MAXRLR to value 0"]
impl crate::Resettable for MAXRLR_SPEC {}
