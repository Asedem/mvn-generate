use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use resource::resource_str;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("Please use mvn-generate <project-name>");
        return;
    }

    let project_name = &args[0];

    if !ask_if_overwrite(&project_name) {
        return;
    }

    let version = prompt("Enter the version of Java that should be used:");
    let group_id = prompt("Provide a groupId for the Project:");
    let artifact_id = prompt("Provide an artifactId for the Project:");

    let group_id_as_path = group_id.replace(".", "/");

    create_directories(&project_name, &group_id_as_path);

    let main_class_name = uppercase_first_letter(group_id.split('.').last().unwrap());

    create_file(
        &format!("{}/src/main/java/{}/{}.java", project_name, group_id_as_path, main_class_name),
        resource_str!("rsc/Main.java")
            .as_ref()
            .to_string()
            .replace("{package}", &group_id)
            .replace("{class_name}", &main_class_name),
    );

    create_file(
        &format!("{}/src/test/java/{}/{}Test.java", project_name, group_id_as_path, main_class_name),
        resource_str!("rsc/MainTest.java")
            .as_ref()
            .to_string()
            .replace("{package}", &group_id)
            .replace("{class_name}", &main_class_name),
    );

    create_file(
        &format!("{}/pom.xml", project_name),
        resource_str!("rsc/pom.xml")
            .as_ref()
            .to_string()
            .replace("{group_id}", &group_id)
            .replace("{artifact_id}", &artifact_id)
            .replace("{name}", &project_name)
            .replace("{version}", &version),
    );
}

fn ask_if_overwrite(project_name: &str) -> bool {
    if !Path::new(project_name).exists() {
        return true;
    }

    println!("There is already a Directory named {}. Should the project be created within it?", project_name);
    println!("yes (y) | no (n)");

    loop {
        let mut decision = String::new();
        io::stdin().read_line(&mut decision).unwrap();
        let decision = decision.trim();

        if decision == "y" {
            return true;
        } else if decision == "n" {
            return false;
        }
    }
}

fn prompt(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_directories(project_name: &str, group_id_as_path: &str) {
    println!("Creating directories:");
    println!("{}/src/main/java/{}", project_name, group_id_as_path);
    println!("{}/src/test/java/{}", project_name, group_id_as_path);

    fs::create_dir_all(format!("{}/src/main/java/{}", project_name, group_id_as_path)).unwrap();
    fs::create_dir_all(format!("{}/src/test/java/{}", project_name, group_id_as_path)).unwrap();
}

fn create_file(path: &str, content: String) {
    println!("Creating file: {}", path);
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
