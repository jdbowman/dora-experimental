#!/usr/bin/env python3

# Copyright 2020 Judd Bowman
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
This file implements the GraphQL definitions for the DORA service.  For the most part,
it follows the KubOS service outline for required functionality, see:

https://docs.kubos.com/1.21.0/ecosystem/services/service-outline-guide.html

"""

import graphene
from .models import DoraSubsystem, PowerState, RawCommandResponse, Status, Telemetry


# API for the DORA subsystem
_subsystem = DoraSubsystem()


class Query(graphene.ObjectType):
    """
    Creates query endpoints exposed by graphene.
    """
    ping = graphene.String()
    config = graphene.String()
    power = graphene.Field(PowerState)
    telemetry = graphene.Field(Telemetry)     

    def resolve_ping(self, info):
        return "pong"

    def resolve_config(self, info):
        return _subsystem.getConfig()

    def resolve_power(self, info):
        return _subsystem.getPower()

    def resolve_telemetry(self, info):
        _subsystem.refresh()
        return _subsystem.telemetry



class noop(graphene.Mutation):
    """
    Creates mutation for DoraSubsystem.noop
    """
    class Arguments:
        pass

    Output = Status

    def mutate(self, info):
        """
        Handles request for nnop
        """
        return Status(status=_subsystem.noop(), telemetry=_subsystem.telemetry)



class commandRaw(graphene.Mutation):
    """
    Creates mutation for DoraSubsystem.commandRaw
    """

    class Arguments:
        sensor = graphene.Int() # 1 = sensor1, 2 = sesnor2
        command = graphene.Int()
        flag = graphene.Int()   # 0 = read, 1 = write
        lsb = graphene.Int()
        msb = graphene.Int()

    Output = RawCommandResponse

    def mutate(self, info, sensor, command, flag, lsb, msb):
        """
        Handles raw command
        """
        if sensor is not None and command is not None and flag is not None and lsb is not None and msb is not None:
            return _subsystem.commandRaw(sensor, command, flag, lsb, msb)
        else:
            return RawCommandResponse(status=True, data=0)



class powerOn(graphene.Mutation):
    """
    Creates mutation for DoraSubsystem.powerOn
    """

    class Arguments:
        power = graphene.Boolean() 

    Output = Status

    def mutate(self, info, power):
        """
        Handles request for powerOn mutation
        """
        if power is not None:
            return _subsystem.setPower(power)
        else:
            return Status(status=True, telemetry=_subsystem.telemetry)



class integrationTime(graphene.Mutation):
    """
    Creates mutation for DoraSubsystem.integrationTime
    """

    class Arguments:
        flag = graphene.Int();                 # These parameter names are used in the graphql mutation request

    Output = Status

    def mutate(self, info, flag):
        """
        Handles request for integrationTime mutation
        """
        if flag is not None:
            return _subsystem.setIntegrationTime(flag)
        else:
            return Status(status=True, telemetry=_subsystem.telemetry)


class Mutation(graphene.ObjectType):
    """
    Creates mutation endpoints exposed by graphene.  
    Converts variable names to CamelCase to use as entry points.
    """

    noop = noop.Field();                        # These parameter names are used in the graphql mutation request
    command_raw = commandRaw.Field();           # These parameter names are used in the graphql mutation request
    power_on = powerOn.Field();                 # These parameter names are used in the graphql mutation request
    integration_time = integrationTime.Field(); # These parameter names are used in the graphql mutation request


schema = graphene.Schema(query=Query, mutation=Mutation)
