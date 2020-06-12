# dora-service 

This is a KubOS service that provides angle of arrival information derived from the ambient light sensors on two VCNL4040 boards.  This code has been tested on a Beagle Bone Black (BBB) running KubOS 1.20.  The service is implemented in Python 3.

## Dependencies 

Dependencies are given in requirements.txt.  All but smbus2 are part of the standard KubOS linux build.

## Installation onto a BBB running KubOS:

1. Download and install the python-smbus2 package on the BBB.  KubOS does not include PIP so install this package by unpacking it and then from inside its top directory calling: _python setup.py install_
2. Copy this directory (dora-service) to /home/system/usr/bin on the BBB
3. Copy the S99dora-service.sh file to /home/system/init.d on the BBB and make sure it is executable by root
4. Copy or add the contents of the config.toml file to /home/system/etc/config.toml
5. The service should launch in the background on the next reboot of the BBB.

## Usage

You can interact with the service using the standard KubOS TCP/IP interface.  The service will listen on the port specified in its config.toml section.  Supported queries and mutations are implemented in the service/schema.py file.  They include:

### Queries

1. ping - retuns a string containing "pong"
2. config - returns a string with information about the service configuration
3. power - returns the following: 
   - _state_ (Boolean): is true if the ambient sensors are actived and false otherwise.  
   - _uptime_ (integer): provides the number of seconds since the service was started.
4. telemetry - returns the following:
   - _connected_ (Boolean): is true if there has been successful I2C communication to the sensors
   - _powerOn_ (Boolean): is true if the ambient sensors are actived and false otherwise.
   - _integrationTime_ (integer enum): a flag value representing one of four ambient sensor integration times (0=80ms, 1=160ms, 2=320ms, and 3=640ms)
   - _sensor1Value_ (integer): ambient light level returned by sensor 1
   - _sensor2Value_ (integer): ambient light level returnd by sensor 2
   - _angleOfArrival_ (float): approximate angle of arrival in radians dervied from the sensors assuming a cosine response and no background light.

### Mutations

1. noop()
   - Returns: _status_ (Boolean) and a copy of the last _telemetry_ query (see Queries section above for details).  Note that a new telemetry query is not actually executed, so the sensor and angle of arrival values will not be up to date in the returned telemetry structure.
2. commandRaw(sensor, command, flag, lsb, msb)
   - _sensor_ (integer enum): 1 or 2, specifies the sensor to send the command to
   - _command_ (integer): one-byte command value (register address on the VCNL4040 board).  See service/hw_vcnl4040.py for list of command values.
   - _flag_ (integer enum): 0=read from register, 1=write to register
   - _lsb_ and _msb_ (integers): Only used when writing (flag=1).  Each register on the VCNL4040 boards contains two bytes.  _lsb_ is the value that should be written to the least significant byte.  _msb_ is the value that should be written to the most significant byte.
   - This command returns:  _status_ (Boolean) and  _data_ (integer).  The response _data_ is only non-zero for read commands and contains the value of the lst and msb in the register.
3. powerOn(power) 
   - Set _power_ (Boolean) to True or False to activate the ambient light sensors.  
   - Returns _status_ and _telemetry_ with the same caveats as the _noop_ response.
4. integrationTime(flag)
   - Set _flag_ (integer enum) to 0, 1, 2, 3 to configure the ambient light sensors to integrate for 80ms, 160ms, 320ms, and 640ms respectively.
   - Returns _status_ and _telemetry_ with the same caveats as the _noop_ response.

## Logging

The service logs many of its actions through the KubOS logging functions, which are similar to syslog.  The KubOS log files are in /home/system/log and you can expect to find dora-service messages in the file kubos-info.log.
