use super::types::RustupPackage;

pub fn toolchain_of_component_was_already_removed(
    removed_toolchains: &[String],
    component: &RustupPackage,
) -> bool {
    removed_toolchains.contains(&component.toolchain)
}

pub fn install_components(line: &str, toolchain: &str, val: &mut Vec<String>) {
    let mut chunks = line.splitn(3, '-');
    let component = chunks.next().expect("Component name is empty!");
    match component {
        // these are the only components that have a single word name
        "cargo" | "rustfmt" | "clippy" | "miri" | "rls" | "rustc" => {
            val.push([toolchain, component].join("/"));
        }
        // all the others have two words hyphenated as component names
        _ => {
            let component = [
                component,
                chunks
                    .next()
                    .expect("No such component is managed by rustup"),
            ]
            .join("-");
            val.push([toolchain, component.as_str()].join("/"));
        }
    }
}

pub fn group_components_by_toolchains(components: Vec<RustupPackage>) -> Vec<Vec<RustupPackage>> {
    let mut result = vec![];

    let mut toolchains: Vec<String> = vec![];

    for component in components {
        let index = toolchains
            .iter()
            .enumerate()
            .find(|(_, toolchain)| toolchain == &&component.toolchain)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| {
                toolchains.push(component.toolchain.clone());
                result.push(vec![]);
                toolchains.len() - 1
            });

        result
            .get_mut(index)
            .expect(
                "either the index already existed or we just pushed the element with that index",
            )
            .push(component);
    }

    result
}
