use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::os::unix::io::{AsRawFd};
use std::fs::File;
use std::{thread, time::Duration};

const I2C_SLAVE: u64 = 0x0703;
const I2C_ADDR: u8 = 0x18;

fn read_acc_value(file: &mut File, reg: u8) -> Result<i16, std::io::Error> {
    let mut array: [u8; 1] = [0; 1];
    array[0] = reg;
    
    file.write(&mut array)?;
    file.read(&mut array)?;

    let result: i16 = array[0].into();
    Ok(result)
}

fn write_acc_value(file: &mut File, reg: u8, val: u8) -> Result<(), std::io::Error> {
    let mut array: [u8; 2] = [0; 2];
    array[0] = reg;
    array[1] = val;
    
    file.write(&mut array)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let i2c_adapter = "/dev/i2c-1";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(i2c_adapter)?;

    // Set the slave address of the I2C device
    unsafe {
        if libc::ioctl(file.as_raw_fd(), I2C_SLAVE, I2C_ADDR as libc::c_uint) < 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    let who_i_am_reg: u8 = 0x0F;
    let result = read_acc_value(&mut file, who_i_am_reg)?;
    println!("Who am I: {:#01x}", result);

    write_acc_value(&mut file, 0x20, 0x27)?;
    write_acc_value(&mut file, 0x21, 0x00)?;
    write_acc_value(&mut file, 0x23, 0x00)?;

    loop {
        let x_val: i16 = (read_acc_value(&mut file, 0x28)? | (read_acc_value(&mut file, 0x29)? << 8)).into();
        let y_val: i16 = (read_acc_value(&mut file, 0x2A)? | (read_acc_value(&mut file, 0x2B)? << 8)).into();
        let z_val: i16 = (read_acc_value(&mut file, 0x2C)? | (read_acc_value(&mut file, 0x2D)? << 8)).into();

        println!("x_val: {}, y_val: {}, z_val: {}", x_val, y_val, z_val);
        thread::sleep(Duration::from_millis(100));
    }

    //Ok(())
}
