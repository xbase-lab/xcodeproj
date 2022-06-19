# XcodeProj

XcodeProj reader and writer.

Currently optimized for reading. please see docs for usage.

## Milestones

- [x] parse `*.xcodeproj` through [pest]
- [x] parse [pest] ast to `PBXRootObject`, as an meaningful abstraction.
- [ ] add helper methods to manipulate and read pbxproj objects.
- [ ] write to `*.xcodeproj` filetype.
- [ ] preserve comments and reduce git conflicts.
- [ ] support reading XCWorkspace and XCScheme


[pest]: https://github.com/pest-parser/pest
