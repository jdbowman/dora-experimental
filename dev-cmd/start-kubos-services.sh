#! /usr/bin/env bash

cd /home/vagrant/kubos

cargo run --bin monitor-service -- -c /home/vagrant/kubos/tools/local_config.toml &

cargo run --bin telemetry-service -- -c /home/vagrant/kubos/tools/local_config.toml &

cargo run --bin kubos-app-service -- -c /home/vagrant/kubos/tools/local_config.toml &

cargo run --bin shell-service -- -c /home/vagrant/kubos/tools/local_config.toml &

cargo run --bin file-service -- -c /home/vagrant/kubos/tools/local_config.toml &

cargo run -p scheduler-service -- -c /home/vagrant/kubos/tools/local_config.toml &