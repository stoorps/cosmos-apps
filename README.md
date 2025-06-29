# cosmos-apps
A monorepo containing my current cosmic app projects

## cosmos-disks
At the moment, I'm using usdisks2-rs and zbus as a basis for this application, but have my eye on disks-rs, and may decide to start using it/contributing to it in the near future.
The code (which is currently in a rough prototyping phase) is available here.  The UI is essentially a clone of gnome disks, but I have some plans to improve this in the future. 
The goal of this project is to ship as a part of the [Cosmic Utilities](https://github.com/cosmic-utils) organisation once it's ready.

![Screenshot of cosmos-disks](https://github.com/stoorps/cosmos-apps/blob/main/screenshots/cosmos-disks.png)


## cosmos-dbus
This project will be an abstraction layer for comsos-disks, and any other dbus interfaces. The idea here is to provide models that can easily be swapped out at a later date, as better suited rust crates become available for achieving the same functionality.

## cosmos-common
A place for UI stuff shared across projects.

## cosmos-apx
This project is a WIP of a UI for APX. It is currently in a very rough state, and is being somewhat neglected while I work on bringing cosmic-disks up to production.

## apx-shim
Similar idea to cosmos-dbus, but for interacting with the APX CLI. I'm not a fan of CLI wrappers, and so I'll be looking into replacing this with a rust implementation/binding. 
