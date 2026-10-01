#!/usr/bin/env python3

# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    https://www.apache.org/licenses/LICENSE-2.0
#
#    Unless required by applicable law or agreed to in writing, software
#    distributed under the License is distributed on an "AS IS" BASIS,
#    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#    See the License for the specific language governing permissions and
#    limitations under the License.

import string
import statistics
import sys
import numpy as np
import matplotlib.pyplot as plt
import matplotlib
import gzip

matplotlib.rcParams.update({'font.family': 'Sans'})
matplotlib.rcParams.update({'font.size': 6})


def open_gz(filename, mode='r'):
    if filename.endswith('.gz'):
        return gzip.open(filename, mode)
    else:
        return open(filename, mode)

def trim_filename(filename):
    MAX_FILE_TITLE=45
    if (len(filename) <= MAX_FILE_TITLE):
        return filename
    return "..."+filename[-(MAX_FILE_TITLE-3):]

allcolors = ["cornflowerblue","firebrick", "gold","forestgreen"]
files = sys.argv
files.pop(0)
chart = []
colors = []
max_val = 0
bins=50

for filename in files:
    datafile = open_gz(filename,"r")
    data = []
    for line in datafile.readlines():
        line = line.strip()
        nsec = float(line)
        msec = nsec/1000000
        data.append(msec)
        if (msec > max_val):
            max_val = msec

    chart.append(data)
    colors.append(allcolors.pop(0))

avg = []
median = []
for data in chart:
    a = statistics.mean(data)
    m = statistics.median(data)
    avg.append(a)
    median.append(m)
    print("avg: " + str(a) + "\tmed: "+ str(m) + "\tmax: "+str(max(data)))


# the histogram of the data
plt.hist(chart, range=(0,max_val), bins=bins, log=True, color=colors, histtype='bar', rwidth=0.9, label=files)

plt.xlabel("msecs")
plt.ylabel("count")
plt.savefig('chart.png', dpi=500)
plt.clf()

if (len(files) > 1):
    if (len(files) > 2):
        fig, axs = plt.subplots(nrows=2,ncols=2)
        ((ax0,ax1),(ax2,ax3)) = axs
        plt.setp(axs[-1, :], xlabel='msecs')
        plt.setp(axs[:, 0], ylabel='count')
    else:
        fig, axs = plt.subplots(ncols=2)
        (ax0,ax1) = axs
        plt.setp(axs[:], xlabel='msecs')
        plt.setp(axs[:], ylabel='count')


    file_title = trim_filename(files[0])
    ax0.hist(chart[0], range=(0,max_val), bins=bins, log=True, color=colors[0], histtype='bar', rwidth=0.8, label=file_title)
    ax0.set_title(file_title)

    x = 0.75
    y = 0.8
    unit = "ms"
    mult = 1
    if (avg[0] < 1):
        mult = 1000
        unit = "us"
    label = "avg: " + "{:.2f}".format(avg[0]*mult) + unit +"\nmed: " + "{:.2f}".format(median[0]*mult) + unit
    ax0.text(x,y, label, transform=ax0.transAxes)

    file_title = trim_filename(files[1])
    ax1.hist(chart[1], range=(0,max_val), bins=bins, log=True, color=colors[1], histtype='bar', rwidth=0.8, label=file_title)
    ax1.set_title(file_title)
    label = "avg: " + "{:.2f}".format(avg[1]*mult) + unit +"\nmed: " + "{:.2f}".format(median[1]*mult) + unit
    ax1.text(x,y, label, transform=ax1.transAxes)

    if (len(files) > 2):
        file_title = trim_filename(files[2])
        ax2.hist(chart[2], range=(0,max_val), bins=bins, log=True, color=colors[2], histtype='bar', rwidth=0.8, label=file_title)
        ax2.set_title(file_title)
        label = "avg: " + "{:.2f}".format(avg[2]*mult) + unit +"\nmed: " + "{:.2f}".format(median[2]*mult) + unit
        ax2.text(x,y, label, transform=ax2.transAxes)
    if (len(files) > 3):
        file_title = trim_filename(files[3])
        ax3.hist(chart[3], range=(0,max_val), bins=bins, log=True, color=colors[3], histtype='bar', rwidth=0.8, label=file_title)
        ax3.set_title(file_title)
        label = "avg: " + "{:.2f}".format(avg[3]*mult) + unit +"\nmed: " + "{:.2f}".format(median[3]*mult) + unit
        ax3.text(x,y, label, transform=ax3.transAxes)

    #There has to be a better way to fix the y-axis to be same on all charts
    top=0
    bottom=sys.float_info.max
    for ax_tmp in fig.axes:
        (low,high) = ax_tmp.get_ylim()
        if (high > top):
            top=high
        if (low < bottom):
            bottom = low
    for ax_tmp in fig.axes:
        ax_tmp.set_ylim(ymin=bottom,ymax=top)

    plt.savefig('chart-split.png', dpi=500)

