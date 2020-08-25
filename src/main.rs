use std::path::PathBuf;
use structopt::StructOpt;

mod file;
mod lcsq;

#[derive(StructOpt, Debug)]
#[structopt(name = "diff")]
struct Opt {
    /// Old File to process
    #[structopt(name = "OLD_FILE", parse(from_os_str))]
    old_file: PathBuf,
    /// New Files to process
    #[structopt(name = "NEW_FILE", parse(from_os_str))]
    new_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    print_diff(opt.old_file.to_str().unwrap(),
               opt.new_file.to_str().unwrap());
}

fn print_diff(old_file: &str, new_file: &str) {
    let old_hash_vec = file::file_to_hash_vector(old_file);
    let new_hash_vec = file::file_to_hash_vector(new_file);
    let rev_diff = lcsq::get_rev_diff(&old_hash_vec, &new_hash_vec);

    let mut old_line_iter = file::file_to_line_iter(old_file);
    let mut new_line_iter = file::file_to_line_iter(new_file);

    lcsq::print_diff_iter(&rev_diff, &mut old_line_iter, &mut new_line_iter);
}
