# downtime
WIP: Use simple pings to record internet downtime

# plan:
```
Record app start time/date
Ping a website regularly (? every 10 seconds)
  if website down: ping router
    if router up: ping 3 more websites
      if all websites down:
        if was "up": record time/date & details in logfile & "down"
repeat pinging loop
  if any website ping successful: record time/date and "up"

If ctrl-C: record end-time
```