# v1.1.0

* Implemented `ControlPanelCategoryShellitem` struct
* Implemented `ControlPanelItemShellitem` struct
* Implemented `URIShellItem` struct
* Implemented  `UsersFilesFolderShellItem` struct
* Fixed a bug on `FileEntryShellItem` struct where it will reach end of file and not properly handle the parsing.
* Change `Path` trait implementation for `IDList` struct
* Changed the `UnimplementedShellItems`  to dump the hex data instead of just saying `Unimplemented`
* Implemented `Name` trait for `ShellItem` struct
* Change the implementation for `Name` trait on `RootShellItem` struct to print the `GUID` if the root is unknown
* Added `Rot13` decoder to the `utils` module

# v1.0.0

 Added `IDList` implementation and general fixes

* Changed structure for the modules and sub-modules.
* Implemented `IDList` struct
* Implemented `NetworkLocationShellItem ` struct
* Added traits to standardize the parsers output
* Added more documentation
* Fixed a bug on `DosDateTime` where it will error if the `dateandtime` passed is not correct
* Fixed a bug on `read_utf8_string` where it will return trailing null bytes
* code cleanup
# v0.1.0

Initial release 

