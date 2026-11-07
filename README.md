TML stands for Task Manager Logger. What that implies is an app that is basically like task manager but with a
logger/timeline mechanic.

So you start the app and it scans for the processes currently running and then goes on to keep track of the changes
in processes since that point in order for you to read what processes are active at any point since that start.

The purpose of this application is to give the user more intuitive insight into what's happening on their device.
I intend to design it so that you don't need extensive knowledge about PIDs, kernel providers and whatnot in
order to know what processes are being ran behind the scenes, what they're doing and what causes them.

The app makes use of NtQuerySystemInformation and ETW (Kernel Process Provider) + TDH to construct a timeline of
running processes since application start. These events then go trough a small series of processing steps before
eventually being presented as a timeline in an interactable window.

---

I also intend to eventually deploy a mechanic that let's you schedule a scan to start for next boot so that you may
also view startup processes.

At the state of the current release, the timeline is displayed as a barchart in a window and clicking anywhere on
the bar-chart lists the PIDs of all active processes at that point in the command panel that opened with it.

**In the GitHub Release/in the Terra HackClub Demo will you find instructions on how to launch the app and run it,
it's inconvenient to list in here because I can't show any pictures**
