mod ser;
pub use ser::{to_string, Serializer};

use indexmap::IndexMap;
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct PBXObjectID(usize);

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum PBXSetting {
	List(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr)]
#[repr(u32)]
pub enum PBXDstSubfolderSpec {
	Wrapper = 1,
	Executables = 6,
	Resources = 7,
	Frameworks = 10,
	SharedFrameworks = 11,
	SharedSupport = 12,
	Plugins = 13,
}

#[derive(Debug, Serialize)]
#[serde(tag = "isa")]
pub enum PBXObject {
	#[serde(rename_all = "camelCase")]
	PBXFileReference {
		path: String,
		explicit_file_type: String,
		source_tree: String,
	},
	#[serde(rename_all = "camelCase")]
	PBXBuildFile {
		file_ref: PBXObjectID,
		settings: IndexMap<String, PBXSetting>,
	},
	#[serde(rename_all = "camelCase")]
	PBXProject {
		build_configuration_list: PBXObjectID,
		targets: Vec<PBXObjectID>,
	},
	#[serde(rename_all = "camelCase")]
	PBXNativeTarget {
		name: String,
		product_name: String,
		product_reference: PBXObjectID,
		product_type: String,
		build_configuration_list: PBXObjectID,
		build_phases: Vec<PBXObjectID>,
		build_rules: Vec<PBXObjectID>,
		dependencies: Vec<PBXObjectID>,
	},
	#[serde(rename_all = "camelCase")]
	XCBuildConfiguration {
		name: String,
		build_settings: IndexMap<String, String>,
	},
	#[serde(rename_all = "camelCase")]
	XCConfigurationList { build_configurations: Vec<PBXObjectID> },
	#[serde(rename_all = "camelCase")]
	PBXSourcesBuildPhase { files: Vec<PBXObjectID> },
	#[serde(rename_all = "camelCase")]
	PBXFrameworksBuildPhase { files: Vec<PBXObjectID> },
	#[serde(rename_all = "camelCase")]
	PBXShellScriptBuildPhase { shell_path: String, shell_script: String },
	#[serde(rename_all = "camelCase")]
	PBXCopyFilesBuildPhase {
		files: Vec<PBXObjectID>,
		dst_path: String,
		dst_subfolder_spec: PBXDstSubfolderSpec,
	},
	#[serde(rename_all = "camelCase")]
	PBXResourcesBuildPhase { files: Vec<PBXObjectID> },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PBXProject {
	archive_version: i32,
	object_version: i32,
	objects: IndexMap<PBXObjectID, PBXObject>,
	root_object: Option<PBXObjectID>,
}

impl Default for PBXProject {
	fn default() -> Self {
		Self::new(1, 55)
	}
}

impl PBXProject {
	pub fn new(archive_version: i32, object_version: i32) -> Self {
		Self {
			archive_version,
			object_version,
			objects: IndexMap::new(),
			root_object: None,
		}
	}

	pub fn add_object(&mut self, object: PBXObject) -> PBXObjectID {
		let id = PBXObjectID(self.objects.len());
		self.objects.insert(id, object);
		id
	}

	pub fn set_root_object(&mut self, object_id: PBXObjectID) {
		self.root_object = Some(object_id);
	}
}

pub fn build_settings<const N: usize>(arr: [(&str, &str); N]) -> IndexMap<String, String> {
	arr.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
}
