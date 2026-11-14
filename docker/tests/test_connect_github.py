"""Exercise repository setup against a real local Git repository, offline."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location(
    'connect_github', Path(__file__).resolve().parents[1] / 'connect-github.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], stderr=subprocess.DEVNULL).decode().strip()


class ConnectTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        base = Path(self.temp.name)
        self.remote, self.workspace = base / 'remote', base / 'workspace'
        self.remote.mkdir()
        self.workspace.mkdir()
        git(self.remote, 'init', '-b', 'main')
        for name, text in [('src/example.txt', 'upstream'), ('README.md', 'readme'),
                           ('build/generated.txt', 'host output'), ('log/old.log', 'host log')]:
            path = self.remote / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
        git(self.remote, 'add', '.')
        git(self.remote, '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
            'commit', '-qm', 'fixture')

    def connect(self):
        module.connect(self.workspace, str(self.remote), 'main')

    def test_preserves_edits_additions_deletions_and_generated_outputs(self):
        source = self.workspace / 'src/example.txt'
        source.parent.mkdir()
        source.write_text('local work')
        output = self.workspace / 'build/generated.txt'
        output.parent.mkdir()
        output.write_text('container output')
        addition = self.workspace / 'src/new.txt'
        addition.write_text('new local file')
        self.connect()
        self.assertEqual(source.read_text(), 'local work')
        self.assertEqual(output.read_text(), 'container output')
        self.assertEqual(addition.read_text(), 'new local file')
        self.assertFalse((self.workspace / 'README.md').exists())
        self.assertFalse((self.workspace / 'log/old.log').exists())
        self.assertEqual(git(self.workspace, 'rev-parse', 'HEAD'), git(self.remote, 'rev-parse', 'HEAD'))
        self.assertEqual(git(self.workspace, 'rev-parse', '--abbrev-ref', '@{upstream}'), 'origin/main')

    def test_rerun_preserves_developer_branch(self):
        self.connect()
        git(self.workspace, 'switch', '-c', 'teammate-work')
        (self.workspace / 'src').mkdir()
        (self.workspace / 'src/example.txt').write_text('new edit')
        self.connect()
        self.assertEqual(git(self.workspace, 'branch', '--show-current'), 'teammate-work')
        self.assertEqual((self.workspace / 'src/example.txt').read_text(), 'new edit')

    def test_different_remote_is_rejected_without_replacing_it(self):
        self.connect()
        git(self.workspace, 'remote', 'set-url', 'origin', 'https://example.invalid/other.git')
        with self.assertRaisesRegex(RuntimeError, 'Existing origin differs'):
            self.connect()
        self.assertEqual(git(self.workspace, 'remote', 'get-url', 'origin'),
                         'https://example.invalid/other.git')


if __name__ == '__main__':
    unittest.main()
