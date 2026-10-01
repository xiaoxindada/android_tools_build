#!/bin/sh
# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

TEST_RUNNING=./TEST.RUNNING.DELME
CGRP_PATH=/dev/cpuctl

# output to parent dir, as our exec dir is arch specific
OUTDIR=../results

#HACK: make sure we're running in the same dir as the script
cd -- "$(dirname -- "$0")"

cleanup () {
	set +e
	rm -rf ./a
	rm -rf ./b
	rmdir $CGRP_PATH/medium
	rmdir $CGRP_PATH/low
	set -e
}

setup_once() {
	if [ "${USER}" != "root" ]; then
		echo User is $USER
		echo "Please run with sudo"
		exit -1
	fi

	echo 1 > /sys/kernel/tracing/tracing_on

	cleanup
	mkdir -p $OUTDIR
}

setup () {
	rm -f $TEST_RUNNING

	mkdir a
	mkdir b
	touch ./a/foreground
	touch ./a/background

	mkdir $CGRP_PATH/medium
	echo "512" > $CGRP_PATH/medium/cpu.shares

	mkdir $CGRP_PATH/low
	echo "4" > $CGRP_PATH/low/cpu.shares

}

something() {
   VAL=5
}

busy_cpu () {
	while [ -f "$TEST_RUNNING" ];
	do
		for i in {1..10000}
		do
			something
		done
	done
}

busy () {
	for i in `seq 1 $(($(nproc)*2))`;
	do
		busy_cpu &
	done
}

run_test () {
	setup
	touch $TEST_RUNNING

	if [ "$BACKGROUND" = "true" ]; then
		./rename-test ./a/background ./b/background &
		BACKGROUND_PID=$!
		if [ "$THROTTLED" = "true" ]; then
			echo $BACKGROUND_PID > $CGRP_PATH/low/cgroup.procs
		fi
	fi
	if [ "$BUSY" = "true" ]; then
		busy &
		BUSY_PID=$!
		if [ "$THROTTLED" = "true" ]; then
			echo $BUSY_PID > $CGRP_PATH/medium/cgroup.procs
		fi
	fi


	./rename-test -p ./a/foreground ./b/foreground > $OUTDIR/$OUT &
	FOREGROUND_PID=$!

	echo "I|$$|Test Begins" > /sys/kernel/tracing/trace_marker

	sleep 30

	echo "I|$$|Test Finished" > /sys/kernel/tracing/trace_marker

	kill -9 $FOREGROUND_PID
	if [ "$BACKGROUND" = "true" ]; then
		kill -9 $BACKGROUND_PID
	fi
	rm -f $TEST_RUNNING

	sort -n -o $OUTDIR/$OUT $OUTDIR/$OUT
	rm -f $OUTDIR/$OUT.gz
	gzip $OUTDIR/$OUT
	cleanup
}

setup_once

# Capture the results of the test running alone
#echo "Running test alone..."
#OUT=results-alone.log
#BACKGROUND="false"
#BUSY="false"
#THROTTLED="false"
#run_test

# Capture the results of the test running with VFS load (to create lock contention)
#echo "Running test with contention..."
#OUT=results-contention.log
#BACKGROUND="true"
#BUSY="false"
#THROTTLED="false"
#run_test

# Capture the results of the test running with scheduler load
#echo "Running test with sched load..."
#OUT=results-schedload.log
#BACKGROUND="false"
#BUSY="true"
#THROTTLED="false"
#run_test

# Capture the results of the test running with scheduler load & contention
echo "Running test with sched load & contention..."
OUT=results-schedload-contention.log
BACKGROUND="true"
BUSY="true"
THROTTLED="false"
run_test

# Capture the results of the test running w/ contetion throttled
#echo "Running test with throttled contention..."
#OUT=results-contention-throttled.log
#BACKGROUND="true"
#BUSY="false"
#THROTTLED="true"
#run_test

# Capture the results of the test running with scheduler load throttled
#echo "Running test with throttled sched load..."
#OUT=results-schedload-throttled.log
#BACKGROUND="false"
#BUSY="true"
#THROTTLED="true"
#run_test

# Capture the results of the test running with scheduler load & contention throttled
echo "Running test with throttled sched load & contention..."
OUT=results-schedload-contention-throttled.log
BACKGROUND="true"
BUSY="true"
THROTTLED="true"
run_test



