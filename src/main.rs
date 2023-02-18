use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::os::unix::io::{AsRawFd};

const I2C_SLAVE: u64 = 0x0703;
const I2C_ADDR: u8 = 0x18;

fn main() -> std::io::Result<()> {
    let i2c_adapter = "/dev/i2c-1";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(i2c_adapter)?;

    // Set the slave address of the I2C device
    let slave_addr = 0x18; // Replace with the actual I2C address of your device
    unsafe {
        if libc::ioctl(file.as_raw_fd(), I2C_SLAVE, slave_addr as libc::c_ulong) < 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    let who_i_am_reg: u8 = 0x0F;
    let mut array: [u8; 1] = [0; 1];
    array[0] = who_i_am_reg;

    println!("{:#01x}", I2C_SLAVE);
    println!("{:#01x}", I2C_ADDR);

    file.write(&mut array)?;

    file.read(&mut array)?;

    let result = array[0];
    println!("Who am I: {:#01x}", result);

    Ok(())
}
