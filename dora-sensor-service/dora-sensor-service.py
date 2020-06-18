#!/usr/bin/env python3

# Copyright 2020 Judd Bowman
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Based on the KubOS example for service applications
"""

from service import schema
from kubos_service.config import Config
from kubos_service import http_service

# Get the configuration (uses command line argument -c if present)
config = Config("dora-sensor-service")

# Start an http service
http_service.start(config, schema.schema)



#from kubos_service import udp_service

# Start a udp service with optional context
# udp_service.start(config, schema, {'bus': '/dev/ttyS3'})

# Start a udp service
#udp_service.start(logger, config, schema)
