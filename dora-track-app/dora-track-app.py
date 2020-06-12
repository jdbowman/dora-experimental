#!/usr/bin/env python3

import argparse
import app_api
import sys
import datetime
import time
import math
from hw_scmd import scmd

class TrackApp(object):

    # Motor drive flags
    R_MTR = 0
    L_MTR = 1
    FWD = 0
    BWD = 1

    # Setup the environment
    app_name = "dora-track-app";

    def __init__(self):
        '''
        Initialize the hardware and logging
        '''

        # Get the configuration
        parser = argparse.ArgumentParser()
        parser.add_argument('--config', '-c')
        args = parser.parse_args()

        if args.config is not None:
            global SERVICES
            SERVICES = app_api.Services(args.config)
        else:
            SERVICES = app_api.Services()

        # Setup logging
        self.logger = app_api.logging_setup(self.app_name);

        # Connect to tracking motor
        self.motor = scmd(1);

        print('I2C bus: ', self.motor.bus);
        print('I2C slave address: ', self.motor.addr);
        print('SCMD ID: ', self.motor.begin());
               
        self.motor.ready();
        self.motor.disable();
        time.sleep(0.25);
        
        self.motor.set_drive(0, 0, 0);
        self.motor.set_drive(1, 0, 0);
        self.motor.enable();
        time.sleep(0.25)

        # Make sure DORA is ready to go
        self.powerOnSensors();


    def __del__(self):
        self.motor.disable();
        self.powerOffSensors();


    def track(self):
        '''
        Track a bright light using the dora-service to provide angle of arrival
        '''    
        count = 0;
        while count<1000:

            count = count + 1;
            request = '{telemetry{angleOfArrival}}';

            try:
                response = SERVICES.query(service="dora-service", query=request)
            except Exception as e:
                self.logger.error("Something went wrong with query to dora-service: " + str(e))
                sys.exit(1)

            data = response["telemetry"]
            angle = data["angleOfArrival"]

            # Calculate motor speed and direction
            speed = abs(angle/(math.pi/2) * 120) + 30;
            direction = 0 if (angle<=0) else 1;

            print('(', count, ')', 'Angle: ', angle, ', Speed: ', speed, ', Direction: ', direction);

            self.motor.set_drive(self.L_MTR, direction, speed);
            time.sleep(0.08);


    def powerOnSensors(self):
        
        self.logger.info("Powering on DORA sensors - %s" % (str(datetime.datetime.now())));
        request = '''
            mutation {
                powerOn(power:true){ status } 
                integrationTime(flag:0) { status }
            }
            ''';

        try:
            response = SERVICES.query(service="dora-service", query=request)
            return True;

        except Exception as e:
            self.logger.error("Something went wrong with mutation to dora-service: " + str(e))
            return False;


    def powerOffSensors(self):
        
        self.logger.info("Powering off DORA sensors - %s " % (str(datetime.datetime.now())));
        request = '''
            mutation {
                powerOn(power:false){ status } 
            }
            ''';

        try:
            response = SERVICES.query(service="dora-service", query=request)
            return True;

        except Exception as e:
            self.logger.error("Something went wrong with mutation to dora-service: " + str(e))
            return False;


if __name__ == "__main__":
    tracker = TrackApp();
    tracker.track();