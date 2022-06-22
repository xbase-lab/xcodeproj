# Changelog
## [0.2.11] - 2022-06-22
### <!-- 1 -->Bug Fixes
- `(Target)` <a href="https://github.com/tami5/xcodeproj/commit/a26044f"> Get platform result in index outbound</a>

## [0.2.10] - 2022-06-21
### <!-- 0 -->Features
- `(Xcodeproj)` <a href="https://github.com/tami5/xcodeproj/commit/e8f38f6"> Get all build file names</a>

## [0.2.9] - 2022-06-21
### <!-- 0 -->Features
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/6195eff"> Skip printing objects</a>
### <!-- 2 -->Refactor
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/c93e7c0"> Only create via xcodeproj_folder</a>

## [0.2.8] - 2022-06-19
### <!-- 0 -->Features
- `(Target)` <a href="https://github.com/tami5/xcodeproj/commit/5052874"> Got the extra mile to find sdkroot</a>
- `(Xcodeproj)` <a href="https://github.com/tami5/xcodeproj/commit/7ca1076"> Expose root and set name</a>
- `(Xcodeproj)` <a href="https://github.com/tami5/xcodeproj/commit/b5ddde2"> Create from project root</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/04a8732"> Helper function to generate hashmap of target names and platform</a>
### <!-- 2 -->Refactor
- `(Target)` <a href="https://github.com/tami5/xcodeproj/commit/d06cba8"> Replace sdkroots with platform</a>

## [0.2.7] - 2022-06-19
### <!-- 0 -->Features
- `(Target)` <a href="https://github.com/tami5/xcodeproj/commit/5730d7b"> Get sdkroots</a>
### <!-- 2 -->Refactor
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/e919309"> Remove unused dependency</a>

## [0.2.6] - 2022-06-19
### <!-- 0 -->Features
- `(Xcodeproj)` <a href="https://github.com/tami5/xcodeproj/commit/06ed5c4"> Implement default</a>

## [0.2.5] - 2022-06-19
### <!-- 0 -->Features
- `(Xcodeproj)` <a href="https://github.com/tami5/xcodeproj/commit/7cfe8c1"> Deref as pbxproj</a>

## [0.2.4] - 2022-06-19
### <!-- 2 -->Refactor
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/8c045ae"> Create PBXObject at request</a>

## [0.2.3] - 2022-06-17
### <!-- 0 -->Features
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/53813b7"> PBXProject helper methods</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/2c465af"> Add swift package</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/bd149af"> Generate md5 hash</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/fdaae8a"> More collection object getters</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/7f44b7a"> Auto set fs_reference parent</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/d1f3678"> Get full path for a group or file</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/c3cc42a"> Get file by name or path</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/ce7c4f6"> Add file</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/dedff8c"> Add build file in add_file</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/0fec427"> Add extra helper methods to Root Object</a>
### <!-- 1 -->Bug Fixes
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/304cada"> Missing object's member build phase</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/c97ca8d"> Broken doc links</a>
### <!-- 2 -->Refactor
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/23e5103"> Move some query logic to collection</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/5e8b060"> Root object no more deref to objects</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/58b5e23"> Abstract PBXFileReference & PBX*Group into one type</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/089d681"> Get full_path returns result</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/b85fbd0"> Move full_path fn to another file</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/0e5706a"> Move pbxproj tests out of meta</a>

## [0.2.2] - 2022-06-09
### <!-- 2 -->Refactor
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/7eaa65e"> Abstract all target variants into one</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/f1be75c"> Abstract all build phase variants into one</a>

## [0.2.1] - 2022-06-09
### <!-- 0 -->Features
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/2334b75"> Use weak reference of objects collection</a>
### <!-- 1 -->Bug Fixes
- `(Doc)` <a href="https://github.com/tami5/xcodeproj/commit/c952698"> Rust doc broken link</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/9221bc0"> Remote url key</a>
### <!-- 2 -->Refactor
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/9c2a66c"> Deref root object to object collection</a>
### Ci
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/23f1a17"> Try v*.* pattern instead</a>

## [0.2] - 2022-06-08
### <!-- 0 -->Features
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/b99ae6d"> Deserialize object representation</a>
### <!-- 2 -->Refactor
- `(Pest)` <a href="https://github.com/tami5/xcodeproj/commit/ad5ca3f"> Keep keys casing as is.</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/c278cbe"> Rename PBXArray to PBXVec</a>
### Ci
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/768c7fd"> Only release on minor releases</a>

## [0.1.4] - 2022-06-07
### <!-- 0 -->Features
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/83e09d0"> Auto convert keys to snake_case unless uuid or string</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/3df8d3f"> Extract key string value</a>
- `(Pbxproj)` <a href="https://github.com/tami5/xcodeproj/commit/41a3ba8"> Extract PBXValue by key</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/d6b08ed"> Add data representation for product types</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/050bbe3"> Abstract hashmap and vec for helper methods</a>
### <!-- 2 -->Refactor
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/fbd07f9"> Move pest parser to pest module</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/a4c4b26"> Move pbxproj to top level</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/697c230"> Isolate and rename root object PBXRootObject</a>

## [0.1.3] - 2022-06-05
### <!-- 0 -->Features
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/c637e48"> Parse xproj ast to HashMap</a>
- `(General)` <a href="https://github.com/tami5/xcodeproj/commit/d15ef62"> Simple represent pbxproj parser result</a>
### <!-- 1 -->Bug Fixes
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/0854f93"> Pest number rule</a>
### <!-- 2 -->Refactor
- `(Grammar)` <a href="https://github.com/tami5/xcodeproj/commit/41489f7"> Identify kind and uuid</a>
- `(Grammar)` <a href="https://github.com/tami5/xcodeproj/commit/191698e"> Make ident as last option to match</a>
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/02e5292"> Reorganize module</a>
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/4d389bb"> Rename to XProj to PBXProject</a>
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/59a0ca0"> Object kind</a>

## [0.1.1] - 2022-06-01
### <!-- 0 -->Features
- `(Parser)` <a href="https://github.com/tami5/xcodeproj/commit/047b76d"> Initial working implementation</a>

