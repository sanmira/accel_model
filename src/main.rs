use libc::{c_void, read, write};
use std::fs::OpenOptions;
use std::os::unix::io::{AsRawFd};
use sysinfo::{System, SystemExt};

const I2C_SLAVE: u64 = 0x0703;
const I2C_ADDR: u8 = 0x18;

fn main() -> std::io::Result<()> {
    let i2c_adapter = "/dev/i2c-1";
    let file = OpenOptions::new()
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

    unsafe {
        let result = write(
            file.as_raw_fd(),
            array.to_vec().as_ptr() as *const c_void,
            1,
        );
        if result != 1 {
            panic!("Cannot specify register to read from")
        }
    };

    unsafe {
        let result = read(file.as_raw_fd(), array.as_ptr() as *mut c_void, 1);
        if result != 1 {
            panic!("Cannot read register value")
        }
    };

    let result = array[0];
    println!("Who am I: {:#01x}", result);

    let mut sys = System::new_all();
    sys.refresh_all();
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    Ok(())
}
