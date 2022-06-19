fn main() {
    let files = skeptic::markdown_files_of_directory("../src/")
        .into_iter()
        .collect::<Vec<_>>();
    skeptic::generate_doc_tests(&files);
}
