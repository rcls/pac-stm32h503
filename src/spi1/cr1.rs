#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    #[doc = "0: Serial peripheral disabled."]
    B_0x0 = 0,
    #[doc = "1: Serial peripheral enabled"]
    B_0x1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPE` reader - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
pub type SPE_R = crate::BitReader<SPE_A>;
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::B_0x0,
            true => SPE_A::B_0x1,
        }
    }
    #[doc = "Serial peripheral disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPE_A::B_0x0
    }
    #[doc = "Serial peripheral enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPE_A::B_0x1
    }
}
#[doc = "Field `SPE` writer - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE_A>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Serial peripheral disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::B_0x0)
    }
    #[doc = "Serial peripheral enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::B_0x1)
    }
}
#[doc = "master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASRX_A {
    #[doc = "0: SPI flow/clock generation is continuous, regardless of overrun condition. (data are lost)"]
    B_0x0 = 0,
    #[doc = "1: SPI flow is suspended temporary on RxFIFO full condition, before reaching overrun condition. The SUSP flag is set when the SPI communication is suspended."]
    B_0x1 = 1,
}
impl From<MASRX_A> for bool {
    #[inline(always)]
    fn from(variant: MASRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASRX` reader - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
pub type MASRX_R = crate::BitReader<MASRX_A>;
impl MASRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASRX_A {
        match self.bits {
            false => MASRX_A::B_0x0,
            true => MASRX_A::B_0x1,
        }
    }
    #[doc = "SPI flow/clock generation is continuous, regardless of overrun condition. (data are lost)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MASRX_A::B_0x0
    }
    #[doc = "SPI flow is suspended temporary on RxFIFO full condition, before reaching overrun condition. The SUSP flag is set when the SPI communication is suspended."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MASRX_A::B_0x1
    }
}
#[doc = "Field `MASRX` writer - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG, MASRX_A>;
impl<'a, REG> MASRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI flow/clock generation is continuous, regardless of overrun condition. (data are lost)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MASRX_A::B_0x0)
    }
    #[doc = "SPI flow is suspended temporary on RxFIFO full condition, before reaching overrun condition. The SUSP flag is set when the SPI communication is suspended."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MASRX_A::B_0x1)
    }
}
#[doc = "master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTART_A {
    #[doc = "0: master transfer is at idle"]
    B_0x0 = 0,
    #[doc = "1: master transfer is on-going or temporary suspended by automatic suspend"]
    B_0x1 = 1,
}
impl From<CSTART_A> for bool {
    #[inline(always)]
    fn from(variant: CSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTART` reader - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
pub type CSTART_R = crate::BitReader<CSTART_A>;
impl CSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTART_A {
        match self.bits {
            false => CSTART_A::B_0x0,
            true => CSTART_A::B_0x1,
        }
    }
    #[doc = "master transfer is at idle"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSTART_A::B_0x0
    }
    #[doc = "master transfer is on-going or temporary suspended by automatic suspend"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSTART_A::B_0x1
    }
}
#[doc = "Field `CSTART` writer - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG, CSTART_A>;
impl<'a, REG> CSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "master transfer is at idle"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTART_A::B_0x0)
    }
    #[doc = "master transfer is on-going or temporary suspended by automatic suspend"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTART_A::B_0x1)
    }
}
#[doc = "Field `CSUSP` writer - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts."]
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDDIR_A {
    #[doc = "0: SPI is Receiver"]
    B_0x0 = 0,
    #[doc = "1: SPI is transmitter"]
    B_0x1 = 1,
}
impl From<HDDIR_A> for bool {
    #[inline(always)]
    fn from(variant: HDDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
pub type HDDIR_R = crate::BitReader<HDDIR_A>;
impl HDDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDDIR_A {
        match self.bits {
            false => HDDIR_A::B_0x0,
            true => HDDIR_A::B_0x1,
        }
    }
    #[doc = "SPI is Receiver"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HDDIR_A::B_0x0
    }
    #[doc = "SPI is transmitter"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HDDIR_A::B_0x1
    }
}
#[doc = "Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG, HDDIR_A>;
impl<'a, REG> HDDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is Receiver"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HDDIR_A::B_0x0)
    }
    #[doc = "SPI is transmitter"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HDDIR_A::B_0x1)
    }
}
#[doc = "Field `SSI` reader - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "32-bit CRC polynomial configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC33_17_A {
    #[doc = "0: Full size (33-bit or 17-bit) CRC polynomial is not used"]
    B_0x0 = 0,
    #[doc = "1: Full size (33-bit or 17-bit) CRC polynomial is used"]
    B_0x1 = 1,
}
impl From<CRC33_17_A> for bool {
    #[inline(always)]
    fn from(variant: CRC33_17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC33_17` reader - 32-bit CRC polynomial configuration"]
pub type CRC33_17_R = crate::BitReader<CRC33_17_A>;
impl CRC33_17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRC33_17_A {
        match self.bits {
            false => CRC33_17_A::B_0x0,
            true => CRC33_17_A::B_0x1,
        }
    }
    #[doc = "Full size (33-bit or 17-bit) CRC polynomial is not used"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRC33_17_A::B_0x0
    }
    #[doc = "Full size (33-bit or 17-bit) CRC polynomial is used"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRC33_17_A::B_0x1
    }
}
#[doc = "Field `CRC33_17` writer - 32-bit CRC polynomial configuration"]
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG, CRC33_17_A>;
impl<'a, REG> CRC33_17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full size (33-bit or 17-bit) CRC polynomial is not used"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRC33_17_A::B_0x0)
    }
    #[doc = "Full size (33-bit or 17-bit) CRC polynomial is used"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRC33_17_A::B_0x1)
    }
}
#[doc = "CRC calculation initialization pattern control for receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCRCINI_A {
    #[doc = "0: All zero pattern is applied"]
    B_0x0 = 0,
    #[doc = "1: All ones pattern is applied"]
    B_0x1 = 1,
}
impl From<RCRCINI_A> for bool {
    #[inline(always)]
    fn from(variant: RCRCINI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRCINI` reader - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_R = crate::BitReader<RCRCINI_A>;
impl RCRCINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCRCINI_A {
        match self.bits {
            false => RCRCINI_A::B_0x0,
            true => RCRCINI_A::B_0x1,
        }
    }
    #[doc = "All zero pattern is applied"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RCRCINI_A::B_0x0
    }
    #[doc = "All ones pattern is applied"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RCRCINI_A::B_0x1
    }
}
#[doc = "Field `RCRCINI` writer - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG, RCRCINI_A>;
impl<'a, REG> RCRCINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All zero pattern is applied"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RCRCINI_A::B_0x0)
    }
    #[doc = "All ones pattern is applied"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RCRCINI_A::B_0x1)
    }
}
#[doc = "CRC calculation initialization pattern control for transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRCINI_A {
    #[doc = "0: all zero pattern is applied"]
    B_0x0 = 0,
    #[doc = "1: all ones pattern is applied"]
    B_0x1 = 1,
}
impl From<TCRCINI_A> for bool {
    #[inline(always)]
    fn from(variant: TCRCINI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCRCINI` reader - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_R = crate::BitReader<TCRCINI_A>;
impl TCRCINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCRCINI_A {
        match self.bits {
            false => TCRCINI_A::B_0x0,
            true => TCRCINI_A::B_0x1,
        }
    }
    #[doc = "all zero pattern is applied"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCRCINI_A::B_0x0
    }
    #[doc = "all ones pattern is applied"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCRCINI_A::B_0x1
    }
}
#[doc = "Field `TCRCINI` writer - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG, TCRCINI_A>;
impl<'a, REG> TCRCINI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "all zero pattern is applied"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCRCINI_A::B_0x0)
    }
    #[doc = "all ones pattern is applied"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCRCINI_A::B_0x1)
    }
}
#[doc = "locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOLOCK_A {
    #[doc = "0: AF configuration is not locked"]
    B_0x0 = 0,
    #[doc = "1: AF configuration is locked"]
    B_0x1 = 1,
}
impl From<IOLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: IOLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOLOCK` reader - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
pub type IOLOCK_R = crate::BitReader<IOLOCK_A>;
impl IOLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOLOCK_A {
        match self.bits {
            false => IOLOCK_A::B_0x0,
            true => IOLOCK_A::B_0x1,
        }
    }
    #[doc = "AF configuration is not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IOLOCK_A::B_0x0
    }
    #[doc = "AF configuration is locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IOLOCK_A::B_0x1
    }
}
#[doc = "Field `IOLOCK` writer - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG, IOLOCK_A>;
impl<'a, REG> IOLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AF configuration is not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IOLOCK_A::B_0x0)
    }
    #[doc = "AF configuration is locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IOLOCK_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
    #[inline(always)]
    pub fn SPE(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
    #[inline(always)]
    pub fn MASRX(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
    #[inline(always)]
    pub fn CSTART(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
    #[inline(always)]
    pub fn HDDIR(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
    #[inline(always)]
    pub fn SSI(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn CRC33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn RCRCINI(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn TCRCINI(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
    #[inline(always)]
    pub fn IOLOCK(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
    #[inline(always)]
    pub fn SPE(&mut self) -> SPE_W<'_, CR1_SPEC> {
        SPE_W::new(self, 0)
    }
    #[doc = "Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
    #[inline(always)]
    pub fn MASRX(&mut self) -> MASRX_W<'_, CR1_SPEC> {
        MASRX_W::new(self, 8)
    }
    #[doc = "Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the . In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
    #[inline(always)]
    pub fn CSTART(&mut self) -> CSTART_W<'_, CR1_SPEC> {
        CSTART_W::new(self, 9)
    }
    #[doc = "Bit 10 - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts."]
    #[inline(always)]
    pub fn CSUSP(&mut self) -> CSUSP_W<'_, CR1_SPEC> {
        CSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
    #[inline(always)]
    pub fn HDDIR(&mut self) -> HDDIR_W<'_, CR1_SPEC> {
        HDDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
    #[inline(always)]
    pub fn SSI(&mut self) -> SSI_W<'_, CR1_SPEC> {
        SSI_W::new(self, 12)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn CRC33_17(&mut self) -> CRC33_17_W<'_, CR1_SPEC> {
        CRC33_17_W::new(self, 13)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn RCRCINI(&mut self) -> RCRCINI_W<'_, CR1_SPEC> {
        RCRCINI_W::new(self, 14)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn TCRCINI(&mut self) -> TCRCINI_W<'_, CR1_SPEC> {
        TCRCINI_W::new(self, 15)
    }
    #[doc = "Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
    #[inline(always)]
    pub fn IOLOCK(&mut self) -> IOLOCK_W<'_, CR1_SPEC> {
        IOLOCK_W::new(self, 16)
    }
}
#[doc = "SPI/I2S control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {}
