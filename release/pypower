#!/usr/bin/env python3

import subprocess
import psutil
import time

WARNING_LEVEL = 20
CRITICAL_LEVEL = 5
CHECK_INTERVAL = 2


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


initial_plugged, initial_percent = get_battery_info()

if initial_plugged:
    if initial_percent == 100:
        notify("Plugged in: battery full")
    else:
        notify(f"Charging: {initial_percent}%")

else:
    if initial_percent <= WARNING_LEVEL and initial_percent > CRITICAL_LEVEL:
        notify(f"Battery Level: {initial_percent}%")
    elif initial_percent <= CRITICAL_LEVEL:
        notify(f"CRITICAL Battery Level: {initial_percent}%", "critical")


while True:
    plugged, current_percent = get_battery_info()

    if plugged != initial_plugged:
        if plugged:
            if current_percent == 100:
                notify("Plugged in: battery full")
            else:
                notify(f"Plugged in: Charging: {current_percent}%")
        else:
            notify(f"Unplugged: {current_percent}%")
        initial_plugged = plugged

    if current_percent != initial_percent:
        if not plugged:
            if current_percent == WARNING_LEVEL:
                notify(f"Battery Level: {current_percent}%")
            elif current_percent == CRITICAL_LEVEL:
                notify(f"CRITICAL Battery Level: {current_percent}%",
                       "critical")
        initial_percent = current_percent

    time.sleep(CHECK_INTERVAL)
