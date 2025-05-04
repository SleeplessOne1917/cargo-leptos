use package_json_schema::PackageJson;

use crate::ext::fs::read_to_string;

use super::Config;

async fn foo(config: &Config) {
    let package_json_path = config.working_dir.join("package.json");

    if package_json_path.exists() {
        let mut package_json_config = read_to_string(package_json_path)
            .await
            .map(PackageJson::try_from)
            .unwrap()
            .unwrap();

        let mut tailwind_version = None;
        let mut sass_version = None;

        if let Some(dev_dependencies) = package_json_config.dev_dependencies.as_ref() {
            for (name, version) in dev_dependencies.iter() {
                match name.as_str() {
                    "tailwindcss" => {
                        tailwind_version = Some(version);

                        if sass_version.is_some() {
                            break;
                        }
                    }
                    "sass" => {
                        sass_version = Some(version);

                        if tailwind_version.is_some() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !(tailwind_version.is_some() && sass_version.is_some()) {
            if let Some(dependencies) = package_json_config.dependencies.as_ref() {
                for (name, version) in dependencies.iter() {
                    match name.as_str() {
                        "tailwindcss" => {
                            tailwind_version = Some(version);

                            if sass_version.is_some() {
                                break;
                            }
                        }
                        "sass" => {
                            sass_version = Some(version);

                            if tailwind_version.is_some() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if sass_version.is_some() {
            let v = package_json_config.scripts.as_mut().unwrap();

            v.insert(
                String::from("cargo-leptos:build-sass"),
                Some(String::from("")),
            );
        }

        if tailwind_version.is_some() {
            let v = package_json_config.scripts.as_mut().unwrap();

            v.insert(
                String::from("cargo-leptos:build-tailwind"),
                Some(String::from("")),
            );
        }
    }
}
