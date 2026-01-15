import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / '.github' / 'lint' / 'message_event_lint.py'

BAD = """
fn wrong(mut writer: MessageWriter<Foo>) {
    commands.spawn(());
}
"""

GOOD = """
fn ok(mut writer: MessageWriter<Foo>) {
    writer.send(Foo{});
}
"""


def run_script_with_src(src: str):
    p = Path('tmp_test.rs')
    p.write_text(src)
    try:
        res = subprocess.run([sys.executable, str(SCRIPT), str(p)], capture_output=True)
        return res.returncode, res.stdout.decode(), res.stderr.decode()
    finally:
        p.unlink()


def test_bad_is_flagged():
    rc, out, err = run_script_with_src(BAD)
    assert rc != 0
    assert 'likely misuse' in out or 'potential misuse' in out


def test_good_is_ok():
    rc, out, err = run_script_with_src(GOOD)
    assert rc == 0
