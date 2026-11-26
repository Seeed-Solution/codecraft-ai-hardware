"""Actual standalone clients against loopback fixtures; no live service calls."""
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT/'tests/tools/skill_packages'))
from skill_harness import local_skill, invoke_contract

SID='codecraft-capabilities-llm'
OP='complete'


def test_standalone_prompt_and_messages_match_actual_client_contract():
    with local_skill() as (case, cloud):
        for key, content in [('prompt_file', 'Hello device cloud.'),
                             ('messages_file', [{'role': 'user', 'content': 'Hello device cloud.'}])]:
            source = case.directory/key
            source.write_text(json.dumps(content) if isinstance(content, list) else content)
            output = invoke_contract(case, cloud, SID, 'complete', {key: str(source)}, {'max_tokens': 73})
            assert output == {'text': 'hello', 'usage': {'total_tokens': 2}, 'finish_reason':'stop','complete':True}
            request = json.loads(case.requests[-1][2])
            assert request['max_tokens'] == 73 and request['stream'] is False
            assert request['messages'] == [{'role': 'user', 'content': 'Hello device cloud.'}]
        before=len(case.requests)
        for args in ([],['--prompt-file',str(case.prompt),'--messages-file',str(case.prompt)],
                     ['--prompt-file',''],['--prompt-file',str(case.prompt),'--unknown-option']):
            assert case.invoke('codecraft-capabilities-llm','llm.py',*args).returncode != 0
        assert len(case.requests)==before


def test_standalone_stream_requires_done_and_preserves_failure_status():
    with local_skill() as (case, cloud):
        inputs = {'prompt_file': str(case.prompt)}
        assert invoke_contract(case, cloud, SID, 'stream', inputs).strip() == 'hello'
        assert json.loads(case.requests[-1][2])['stream'] is True
        case.mode = 'incomplete'
        before = len(case.requests)
        run = invoke_contract(case, cloud, SID, 'stream', inputs, succeeds=False)
        assert run.stdout == 'hello' and len(case.requests) == before+1


def test_service_token_default_and_caller_override_reach_the_isolated_client():
    with local_skill() as (case, cloud):
        default = cloud[SID]['policy']['defaults']['max_tokens']
        inputs = {'prompt_file': str(case.prompt)}
        for configuration, expected in (({}, default), ({'max_tokens': 7}, 7)):
            invoke_contract(case, cloud, SID, OP, inputs, configuration)
            assert json.loads(case.requests[-1][2])['max_tokens'] == expected
        before = len(case.requests)
        for value in ('0','True','-1','2049'):
            run=case.invoke('codecraft-capabilities-llm','llm.py','--prompt-file',str(case.prompt),'--max-tokens',value)
            assert run.returncode != 0
        assert len(case.requests) == before
