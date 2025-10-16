use core::arch::asm;
use stm32f4xx_hal::gpio::Output;
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::spi::{Instance, Spi};

enum DataAndStatusRegCmds {
    WhoAmI = 0x0F,
    Status = 0x27,
    OutX = 0x29,
    OutY = 0x2B,
    OutZ = 0x2D,
}

/// # LIS302DL
/// ST MEMS 3-axis accelerometer
pub struct LIS302DL<SpiType>
where
    SpiType: Instance,
{
    pub spi: Spi<SpiType>,
    pub spi_cs: Pin<'E', 3, Output>,
}

impl<SpiType> LIS302DL<SpiType>
where
    SpiType: Instance,
{
    /// # init
    /// START-UP SEQUENCE:
    /// - CTRL_REG1: With this command the three acceleration channels (i.e. X, Y and Z axis) are
    /// enabled and the Output Data Rate is set to 100 Hz
    pub fn init(&mut self) {
        let control_register = 0x20;
        self.spi_cs.set_low();
        self.wait_1us();
        self.spi.write(&[control_register, 0x47]).unwrap(); // CTRL_REG1: Power on, enable X/Y/Z, 100Hz
        self.spi_cs.set_high();
        self.wait_1us();
    }

    /// # get_device_id
    /// Device identification register.
    /// This register contains a device identifier which for LIS302DL is set to 0x3B.
    pub fn get_device_id(&mut self) -> u8 {
        self.mems_read(DataAndStatusRegCmds::WhoAmI as u8)
    }

    /// # get_device_status
    /// Data output status register.
    pub fn get_device_status(&mut self) -> u8 {
        self.mems_read(DataAndStatusRegCmds::Status as u8)
    }

    /// # read_x_axis
    /// X-axis output register.
    pub fn read_x_axis(&mut self) -> i8 {
        self.mems_read(DataAndStatusRegCmds::OutX as u8) as i8
    }

    /// # read_y_axis
    /// Y-axis output register.
    pub fn read_y_axis(&mut self) -> i8 {
        self.mems_read(DataAndStatusRegCmds::OutY as u8) as i8
    }

    /// # read_z_axis
    /// Z-axis output register.
    pub fn read_z_axis(&mut self) -> i8 {
        self.mems_read(DataAndStatusRegCmds::OutZ as u8) as i8
    }

    fn mems_read(&mut self, cmd: u8) -> u8 {
        let read_cmd = cmd | 0x80;
        let mut buffer = [read_cmd, 0x00];
        self.spi_cs.set_low();
        self.wait_1us();
        self.spi.transfer_in_place(&mut buffer).unwrap();
        self.spi_cs.set_high();
        self.wait_1us();

        return buffer[1];
    }

    /// This only works if uC works on 168 Mhz
    /// 1 cycle ≈ 1/168MHz ​≈ 6ns
    /// 5.95ns / 1μs​ ≈ 168 cycles
    fn wait_1us(&self) {
        for _ in 0..168 {
            unsafe {
                asm!("nop");
            }
        }
    }
}
