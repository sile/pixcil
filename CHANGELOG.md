# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Fix a bug that a non-move drawing could increase undo counter to the maximum value
- Don't cache fetched workspace files

### Added

- Add non square pixel size support
- Add `orfail` crate to the dependencies

### Removed

- Remove max undos setting
- Remove import-from-clipboard feature
- Remove finger mode
- Remove button long press feature
- Remove `pixcil_windows` crate to reduce maintenance costs

### Changed

- Don't save undo buffer in the image file
- Change default pixel size to 1
- Change PWA display mode from "minimal-ui" to "standalone"
- Update pagurus version from v0.6 to v0.7
- Update libflate version from v1 to v2

## [0.5.0] - 2023-05-31

### Added

- Add created time and updated time attributes
- Add `load` query string parameter to specify a URL of a PNG image to load (Web)
- Add vibration when drawing / erasing / selecting actions are completed
- Add bucket selecting tool
- Add preview scale setting
- Add import-image-from-clipboard feature
- Add finger friendly drawing mode

### Changed

- Set `CanvasRenderingContext2D.imageSmoothingEnabled` to `true`

## [0.4.0] - 2023-02-18

### Added

- Ellipse drawing tool
- Small screen support (auto resize)
- PWA (Progressive Web Apps) support
- Make it possible to load PNG files using palette mode
- VSCode extension

### Fixed

- Don't let preview area consume mouse events for buttons
- Don't reset "FRAME PREVIEW" setting when opening settings dialog
- Ensure that preview size reflects loaded image size

## [0.3.0]

- [UPDATE] Support to import gray scale PNG files
- [FIX] Fix a bug that the program crashes when an HSV color slider reaches the max value and then the up button is pressed
- [CHANGE] Limit maximum FPS to 120 to eliminate too many redraws
- [Add] Use service worker to support offline mode
- [UPDATE] Remove green margin around the canvas (web)
- [ADD] Windows binary

## [0.2.0]

- [UPDATE] Skip storing a pixel instance if it's alpha is zero
- [FIX] Ensure imported image positions align with pixel size
- [UPDATE] Brighten unfocused text box background color
- [CHANGE] Remove erasing tool dialog as it's redundant with the selection-then-cut feature
- [UPDATE] Merge multiple redraw requests issued during an event handling
- [UPDATE] Change the pick-button into non-clickable state when it's selected
- [ADD] Add buttons to halve / double the pixel size setting
- [FIX] Consider pixel size when copying
- [ADD] Support flip and rotate operations

## [0.1.0]

- Initial release
