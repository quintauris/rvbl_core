architecture RISCV_BASE
{
  endianness
  {
    little;
  }

  space linear
  {
    id = 1;
    mau = 8;
    map (size = 4G, dest = bus:local_bus );

    copytable
    (
      align = 4,
      dest = linear
    );

    start_address
    (
      symbol = "_start"
    );
  }

  bus local_bus
  {
    mau = 8;
    width = 32;
  }
}

architecture RV32I extends RISCV_BASE
{
}

derivative rv32i
{
  bus main_bus
  {
    mau = 8;
    width = 32;
    map (size=4G, dest=bus:rv32i:local_bus);
  }
  core rv32i
  {
    architecture = RV32I;
  }
}

processor rv32i
{
  derivative = rv32i;
}
