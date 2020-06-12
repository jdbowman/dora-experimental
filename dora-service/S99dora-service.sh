#!/bin/sh

# Start the DORA sensors service in the background 
# passing in the location of the config.toml file
python /home/system/usr/bin/dora-service/dora-service.py -c /home/system/etc/config.toml &

exit 0