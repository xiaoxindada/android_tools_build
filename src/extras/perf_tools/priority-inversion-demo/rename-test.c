/*
 * Copyright 2022 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * rename-test.c:
 *  Renames a file back and forth repeatedly timing how long it takes
 *
 * Rewritten from a test by Connor O'Brien <connoro@google.com>
 */
#include <stdio.h>
#include <stdbool.h>
#include <unistd.h>
#include <time.h>
#include <sys/prctl.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <string.h>
#include <stdlib.h>

#define NUM_VALS 1024

#define TEST_RUNNING_FILE "./TEST.RUNNING.DELME"

#define NSEC_PER_SEC 1000000000ULL
unsigned long long ts_to_ns(struct timespec ts)
{
	return ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec;
}

int main(int /*argc*/, char** argv)
{
	unsigned long long *delta_values;
	int i = 1;
	char* first;
	char* second;
	bool display = false;
	int count = 0;

	delta_values = malloc(sizeof(unsigned long long) * NUM_VALS);
	if (!delta_values) {
		printf("Error: ENOMEM\n");
		return -1;
	}

	/* TODO: better arg parsing */
	if (!strncmp(argv[i], "-p", 2)) {
		prctl(PR_SET_NAME, "foreground", 0, 0, 0);
		display = true;
		i++;
	}else
		prctl(PR_SET_NAME, "background", 0, 0, 0);

	first = argv[i];
	second = argv[i+1];

	while(!access(TEST_RUNNING_FILE, F_OK)){
		struct timespec start, stop;

		/* time rename */
		clock_gettime(CLOCK_MONOTONIC, &start);
		rename(first, second);
		clock_gettime(CLOCK_MONOTONIC, &stop);

		/* reset */
		rename(second, first);

		if (display) {
			delta_values[count++] =  ts_to_ns(stop) - ts_to_ns(start);
			/*
			 * print the whole array once its full.
			 * The idea being we don't preturb the test by printing
			 * each iteration and ending up serializing the behaivor
			 * on console output timing.
			 */
			if (count >= NUM_VALS) {
				int j;
				for (j = 0; j < NUM_VALS; j++)
					printf("%llu\n", delta_values[j]);
				count = 0;
			}
		}
	}
	free(delta_values);
	return 0;
}

