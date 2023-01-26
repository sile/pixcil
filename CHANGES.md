main
====

core
----

- [UPDATE] Support small screen (auto resize)
- [FIX] Ensure that preview size reflects loaded image size
- [UPDATE] Support loading PNG files using palette mode
- [FIX] Don't reset "FRAME PREVIEW" setting when opening settings dialog
- [CHANGE] Update pagurus to v0.6.2

web
---

- [UPDATE] Factor out common logic as `pixcil` NPM library
- [UPDATE] Add PWA support

vscode
------

- Initial release

0.3.0
=====

core
----

- [UPDATE] Support to import gray scale PNG files
- [FIX] Fix a bug that the program crashes when an HSV color slider reaches the max value and then the up button is pressed
- [CHANGE] Limit maximum FPS to 120 to eliminate too many redraws
- [UPDATE] Update pagurus version to v0.5.0

web
---

- [Add] Use service worker to support offline mode
- [UPDATE] Remove green margin around the canvas

windows
-------

- [ADD] Initial release

0.2.0
=====

- [UPDATE] Skip storing a pixel instance if it's alpha is zero
- [FIX] Ensure imported image positions align with pixel size
- [UPDATE] Brighten unfocused text box background color
- [CHANGE] Remove erasing tool dialog as it's redundant with the selection-then-cut feature
- [UPDATE] Merge multiple redraw requests issued during an event handling
- [UPDATE] Change the pick-button into non-clickable state when it's selected
- [ADD] Add buttons to halve / double the pixel size setting
- [FIX] Consider pixel size when copying
- [ADD] Support flip and rotate operations

0.1.0
=====

First release
