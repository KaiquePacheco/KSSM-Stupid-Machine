const RAM_SIZE: usize = 1028;

pub(crate) struct Ram{
  array: [u8; RAM_SIZE]
}
impl Ram{
  pub fn new() -> Self{
    Ram { array: [0; RAM_SIZE] }
  }

  pub fn read_byte(&self, address: usize) -> Result<u8, &'static str>{
    if address >= RAM_SIZE{
      return Err("The address exceeds the ram size.")
    }
    Ok(self.array[address])
  }
  pub fn read_bytes<'a>(&self, address: usize, num_bytes: usize) -> Result<&[u8], &'static str> {
    if address + num_bytes >= RAM_SIZE{
      return Err("The addresses exceed the ram size.")
    }
    Ok(&self.array[address..(address + num_bytes)])
  }

  pub fn write_byte(&mut self, address: usize, byte: u8) -> Result<(), &'static str>{
    if address >= RAM_SIZE{
      return Err("The address exceeds the ram size.")
    }
    self.array[address] = byte;
    Ok(())
  }
  pub fn write_bytes(&mut self, address: usize, bytes: &[u8]) -> Result<(), &'static str>{
    if address + bytes.len() >= RAM_SIZE{
      return Err("The addresses exceed the ram size.")
    }
    for (id, byte) in bytes.iter().enumerate(){
      self.array[address + id] = *byte;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests_for_ram{
  use super::Ram;
  use test_case::test_case;

  #[test_case(0     ,23  ;"Write 23 in address 0." )]
  #[test_case(10    ,30  ;"Write 10 in address 30.")]
  #[test_case(1002  ,90  ;"Write 90 in address 90.")]
  fn read_and_write_byte(address: usize, byte: u8){
    let mut my_ram = Ram::new();

    my_ram.write_byte(address, byte).unwrap();
    assert_eq!(my_ram.read_byte(address).unwrap(), byte);
  }

  #[test_case(0     ,&[132 ,35 ,233] ;"Write 132, 35 and 233, each following other, starting on address 0.")]
  #[test_case(20    ,&[90]           ;"Write 90 starting on address 20."                                   )]
  #[test_case(1000  ,&[101 ,9]       ;"Write 101 and 9, each following other, starting on address 1000."   )]
  fn read_and_write_bytes(address: usize, bytes: &[u8]){
    let mut my_ram = Ram::new();

    my_ram.write_bytes(address, bytes).unwrap();
    assert_eq!(my_ram.read_bytes(address, bytes.len()).unwrap(), bytes);
  }
}