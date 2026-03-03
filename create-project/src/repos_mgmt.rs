/* APIs to manage cheriot-rtos and network-stack repositories */

use std::process::Command;
use std::path::Path;

fn repos_mgmt_sync (repo_path: &Path) {
    assert!(!repo_path.exists());

    let _output = Command::new("git").arg("submodule").arg("sync").arg("--recursive").current_dir(repo_path);
}

fn repos_mgmt_update (repo_path: &Path) {
    assert!(!repo_path.exists());

    let _output = Command::new("git").arg("submodule").arg("--update").arg("--init").arg("--recursive").current_dir(repo_path);
}