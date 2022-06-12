use super::*;

impl PBXFSReference {
    /// Returns a file path to current fs reference using source root.
    pub fn full_path<P: AsRef<Path>>(&self, source_root: P) -> Result<PathBuf> {
        let source_root = source_root.as_ref();

        let path = || {
            self.path()
                .ok_or_else(|| anyhow::anyhow!("Expected path to be set in file element!!"))
        };

        fn get_parts(path: &String) -> Vec<&str> {
            if path.contains("/") {
                path.split("/").collect()
            } else {
                vec![path]
            }
        }

        match self.source_tree() {
            Some(PBXSourceTree::Absolute) => path()?.pipe(PathBuf::from),
            Some(PBXSourceTree::SourceRoot) => {
                let mut root = source_root.to_path_buf();
                root.extend(get_parts(path()?));
                root
            }
            Some(PBXSourceTree::Group) => {
                let mut group_path: PathBuf;

                if let Some(parent) = self.parent() {
                    group_path = parent.borrow().full_path(&source_root)?;
                    if let Some(path) = self.path() {
                        group_path.extend(get_parts(path))
                    }
                } else {
                    let objects = self
                        .objects
                        .upgrade()
                        .ok_or_else(|| anyhow::anyhow!("objects is released already!"))?;

                    let objects = objects.borrow();

                    if objects
                        .projects()
                        .into_iter()
                        .find(|(_, p)| &*p.borrow().main_group().borrow() == self)
                        .is_some()
                    {
                        if let Some(path) = self.path() {
                            let mut root = source_root.to_path_buf();
                            root.extend(get_parts(path));
                            return Ok(root);
                        } else {
                            return Ok(source_root.to_path_buf());
                        }
                    }

                    // Fallback if parent is nil and it's not root element
                    let group = objects
                        .groups()
                        .into_iter()
                        .find(|(_, o)| {
                            o.borrow()
                                .children()
                                .into_iter()
                                .any(|o| &*o.borrow() == self)
                        })
                        .map(|(_, o)| o)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "Invalid group path {source_root:?} with {:?}",
                                self.path()
                            )
                        })?;

                    group_path = group.borrow().full_path(source_root)?;
                }
                group_path
            }
            _ => {
                bail!("Can't get full_path from {:#?}", self)
            }
        }
        .pipe(Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_root_full_path() {
        use crate::pbxproj::test_demo_file;
        let project = test_demo_file!(demo1);
        let main_group = project
            .objects()
            .projects()
            .first()
            .unwrap()
            .1
            .borrow()
            .main_group();

        let root = PathBuf::from("/path/to/project");
        let main_group = main_group.borrow();
        let main_group_full_path = main_group.full_path(&root);
        assert_eq!(main_group_full_path.unwrap(), root);
    }

    #[test]
    fn get_subgroup_full_path() {
        let root = PathBuf::from("/path/to/project");
        let project = crate::pbxproj::test_demo_file!(demo1);

        let source_group = project
            .objects()
            .groups()
            .into_iter()
            .find(|(_, o)| o.borrow().path().map(|p| p == "Source").unwrap_or_default())
            .map(|(_, o)| o.clone())
            .unwrap();

        let source_group = source_group.borrow();
        let source_group_full_path = source_group.full_path(&root);
        assert_eq!(source_group_full_path.unwrap(), root.join("Source"));
    }

    #[test]
    fn get_file_full_path() {
        let root = PathBuf::from("/path/to/project");
        let project = crate::pbxproj::test_demo_file!(demo1);

        let mut expected_file_path = root.clone();
        expected_file_path.extend(&["Source", "Views", "GuessView.swift"]);

        let file = project
            .objects()
            .get_fs_references(|fs_reference| {
                fs_reference
                    .path()
                    .map(|name| name == "GuessView.swift")
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
            .first()
            .map(|(_, o)| o.clone())
            .unwrap();

        let file = file.borrow();

        assert_eq!(file.full_path(root).unwrap(), expected_file_path)
    }
}
