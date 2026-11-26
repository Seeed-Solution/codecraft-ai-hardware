"""Run the native scenarios owned by this Target or component."""
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[4]/'tests/platforms/esp-idf'))
from host_io import run_cases


def test_indicator_display_readiness_bitbang_errors_and_ownership():
    cases=[[name] for name in ('0','null','unready')]
    cases += [[str(n)] for n in range(1,21)]
    cases += [['20','40']]
    # Exercise later register words and the idle line writes too.
    cases += [[str(n),'0',str(occurrence)] for n in (17,18,19) for occurrence in (10,100)]
    cases += [[str(n),'0',str(last)] for n,last in ((17,487),(18,4990),(19,2188))]
    run_cases('sensecap-indicator','display',cases)
    # Complete GX GPIO/register/delay trace, including the 5 ms reset pulse.
    assert run_cases('sensecap-indicator','display',[['wire']])==['17802739854524639500 487 4990 2188\n']



def test_indicator_expander_rollback_shadow_recovery_and_concurrent_pins():
    cases=[[name] for name in ('0','fallback','missing','concurrent','lock-timeout')]
    cases += [[str(n)] for n in (1,2,5,6,7,8,9)]
    cases += [['9',str(n)] for n in (10,11)]
    cases += [[name,str(applied)] for name in ('shadow-output','shadow-config') for applied in (0,1)]
    run_cases('sensecap-indicator','expander',cases)


def test_indicator_dx_avoids_register_writes_and_retains_failure_recovery():
    cases=[['0'],['null'],['unready']]+[[str(n)] for n in range(1,16)]
    cases += [['20'],['20','40']]
    run_cases('sensecap-indicator','display',cases,variant='dx')


def test_indicator_explicit_display_policy_and_pwm_errors():
    for variant in ('gx', 'dx'):
        run_cases('sensecap-indicator', 'display', [['config'], ['config','22'], ['config','23']], variant=variant)
