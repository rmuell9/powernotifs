import pyudev

class BatteryMonitor:
    def __init__(self):
        self.context = pyudev.Context()
        self.monitor = pyudev.Monitor.from_netlink(self.context)
        self.monitor.filter_by(subsystem='power_supply')
    
    def get_battery_info(self):
        for device in self.context.list_devices(subsystem='power_supply'):
            if device.get('POWER_SUPPLY_TYPE') == 'Battery':
                capacity = device.get('POWER_SUPPLY_CAPACITY')
                status = device.get('POWER_SUPPLY_STATUS')
                
                power_percentage = int(capacity) if capacity else None
                is_plugged_in = status in ('Charging', 'Full', 'Not charging')
                
                return {
                    'power_percentage': power_percentage,
                    'is_plugged_in': is_plugged_in,
                    'status': status
                }
        return None
    
    def start_monitoring(self, callback):
        observer = pyudev.MonitorObserver(self.monitor, callback=callback)
        observer.start()
        return observer


def on_battery_event(device):
    if device.get('POWER_SUPPLY_TYPE') == 'Battery':
        capacity = device.get('POWER_SUPPLY_CAPACITY')
        status = device.get('POWER_SUPPLY_STATUS')
        print(f"Battery update - Percentage: {capacity}%, Status: {status}")


if __name__ == '__main__':
    monitor = BatteryMonitor()
    
    info = monitor.get_battery_info()
    if info:
        print(f"Current battery: {info['power_percentage']}%")
        print(f"Plugged in: {info['is_plugged_in']}")
        print(f"Status: {info['status']}")
    else:
        print("No battery found")
    
    print("\nMonitoring for changes (Ctrl+C to stop)...")
    observer = monitor.start_monitoring(on_battery_event)
    
    try:
        import time
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
        print("\nStopped monitoring")
