# dora-service 

This is a KubOS service that provides angle of arrival information derived from the ambient light sensors on two NCVL4040 boards.  This code has been tested on a Beagle Bone Black (BBB) running KubOS 1.20.  The service is implemented in Python 3.

## Dependencies 

Dependencies are given in requirements.txt.  All but smbus2 are part of the standard KubOS linux build.

## To install the service on the BBB:

1. Download and install the python-smbus2 package on the BBB.  KubOS does not include PIP so install this package by unpacking it and then from inside its top directory calling: _python setup.py install_
2. Copy this directory to /home/system/usr/bin on the BBB
3. Copy the S99dora-service.sh file to /home/system/init.d on the BBB and make sure it is executable by root
4. The service should launch in the background on the next reboot of the BBB.




