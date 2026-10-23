"""Packaged validation must retain the checkout's selected compiler."""

import importlib.util
import io
import os
import pathlib
import subprocess
import tarfile
import unittest
import tempfile
from unittest.mock import patch


class PackageToolchainTests(unittest.TestCase):
    def test_hook_installation_uses_package_root_outside_working_directory(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        with tempfile.TemporaryDirectory() as directory:
            temporary = pathlib.Path(directory)
            stub = temporary / "rhusky.rs"
            stub.write_text('''
pub struct Rhusky;
impl Rhusky {
    pub fn new() -> Self { Self }
    pub fn hooks_dir(self, _: &str) -> Self { self }
    pub fn skip_in_env(self, _: &str) -> Self { self }
    pub fn with_default_hooks(self) -> Self { self }
    pub fn install_from_build_script(&self, root: impl AsRef<std::path::Path>)
        -> std::io::Result<()> {
        assert_eq!(root.as_ref(), std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
        assert_ne!(root.as_ref(), std::env::current_dir()?.as_path());
        Ok(())
    }
}
''')
            environment = dict(os.environ, CARGO_MANIFEST_DIR=str(root))
            library = temporary / "librhusky.rlib"
            subprocess.run(["rustc", "--crate-name", "rhusky", "--crate-type", "rlib",
                            str(stub), "-o", str(library)], env=environment, check=True)
            binary = temporary / ("build-hook.exe" if os.name == "nt" else "build-hook")
            subprocess.run(["rustc", "--edition=2024", str(root / "build.rs"),
                            "--extern", f"rhusky={library}", "-o", str(binary)],
                           env=environment, check=True)
            subprocess.run([str(binary)], cwd=temporary, check=True)

    def test_extracted_archive_retains_active_toolchain(self):
        source = pathlib.Path(__file__).with_name("test-package.py")
        spec = importlib.util.spec_from_file_location("package_validation", source)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        calls = []

        def output(command, **kwargs):
            if command == ["git", "ls-files", "tests/"]:
                return "tests/fixture.rs\n"
            if command == ["rustup", "show", "active-toolchain"]:
                return "1.99.1-test-host (directory override)\n"
            raise AssertionError(command)

        def run(command, **kwargs):
            calls.append((command, kwargs))
            if command[:2] == ["cargo", "package"]:
                target = pathlib.Path(command[command.index("--target-dir") + 1])
                package = module.tomllib.loads((module.pathlib.Path(module.__file__).parents[1] / "Cargo.toml").read_text())["package"]
                stem = f"{package['name']}-{package['version']}"
                archive_path = target / "package" / f"{stem}.crate"
                archive_path.parent.mkdir()
                with tarfile.open(archive_path, "w:gz") as archive:
                    content = b"// fixture"
                    entry = tarfile.TarInfo(f"{stem}/tests/fixture.rs")
                    entry.size = len(content)
                    archive.addfile(entry, io.BytesIO(content))
            return subprocess.CompletedProcess(command, 0)

        with patch.object(module.subprocess, "check_output", side_effect=output), patch.object(module.subprocess, "run", side_effect=run):
            module.main()
        tests = [kwargs for command, kwargs in calls if command[:2] == ["cargo", "test"]]
        self.assertEqual(len(tests), 2)
        for kwargs in tests:
            self.assertEqual(kwargs.get("env", {}).get("RUSTUP_TOOLCHAIN"), "1.99.1-test-host")


if __name__ == "__main__":
    unittest.main()
