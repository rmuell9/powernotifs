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
        notify("Plugged In: battery full")
    else:
        notify(f"Charging: {initial_percent}%")

else:
    if initial_percent <= WARNING_LEVEL and initial_percent > CRITICAL_LEVEL:
        notify(f"Current level: {initial_percent}%", "normal",
               "Battery - WARNING")
    elif initial_percent <= CRITICAL_LEVEL:
        notify(f"Current level: {initial_percent}%",
               "critical", "Battery - CRITICAL")


while True:
    plugged, current_percent = get_battery_info()

    if plugged != initial_plugged:
        if plugged:
            if current_percent == 100:
                notify("Plugged In: battery full")
            else:
                notify(f"Charging: {current_percent}%", "normal",
                       "Battery Plugged In")
        else:
            if current_percent <= WARNING_LEVEL and current_percent > CRITICAL_LEVEL:
                notify(f"Current level: {current_percent}%", "normal",
                       "Battery Unplugged - WARNING")
            elif initial_percent <= CRITICAL_LEVEL:
                notify(f"Current level: {current_percent}%",
                       "critical", "Battery Unplugged - CRITICAL")
            else:
                notify(f"Current level: {current_percent}%", "normal",
                       "Battery Unplugged")
        initial_plugged = plugged

    if current_percent != initial_percent:
        if not plugged:
            if current_percent == WARNING_LEVEL:
                notify(f"Current level: {current_percent}%", "normal",
                       "Battery - WARNING")
            elif current_percent == CRITICAL_LEVEL:
                notify(f"Current level: {current_percent}%",
                       "critical", "Battery - CRITICAL")
        initial_percent = current_percent

    time.sleep(CHECK_INTERVAL)
