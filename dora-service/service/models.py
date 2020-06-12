#!/usr/bin/env python3

# Copyright 2017 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
Graphene ObjectType classes for DORA subsystem modeling.
"""
import graphene
import datetime
import time
import math
import logging
from logging.handlers import SysLogHandler
from kubos_service.config import Config
from .hw_vcnl4040 import vcnl4040



class DoraSubsystem(object):
    """
    API class encapsulating DORA subsystem functionality.
    """

    app_name = "dora-service"


    def __init__(self):
        """
        Initialize the hardware when the service starts
        """

        # Setup logging
        self.logger = logging.getLogger(self.app_name);
        self.logger.setLevel(logging.DEBUG)
        handler = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_DAEMON)
        formatter = logging.Formatter('{}[%(process)d]: %(message)s'.format(self.app_name))
        handler.formatter = formatter
        self.logger.addHandler(handler)

        # Get the configuration options for the service out of the `config.toml` file
        config = Config(self.app_name);
        sensor1_bus = config.raw['device']['bus1'];
        sensor2_bus = config.raw['device']['bus2'];

        # Create the sensor objects
        self.sensor1 = vcnl4040(sensor1_bus);
        self.sensor2 = vcnl4040(sensor2_bus);

        # Report settings
        print('\n--- Sensor 1 ---');
        print('I2C bus: ', self.sensor1.bus);
        print('I2C slave address: ', self.sensor1.addr);

        print('\n--- Sensor 2 ---');
        print('I2C bus: ', self.sensor2.bus);
        print('I2C slave address: ', self.sensor2.addr);


        self.telemetry = Telemetry(connected=self.noop(), power_on=False, integration_time=0, sensor1_value=0, sensor2_value=0, angle_of_arrival=0);
  

        print('\n--- Initializing ---');
        print('Power off proximity sensor 1: ', self.sensor1.powerOffProximity());
        print('Power off ambient sensor 1: ', self.sensor1.powerOffAmbient());
        print('Set ambient integration time: ', self.sensor1.setAmbientIntegrationTime(self.getIntegrationFlag()));

        print('Power off proximity sensor 2: ', self.sensor2.powerOffProximity());
        print('Power off ambient sensor 2: ', self.sensor2.powerOffAmbient());
        print('Set ambient integration time: ', self.sensor2.setAmbientIntegrationTime(self.getIntegrationFlag()));    

        time.sleep(0.25);

        self.logger.info('Starting DORA sensor service');
        self.logger.info('Listening on: {}:{}'.format(config.ip, config.port));
        self.logger.info('Using i2c bus {} for sensor 1 and bus {} for sensor 2'.format(self.sensor1.bus, self.sensor2.bus));
        self.start_time = datetime.datetime.now();



    def __del__(self):
        """
        Shutdown the hardware when the service ends
        """
        if (self.telemetry.connected):
            print('Power off ambient sensor 1: ', self.sensor1.powerOffAmbient());
            print('Power off ambient sensor 2: ', self.sensor2.powerOffAmbient());

        self.logger.info('Stopped');



    def getConfig(self):
        """
        Returns the service configuration string
        """        
        return 'name={}, bus1={}, bus2={}'.format(self.app_name, self.sensor1.bus, self.sensor2.bus)


    def getPower(self):
        """
        Returns the internal power state
        """
        return PowerState(1 if self.telemetry.power_on==True else 0, (datetime.datetime.now()-self.start_time).total_seconds())


    def getIntegrationFlag(self):
        """
        Converts simple integration time values (0, 1, 2, 3) to the flags used by the sensors.
        """        
        switcher = {
            0:vcnl4040.ALS_IT_80MS,
            1:vcnl4040.ALS_IT_160MS,
            2:vcnl4040.ALS_IT_320MS,
            3:vcnl4040.ALS_IT_640MS };
        return switcher.get(self.telemetry.integration_time, vcnl4040.ALS_IT_80MS);


    def commandRaw(self, sensor, command, flag, lsb, msb):
        """
        Sends a raw command to the specified sensor and returns the response

        Arguments: 
            sensor: integer of value 1 = sensor 1, 2 = sensor 2
            command: integer containing one byte specifying the register to read/write
            flag: integer of value 0 = read, 1 = write
            lsb: Integer containng the least significant byte value (only used for write)
            msb: Itneger contaiing the most significant byte value (only used for write)
        """
        if not (sensor==1 or sensor==1):
            return RawCommandResponse(status=False, data=0);

        if not (flag==0 or flag==1):
            return RawCommandResponse(status=False, data=0);

        # Get the sensor handle
        device = self.sensor1 if sensor==1 else 2;

        status = True;
        data = 0;

        if flag==0:
            data = int.from_bytes(device.read(bytes([command])), byteorder='little')

        elif flag == 1:
            status = device.write(bytes([command]), lsb, msb);

        return RawCommandResponse(status=status, data=data);



    def noop(self):
        """
        Sends 'noop' (get ID) commands to each sesnor and reports True if both returned successfully
        """
        sensor1_connected = True if (self.sensor1.getID()[0] == 0x86) else False;        
        sensor2_connected = True if (self.sensor2.getID()[0] == 0x86) else False;
        return True if (sensor1_connected and sensor2_connected) else False;


    def refresh(self):
        """
        Refreshes the status of the DORA subsystem
        model based on queries to the actual hardware.
        """
        if (self.telemetry.connected):
            print("Querying DORA sensors")
            self.telemetry.sensor1_value = self.sensor1.getAmbient();
            self.telemetry.sensor2_value = self.sensor2.getAmbient();
            self.telemetry.angle_of_arrival = math.atan2(self.telemetry.sensor2_value, self.telemetry.sensor1_value) - math.pi/4;
        else:
            print("Cannot query DORA sensors(s) because not connected")


    def setPower(self, power_on):
        """
        Controls the power state of the DORA subsystem sensors
        Arguments:
                power_on: true to turn on, false to turn off the ambient light sensor
        """
        print("Sending new power state to DORA subsystem")
        print("Previous State: {}".format(self.telemetry.power_on))
        print("New State: {}".format(power_on))
        self.telemetry.power_on = power_on;
        if power_on:
          self.sensor1.powerOnAmbient();
          self.sensor2.powerOnAmbient();
          self.logger.info('Power on.');
        else:
          self.sensor1.powerOffAmbient();
          self.sensor2.powerOffAmbient();
          self.logger.info('Power off.');

        return Status(status=True, telemetry=self.telemetry)


    def setIntegrationTime(self, integration_time):
        """
        Controls the integration time of the DORA subsystem sensors 
        Arguments:
            integration_time: a flag with value 0, 1, 2, or 3 corresponding 
                              to 80, 160, 320, or 640 ms integrations
        """
        print("Sending new sensor integration time to subsystem");
        print("Previous State: {}".format(self.telemetry.integration_time));
        print("New State Requested: {}".format(integration_time));

        if (integration_time > 3 or integration_time < 0): 
          print("Failed.  Requested integration time flag out of bounds.  Must be 0, 1, 2, or 3.");
          self.logger.warn('Set integration time called with out of bounds flag.');
          return Status(status=False, telemetry=self.telemetry)
        else: 
          self.telemetry.integration_time = integration_time;

          print("Success. New State Used: {}".format(self.telemetry.integration_time));
          self.sensor1.setAmbientIntegrationTime(self.getIntegrationFlag());
          self.sensor2.setAmbientIntegrationTime(self.getIntegrationFlag());
          self.logger.info('Integration time set to flag={}'.format(self.telemetry.integration_time));
          return Status(status=True, telemetry=self.telemetry)




class Telemetry(graphene.ObjectType):
    """
    Model representing telemetry response from the DORA service
    """
    # Exposed response fields     
    connected = graphene.Boolean();
    power_on = graphene.Boolean();
    integration_time = graphene.Int();
    sensor1_value = graphene.Int();
    sensor2_value = graphene.Int();
    angle_of_arrival = graphene.Float();



class Status(graphene.ObjectType):
    """
    Model representing execution status. This allows us to return
    the status of the mutation function alongside the state of
    the model affected.
    """
    # Exposed response fields
    status = graphene.Boolean();            # These parameter names are used in the graphql mutation response
    telemetry = graphene.Field(Telemetry);  # These parameter names are used in the graphql mutation response



class PowerState(graphene.ObjectType):
    """
    Model representing power status.
    """
    # Exposed response fields    
    state = graphene.Int();
    uptime = graphene.Int();



class RawCommandResponse(graphene.ObjectType):
    """
    Model representing raw command response
    """
    # Exposed response fields    
    status = graphene.Boolean();
    data = graphene.Int();    


