extern crate autojump;


// Inspired by and taken from `rustfmt`.
// Include git commit hash and worktree status; contents are like
//   const COMMIT_HASH: Option<&'static str> = Some("5d53581");
//   const WORKTREE_CLEAN: Option<bool> = Some(false);
// with `None` if running git failed, eg if it is not installed.
include!(concat!(env!("OUT_DIR"), "/git_info.rs"));


fn main() {
    autojump::main(autojump::get_version_str(COMMIT_HASH, WORKTREE_CLEAN));
}
