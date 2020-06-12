# dora-service 

This is a KubOS service that provides angle of arrival information derived from the ambient light sensors on two VCNL4040 boards.  This code has been tested on a Beagle Bone Black (BBB) running KubOS 1.20.  The service is implemented in Python 3.

## Dependencies 

Dependencies are given in requirements.txt.  All but smbus2 are part of the standard KubOS linux build.

## To install the service on a BBB running KubOS:

1. Download and install the python-smbus2 package on the BBB.  KubOS does not include PIP so install this package by unpacking it and then from inside its top directory calling: _python setup.py install_
2. Copy this directory (dora-service) to /home/system/usr/bin on the BBB
3. Copy the S99dora-service.sh file to /home/system/init.d on the BBB and make sure it is executable by root
4. Copy or add the contents of the config.toml file to /home/system/etc/config.toml
5. The service should launch in the background on the next reboot of the BBB.

## Using the service

You can interact with the service using the standard KubOS TCP/IP interface.  The service will listen on the port specified in its config.toml section.  Supported queries and mutations are implemented in the services/schema.py file.  They include:

### Queries

1. ping - retuns a string containing "pong"
2. config - returns a string with information about the service configuration
3. power - returns the following: 
    _state_ (Boolean): is true if the ambient sensors are actived and false otherwise.  
    _uptime_ (integer): provides the number of seconds since the service was started.
4. telemetry - returns the following:
    _connected_ (Boolean): is true if there has been successful I2C communication to the sensors
    _powerOn_ (Boolean): is true if the ambient sensors are actived and false otherwise.
    _integrationTime_ (integer enum): a flag value representing one of four ambient sensor integration times (0=80ms, 1=160ms, 2=320ms, and 3=640ms)
    _sensor1Value_ (integer): ambient light level returned by sensor 1
    _sensor2Value_ (integer): ambient light level returnd by sensor 2
    _angleOfArrival_ (float): approximate angle of arrival in radians dervied from the sensors assuming a cosine response and no background light.


