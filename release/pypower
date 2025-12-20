#!/usr/bin/env python3

import subprocess
import psutil
import time

warning_level = 20
critical_level = 5
check_interval = 2


def get_battery_info():
    try:
        battery = psutil.sensors_battery()
        if battery is None:
            exit(1)
    except Exception:
        exit(1)
    return battery.power_plugged, int(battery.percent)


def notify(message, urgency="normal", title="Battery"):
    subprocess.run([
        "notify-send",
        "-u", urgency,
        title,
        message
    ])


last_plugged = get_battery_info()[0]
last_percent = get_battery_info()[1]

if last_plugged:
    if last_percent == 100:
        notify("Plugged in: battery full")
    else:
        notify(f"Charging: {last_percent}%")


if last_percent <= warning_level and last_percent > critical_level:
    notify(f"Battery Level: {last_percent}%")
elif last_percent < critical_level:
    notify(f"CRITICAL Battery Level: {last_percent}%", "critical")


while True:
    plugged = get_battery_info()[0]
    current_percent = get_battery_info()[1]

    if plugged != last_plugged:
        if plugged:
            notify(f"Charging: {current_percent}%")
        else:
            notify(f"Unplugged: {current_percent}%")
        last_plugged = plugged

    if current_percent != last_percent:
        if current_percent == warning_level:
            notify(f"Battery Level: {current_percent}%")
        elif current_percent == critical_level:
            notify(f"CRITICAL Battery Level: {current_percent}%")
        last_percent = current_percent

    time.sleep(check_interval)
