"""Run the native scenarios owned by this Target or component."""
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[4]/'tests/platforms/esp-idf'))
from host_peripheral import run_cases, run_example_cases


def test_indicator_touch_readiness_candidates_and_io_ownership():
    cases = [[str(n)] for n in range(8)]
    cases += [['candidate', str(n)] for n in range(4)]
    cases += [[name] for name in ('missing', 'repeat', 'cleanup', 'optional')]
    for variant in ('gx', 'dx'):
        run_cases('sensecap-indicator', 'touch', cases, variant=variant)


def test_indicator_touch_reset_retains_minimum_pulse_across_tick_rates():
    for tick_ms in (1, 10, 50):
        run_cases('sensecap-indicator', 'touch', [['0']], tick_ms=tick_ms)



def test_buttons_single_task_callback_lifetime_and_debounce():
    boards = ('sensecap-indicator',)
    cases = [[name] for name in ('0', '1', '2', 'repeat', 'replace', 'poll-only', 'samples', 'wrap', 'early-callback')]
    for board in boards:
        run_cases(board, 'button', cases)
        run_cases(board, 'button', [['min-tick']], tick_ms=50)



def test_indicator_example_notifies_display_owner():
    run_example_cases('sensecap-indicator', 'button_backlight', [['0']])



def test_touch_examples_do_not_publish_failed_samples():
    for board in ('sensecap-indicator',):
        run_example_cases(board, 'touch_read', [['0']])
