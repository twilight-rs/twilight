fn main() {
    let files = skeptic::markdown_files_of_directory("../src/")
        .into_iter()
        .filter(|path| !path.to_str().unwrap().contains("versions/0.15/"))
        .collect::<Vec<_>>();
    skeptic::generate_doc_tests(&files);
}
