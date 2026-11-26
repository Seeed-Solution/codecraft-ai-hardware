"""Actual standalone clients against loopback fixtures; no live service calls."""
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT/'tests/tools/skill_packages'))
from skill_harness import local_skill, invoke_contract

SID='codecraft-capabilities-tts'
OP='speak'


def test_standalone_local_and_archived_speech_match_actual_output_contract():
    with local_skill() as (case, cloud):
        for archive in (False, True):
            destination = case.directory/('archived.wav' if archive else 'local.wav')
            before = len(case.requests)
            output = invoke_contract(case, cloud, SID, OP,
                                     {'text_file': str(case.prompt), 'output': str(destination)},
                                     {'archive': archive})
            assert output['file'] == str(destination.resolve()) and destination.is_file()
            assert 'voice' not in json.loads(case.requests[before][2])
            assert len(case.requests) == before+(2 if archive else 1)
            if archive:
                assert output['authentication'] == 'Device header required'
                assert output['content_url'].startswith(case.origin+'/deviceapi/media-assets/')
            else:
                assert set(output) == {'file'}


def test_standalone_archive_failure_keeps_file_and_nonzero_exit():
    with local_skill() as (case, cloud):
        case.mode = 'archive-fail'
        destination = case.directory/'speech.wav'
        run = invoke_contract(case, cloud, SID, OP,
                              {'text_file': str(case.prompt), 'output': str(destination)},
                              {'archive': True}, succeeds=False)
        assert destination.is_file() and json.loads(run.stdout)['file'] == str(destination.resolve())
        assert len(case.requests) == 2


def test_service_text_bound_and_fixed_routing_reach_the_isolated_client():
    with local_skill() as (case, cloud):
        policy = cloud[SID]['policy']
        maximum = policy['limits']['maximum_text_characters']
        case.prompt.write_text('a'*maximum)
        destination = case.directory/'maximum.wav'
        invoke_contract(case, cloud, SID, OP, {'text_file': str(case.prompt), 'output': str(destination)})
        request = json.loads(case.requests[-1][2])
        assert not {'model','provider','voice'} & request.keys()
        assert request['response_format'] == policy['limits']['format'].lower()
        assert len(case.requests) == 1
        case.prompt.write_text('a'*(maximum+1))
        rejected = case.directory/'oversized.wav'
        invoke_contract(case, cloud, SID, OP, {'text_file': str(case.prompt), 'output': str(rejected)}, succeeds=False)
        assert not rejected.exists() and len(case.requests) == 1
        case.prompt.write_text('valid text')
        for flag in ('--voice','--model','--provider'):
            assert case.invoke('codecraft-capabilities-tts','speak.py','--text-file',str(case.prompt),
                               '--output',str(rejected),flag,'unsupported').returncode != 0
        assert not rejected.exists() and len(case.requests)==1
