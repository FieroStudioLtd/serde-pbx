A Rust crate for working with Apple's .pbxproj format.

[<img src="https://img.shields.io/crates/v/serde-pbx">](https://crates.io/crates/serde-pbx)
[<img src="https://img.shields.io/badge/docs.rs-serde--pbx-green">](https://docs.rs/serde-pbx)

# Installation
Add a dependency on `serde-pbx`.

```toml
[dependencies]
serde-pbx = "0.1"
```

# Usage
Build up your .pbxproj in code! Each call to `add_object` returns the ID of the newly-created object, for referencing in downstream steps.

```rust
use serde_pbx::{self, PBXProject, PBXObject, build_settings};

let mut project = PBXProject::default();

// let's add a built file to this project
let main_swift = project.add_object(PBXObject::PBXFileReference {
	source_tree: "<group>".to_owned(),
	path: "game/main.swift".to_owned(),
	explicit_file_type: "sourcecode.swift".to_owned(),
});

let main_swift_build = project.add_object(PBXObject::PBXBuildFile {
	file_ref: main_swift,
	settings: IndexMap::new(),
});

let source_build = project.add_object(PBXObject::PBXSourcesBuildPhase {
	files: vec![main_swift_build],
});

// NB: we have to do lots more stuff in between here - .pbxproj files typically aren't small!

// now, let's finalise this with a project

let debug_project_build = project.add_object(PBXObject::XCBuildConfiguration {
	name: "Debug".to_owned(),
	build_settings: build_settings([
		("SDKROOT", "iphoneos"),
		("IPHONEOS_DEPLOYMENT_TARGET", "15.5"),
		("ONLY_ACTIVE_ARCH", "YES"),
	]),
});

let project_build_configuration_list = project.add_object(PBXObject::XCConfigurationList {
	build_configurations: vec![debug_project_build],
});

let project_id = project.add_object(PBXObject::PBXProject {
	build_configuration_list: project_build_configuration_list,
	targets: vec![target_id],
});

// and then remember to set the root object too!
project.set_root_object(project_id);

let pbx_string = serde_pbx::to_string(&project).expect("failed to serialise project");
std::fs::write("my_cool_project.xcodeproj/project.pbxproj", pbx_string).expect("failed to write project");
```
