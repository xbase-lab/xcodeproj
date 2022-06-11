# XcodeProj

Work-in-progress XcodeProj reader and writer.

Currently optimized for reading and not garanted to be used to modify existing xcodeproj.

## Milestones

- [x] parse `*.xcodeproj` through [pest]
- [x] parse [pest] ast to `PBXRootObject`, as an meaningful abstraction.
- [ ] add helper methods to maniuplate and read pbxproj objects.
- [ ] write `ProjectData` back to `*.xcodeproj` filetype.
- [ ] preserve comments and reduce git conflicts.
- [ ] support reading XCWorkspace and XCScheme


[pest]: https://github.com/pest-parser/pest
