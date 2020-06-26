# The dora-experimental repository
Codes for DORA experimental flatsat development using the KubOS framework

To gain experience for implementing the DORA cubesat, we have created some trial apps and services using KubOS.  We expect DORA will use an MBM2 with BeagleBone Black onboard computer.

## DORA oveview

The DORA cubesat is a 3U cubesat currently under development (2020) by Arizona State University (ASU) and the Jet Propulsion Laboratory (JPL).  It is funded through NASA's Smallsat Technology Partnerships program.  The payload is a novel deployable optical receive aperture (DORA) for inter-spacecraft communications that will be capable of up to one gigabit per second (Gbps) data rates at distances of 5,000 to 10,000 km. The DORA system enables a large collecting area and eliminates precision pointing accuracy requirements on the host spacecraft. It is ideally suited for crosslink communications among small spacecraft, especially for those forming a swarm and/or a constellation, and for surface to orbit communications. Critically, the DORA system will enable the host spacecraft to overcome constraints imposed by traditional optical communications systems that require high-precision bus pointing of order arcseconds. DORA requires host pointing accuracy of 10°, allowing the primary mission to continue without reorienting to communicate and/or enabling small satellite missions using low-cost off-the-shelf attitude determination and control systems with typical pointing accuracy of 1-10°. The funded project is to design, build, and test a 3U cubesat that contains a DORA demonstration payload that is capable of closing a 1 Gbps link from LEO to a ground station at ASU.  

## Apps in this repository

* dora-health-app: A trial Rust app that gathers basic system health information and saves it to the KubOS telemetery database or transmits it through a KubOS communication layer
* dora-track-app: A trial Python app that queries the dora-sensor-service for angle of arrival information and adjusts uses a motor contoller connected by I2C to steer the sensors so that the angle of arrival is zero.

## Services in this repository

* dora-sensor-service:  A trial Python service that reads from two ambient light detectors over I2C and reports the angle of arrival.

## Other contents

### dev-cmd

This folder contains a couple useful shell scripts for the KubOS Vagrant development environment.  One starts the relevant KubOS services and another runs an infinite loop to provide a measurable CPU load.

### tasks

This folder contains the task list description files for the trial CONOPS operational modes.

