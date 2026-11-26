"""Actual standalone clients against loopback fixtures; no live service calls."""
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT/'tests/tools/skill_packages'))
from skill_harness import local_skill, invoke_contract

SID='codecraft-capabilities-image-generation'
OP='generate'


def test_standalone_local_and_archived_image_match_actual_output_contract():
    with local_skill() as (case, cloud):
        for archive in (False, True):
            destination = case.directory/('archived.png' if archive else 'local.png')
            before = len(case.requests)
            output = invoke_contract(case, cloud, SID, OP,
                                     {'prompt_file': str(case.prompt), 'output': str(destination)},
                                     {'archive': archive})
            assert output['file'] == str(destination.resolve())
            assert destination.read_bytes().startswith(b'\x89PNG\r\n\x1a\n')
            assert len(case.requests) == before+(2 if archive else 1)
            request = json.loads(case.requests[before][2])
            assert (request['n'], request['size'], request['response_format']) == (1, '1024x1024', 'b64_json')
            if archive:
                assert output['authentication'] == 'Device header required'
                assert output['content_url'].startswith(case.origin+'/deviceapi/media-assets/')
            else:
                assert set(output) == {'file'}


def test_standalone_image_rejects_existing_output_before_api_execution():
    with local_skill() as (case, cloud):
        destination = case.directory/'existing.png'
        destination.write_bytes(b'existing user file')
        invoke_contract(case, cloud, SID, OP,
                        {'prompt_file': str(case.prompt), 'output': str(destination)}, succeeds=False)
        assert destination.read_bytes() == b'existing user file' and case.requests == []


def test_service_prompt_bound_and_request_size_reach_the_isolated_client():
    with local_skill() as (case, cloud):
        limits = cloud[SID]['policy']['limits']
        maximum = limits['maximum_prompt_characters']
        case.prompt.write_text('a'*maximum)
        destination = case.directory/'maximum.png'
        invoke_contract(case, cloud, SID, OP, {'prompt_file': str(case.prompt), 'output': str(destination)})
        request = json.loads(case.requests[-1][2])
        assert request['size'] == cloud[SID]['policy']['defaults']['size']
        assert request['n'] == cloud[SID]['limits']['count']
        case.prompt.write_text('a'*(maximum+1))
        rejected = case.directory/'oversized.png'
        invoke_contract(case, cloud, SID, OP, {'prompt_file': str(case.prompt), 'output': str(rejected)}, succeeds=False)
        assert not rejected.exists() and len(case.requests) == 1
