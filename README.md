TML stands for Task Manager Logger which is kind of stupid

What those words try to imply is an app that is basically like task manager but with a logger/timeline mechanic.

So you start the app and it scans for the processes currently running and then goes on to keep track of the changes
in processes since that point in order for you to read what processes are active at any point since that start.

It makes use of NtQuerySystemInformation for the initial event snapshot and ETW + TDH currently for the continuous
flow of events afterwards.

These events then go trough a small series of processing steps before eventually being presented as a timeline
in an interactable window.

This app requires admin to run as well as the following installments from the Desktop Development with C++ workload
from Visual Studio Community 2022:

MSVC v143 - VS 2022 C++ x64/x86 build tools

Windows 11 SDK (some recent version)

At the current moment, the timeline is presented as a bar-chart using the egui_plot library and if you press anywhere
on it then all of the PIDs for each process active at that point will be displayed, that's bound to change soon though
if it hasn't already.

If it's still refusing to work, restarting your pc tends to do the trick
