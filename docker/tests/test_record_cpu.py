"""Check CPU accounting math, PID reuse, cgroup scope, and saved CSV output."""
import csv
import importlib.util
import json
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'record_cpu.py'
spec = importlib.util.spec_from_file_location('record_cpu', SCRIPT)
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)


class CpuTests(unittest.TestCase):
    def test_system_percent_excludes_iowait_steal_and_duplicate_guest(self):
        data = module.cpu_percent([0]*8, [10, 0, 20, 50, 10, 0, 0, 10, 100])
        self.assertAlmostEqual(data['busy_pct'], 30.)
        self.assertAlmostEqual(data['iowait_pct'], 10.)
        self.assertAlmostEqual(data['steal_pct'], 10.)
        self.assertIsNone(module.cpu_percent([], [0]*8))

    def test_stat_name_can_contain_spaces_and_parentheses(self):
        fields = ['0']*37
        fields[0], fields[11], fields[12], fields[17], fields[19], fields[36] = 'S', '200', '50', '8', '1234', '6'
        p = module.parse_process_stat('42 (gz (sim)) '+' '.join(fields))
        self.assertEqual((p.pid, p.name, p.user, p.kernel, p.start, p.threads, p.last_cpu),
                         (42, 'gz (sim)', 200, 50, 1234, 8, 6))

    def test_pid_reuse_and_new_process_baselines(self):
        old = module.Process(42, 'old', 1000, 1000, 100, 1, 0)
        new = module.Process(42, 'new', 5, 2, 250, 1, 0)
        self.assertEqual(module.process_delta(old, new, 2., 100), (5, 2))
        self.assertIsNone(module.process_delta(None, new, 3., 100))
        self.assertEqual(module.process_delta(new, module.Process(42, 'new', 15, 7, 250, 1, 0), 3., 100), (10, 5))

    def test_missing_cgroup_is_not_replaced_by_host_root(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            proc = root/'proc'; (proc/'self').mkdir(parents=True)
            cg = root/'cgroup'; cg.mkdir(); (cg/'cpu.stat').write_text('usage_usec 999\n')
            (proc/'self/cgroup').write_text('0::/missing\n')
            self.assertIsNone(module.cgroup_directory(proc, cg))
            (proc/'self/cgroup').write_text('0::/\n')
            self.assertEqual(module.cgroup_directory(proc, cg), cg)

    @unittest.skipUnless(Path('/proc/stat').exists(), 'Linux required')
    def test_cli_writes_csv_and_refuses_overwrite(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)/'run'
            cmd = [sys.executable, str(SCRIPT), '--duration', '.35', '--interval', '.1', '--output', str(output)]
            subprocess.run(cmd, check=True, capture_output=True)
            summary = json.loads((output/'summary.json').read_text())
            self.assertGreaterEqual(summary['samples'], 3)
            self.assertGreater(summary['recorded_seconds'], .3)
            for name in ('system.csv', 'processes.csv', 'cgroup.csv'):
                with (output/name).open() as stream:
                    self.assertTrue(list(csv.DictReader(stream)), name)
            self.assertNotEqual(subprocess.run(cmd, capture_output=True).returncode, 0)

    @unittest.skipUnless(Path('/proc/stat').exists(), 'Linux required')
    def test_ctrl_c_finishes_summary(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)/'run'
            p = subprocess.Popen([sys.executable, str(SCRIPT), '--interval', '.15', '--output', str(output)],
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            try:
                self.assertIn('Recording CPU', p.stdout.readline())
                time.sleep(.2)
                p.send_signal(signal.SIGINT)
                p.communicate(timeout=5)
                self.assertEqual(p.returncode, 0)
                self.assertGreater(json.loads((output/'summary.json').read_text())['samples'], 0)
            finally:
                if p.poll() is None:
                    p.kill(); p.wait()


if __name__ == '__main__':
    unittest.main()
