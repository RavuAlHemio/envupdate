# envupdate

Windows tool that informs all top-level windows that the environment variables have changed.

[Microsoft's documentation for WM_SETTINGCHANGE](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settingchange) states:

> To effect a change in the environment variables for the system or the user, broadcast this message with lParam set to the string "Environment".

This should inform the shell (mostly Windows Explorer), which should then start new programs with the updated environment variables.
