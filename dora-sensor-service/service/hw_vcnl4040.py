from smbus2 import SMBus, i2c_msg
import struct

class vcnl4040:

  # VCNL4040 default I2C slave address
  DEFAULT_ADDR = 0x60;

  # VCNL4040 Command Codes
  ALS_CONF = b'\x00';
  ALS_THDH = b'\x01';
  ALS_THDL = b'\x02';
  PS_CONF1 = b'\x03';  # Lower
  PS_CONF2 = b'\x03';  # Upper
  PS_CONF3 = b'\x04';  # Lower
  PS_MS = b'\x04';     # Upper
  PS_CANC = b'\x05';
  PS_THDL = b'\x06';
  PS_THDH = b'\x07';
  PS_DATA = b'\x08';
  ALS_DATA = b'\x09';
  WHITE_DATA = b'\x0A';
  INT_FLAG = b'\x0B';  # Upper
  ID = b'\x0C';

  # VCNL4040 Command Data Values
  ALS_IT_MASK = ~((1 << 7) | (1 << 6));
  ALS_IT_80MS = 0;
  ALS_IT_160MS = (1 << 7);
  ALS_IT_320MS = (1 << 6);
  ALS_IT_640MS = (1 << 7) | (1 << 6);

  ALS_PERS_MASK = ~((1 << 3) | (1 << 2));
  ALS_PERS_1 = 0;
  ALS_PERS_2 = (1 << 2);
  ALS_PERS_4 = (1 << 3);
  ALS_PERS_8 = (1 << 3) | (1 << 2);

  ALS_INT_EN_MASK = ((1 << 1));
  ALS_INT_DISABLE = 0;
  ALS_INT_ENABLE = (1 << 1);

  ALS_SD_MASK = ~((1 << 0));
  ALS_SD_POWER_ON = 0;
  ALS_SD_POWER_OFF = (1 << 0);

  PS_DUTY_MASK = ~((1 << 7) | (1 << 6));
  PS_DUTY_40 = 0;
  PS_DUTY_80 = (1 << 6);
  PS_DUTY_160 = (1 << 7);
  PS_DUTY_320 = (1 << 7) | (1 << 6);

  PS_PERS_MASK = ~((1 << 5) | (1 << 4));
  PS_PERS_1 = 0;
  PS_PERS_2 = (1 << 4);
  PS_PERS_3 = (1 << 5);
  PS_PERS_4 = (1 << 5) | (1 << 4);

  PS_IT_MASK = ~((1 << 3) | (1 << 2) | (1 << 1));
  PS_IT_1T = 0;
  PS_IT_15T = (1 << 1);
  PS_IT_2T = (1 << 2);
  PS_IT_25T = (1 << 2) | (1 << 1);
  PS_IT_3T = (1 << 3);
  PS_IT_35T = (1 << 3) | (1 << 1);
  PS_IT_4T = (1 << 3) | (1 << 2);
  PS_IT_8T = (1 << 3) | (1 << 2) | (1 << 1);

  PS_SD_MASK = ~((1 << 0));
  PS_SD_POWER_ON = 0;
  PS_SD_POWER_OFF = (1 << 0);

  PS_HD_MASK = ~((1 << 3));
  PS_HD_12_BIT = 0;
  PS_HD_16_BIT = (1 << 3);

  PS_INT_MASK = ~((1 << 1) | (1 << 0));
  PS_INT_DISABLE = 0;
  PS_INT_CLOSE = (1 << 0);
  PS_INT_AWAY = (1 << 1);
  PS_INT_BOTH = (1 << 1) | (1 << 0);

  PS_SMART_PERS_MASK = ~((1 << 4));
  PS_SMART_PERS_DISABLE = 0;
  PS_SMART_PERS_ENABLE = (1 << 1);

  PS_AF_MASK = ~((1 << 3));
  PS_AF_DISABLE = 0;
  PS_AF_ENABLE = (1 << 3);

  PS_TRIG_MASK = ~((1 << 3));
  PS_TRIG_TRIGGER = (1 << 2);

  WHITE_EN_MASK = ~((1 << 7));
  WHITE_ENABLE = 0;
  WHITE_DISABLE = (1 << 7);

  PS_MS_MASK = ~((1 << 6));
  PS_MS_DISABLE = 0;
  PS_MS_ENABLE = (1 << 6);

  LED_I_MASK = ~((1 << 2) | (1 << 1) | (1 << 0));
  LED_50MA = 0;
  LED_75MA = (1 << 0);
  LED_100MA = (1 << 1);
  LED_120MA = (1 << 1) | (1 << 0);
  LED_140MA = (1 << 2);
  LED_160MA = (1 << 2) | (1 << 0);
  LED_180MA = (1 << 2) | (1 << 1);
  LED_200MA = (1 << 2) | (1 << 1) | (1 << 0);

  INT_FLAG_ALS_LOW = (1 << 5);
  INT_FLAG_ALS_HIGH = (1 << 4);
  INT_FLAG_CLOSE = (1 << 1);
  INT_FLAG_AWAY = (1 << 0);


  def __init__(self, bus, addr=None):
    self.bus = bus;
    if addr is None:
    	self.addr = self.DEFAULT_ADDR
    else:
    	self.addr = addr;
  
  	# Initialize some registers
    self.setLEDCurrent(self.LED_140MA);
    self.setIRDutyCycle(self.PS_DUTY_320);
    self.setAmbientIntegrationTime(self.ALS_IT_160MS);
    self.setProxLowThreshold(10);
    self.setProxHighThreshold(1000);

    print('VCNL4040: Initialized');

  
  # Read the sensors ID
  def getID(self):
    return self.read(self.ID);

  # Read the Proximity value
  def getProximity(self):
    data = self.read(self.PS_DATA);
    return data[0] + (data[1]<<8);
  
  # Read the Ambient light value
  def getAmbient(self):
    data = self.read(self.ALS_DATA);
    return data[0] + (data[1]<<8);
  
  # Read the White light value
  def getWhite(self):
    data = self.read(self.WHITE_DATA);
    return data[0] + (data[1]<<8);

  def setIRDutyCycle(self, value):
    return self.writeLower(self.PS_CONF1, self.PS_DUTY_MASK, value);

  def setProxInterruptPersistance(self, value):
    return self.writeLower(self.PS_CONF1, self.PS_PERS_MASK, value);

  def setAmbientInterruptPersistance(self, value):
    return self.writeLower(self.ALS_CONF, self.ALS_PERS_MASK, value);

  def enableAmbientInterrupts(self):
    return self.writeLower(self.ALS_CONF, self.ALS_INT_EN_MASK, self.ALS_INT_ENABLE);

  def disableAmbientInterrupts(self):
    return self.writeLower(self.ALS_CONF, self.ALS_INT_EN_MASK, self.ALS_INT_DISABLE);

  def powerOnAmbient(self):
    return self.writeLower(self.ALS_CONF, self.ALS_SD_MASK, self.ALS_SD_POWER_ON);

  def powerOffAmbient(self):
    return self.writeLower(self.ALS_CONF, self.ALS_SD_MASK, self.ALS_SD_POWER_OFF);

  def setAmbientIntegrationTime(self, value):
    return self.writeLower(self.ALS_CONF, self.ALS_IT_MASK, value);

  def setProxIntegrationTime(self, value):
    return self.writeLower(self.PS_CONF1, self.PS_IT_MASK, value);

  def powerOnProximity(self):
    return self.writeLower(self.PS_CONF1, self.PS_SD_MASK, self.PS_SD_POWER_ON);

  def powerOffProximity(self):
    return self.writeLower(self.PS_CONF1, self.PS_SD_MASK, self.PS_SD_POWER_OFF);

  def setProxResolution(self, value):
    return self.writeUpper(self.PS_CONF2, self.PS_HD_MASK, value);

  def setProxInterruptType(self, value):
    return self.writeUpper(self.PS_CONF2, self.PS_INT_MASK, value);

  def enableSmartPersistance(self):
    return self.writeLower(self.PS_CONF3, self.PS_SMART_PERS_MASK, self.PS_SMART_PERS_ENABLE); 

  def disableSmartPersistance(self):
    return self.writeLower(self.PS_CONF3, self.PS_SMART_PERS_MASK, self.PS_SMART_PERS_DISABLE); 

  def enableActiveForceMode(self):
    return self.writeLower(self.PS_CONF3, self.PS_AF_MASK, self.PS_AF_ENABLE); 

  def disableActiveForceMode(self):
    return self.writeLower(self.PS_CONF3, self.PS_AF_MASK, self.PS_AF_DISABLE); 

  def takeSingleProxMeasurement(self):
    return self.writeLower(self.PS_CONF3, self.PS_TRIG_MASK, self.PS_TRIG_TRIGGER); 

  def enableWhiteChannel(self):
    return self.writeUpper(self.PS_MS, self.WHITE_EN_MASK, self.WHITE_ENABLE); 

  def disableWhiteChannel(self):
    return self.writeUpper(self.PS_MS, self.WHITE_EN_MASK, self.WHITE_DISABLE); 

  def enableProxLogicMode(self):
    return self.writeUpper(self.PS_MS, self.PS_MS_MASK, self.PS_MS_ENABLE); 

  def disableProxLogicMode(self):
    return self.writeUpper(self.PS_MS, self.PS_MS_MASK, self.PS_MS_DISABLE); 

  def setLEDCurrent(self, value):
    return self.writeUpper(self.PS_MS, self.LED_I_MASK, value); 

  def setProxCancellation(self, value):
    value = int(value);
    return self.write(self.PS_CANC,  (value & 0xFF), (value & 0xFF00) >> 8); 

  def setALSHighThreshold(self, value):
    value = int(value);
    return self.write(self.ALS_THDH, (value & 0xFF), (value & 0xFF00) >> 8); 

  def setALSLowThreshold(self, value):
    value = int(value);
    return self.write(self.ALS_THDL, (value & 0xFF), (value & 0xFF00) >> 8);   

  def setProxHighThreshold(self, value):
    value = int(value);
    return self.write(self.PS_THDH, (value & 0xFF), (value & 0xFF00) >> 8);  

  def setProxLowThreshold(self, value):
    value = int(value);
    return self.write(self.PS_THDL, (value & 0xFF), (value & 0xFF00) >> 8);  


  def read(self, command):

    msgw = i2c_msg.write(self.addr, command);
    msgr = i2c_msg.read(self.addr, 2);
    with SMBus(self.bus) as bus:
      bus.i2c_rdwr(msgw, msgr);
      return list(msgr);  
    return [0, 0];


  def write(self, command, lsb, msb):
    msgw = i2c_msg.write(self.addr, bytearray([int.from_bytes(command, byteorder='little'), lsb, msb]));
    with SMBus(self.bus) as bus:
      bus.i2c_rdwr(msgw);
      return True;
    return False;


  def writeLower(self, command, mask, value):
    data = self.read(command);
    if (data[0] == -1):
      return False
    else:
      lsb = data[0] & mask; # zero bits we plan to change
      lsb = lsb | value;
      return self.write(command, lsb, data[1]);


  def writeUpper(self, command, mask, value):
    data = self.read(command);
    if (data[0] == -1):
      return False;
    else:
      msb = data[1] & mask;  # zero bits we plan to change
      msb = msb | value;
      return self.write(command, data[0], msb); 
  



