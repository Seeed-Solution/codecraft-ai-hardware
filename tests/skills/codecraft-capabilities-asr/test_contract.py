"""Actual standalone clients against loopback fixtures; no live service calls."""
import io
import importlib.util
import json
from pathlib import Path
import sys
import wave
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT/'tests/tools/skill_packages'))
from skill_harness import local_skill, invoke_contract
from skill_harness import load_package, payload, policy

SID='codecraft-capabilities-asr'
OP='transcribe'


def test_realtime_contract_matches_actual_standalone_operations():
    spec=importlib.util.spec_from_file_location('asr_wire_fixture',ROOT/'tests/skills/codecraft-capabilities-asr/test_realtime.py')
    fixture=importlib.util.module_from_spec(spec);spec.loader.exec_module(fixture)
    case=fixture.RealtimeSkillTests();case.setUp()
    try:
        record,contract=load_package(ROOT,'codecraft-capabilities-asr')
        schemas=policy(contract,record['files'])['operations']
        for operation,inputs in (
                ('realtime-stream',{'file':str(case.audio),'idempotency_key':'contract-key-123'}),
                ('realtime-status',{'session_id':fixture.SID}),
                ('realtime-recover',{'idempotency_key':'contract-key-123'})):
            mapping=contract['client']['operations'][operation]
            assert mapping['entrypoint']=='scripts/realtime.py'
            payload(schemas[operation]['input'],inputs)
            args=list(mapping['command'])
            for key,value in inputs.items():args.extend([mapping['arguments'][key],value])
            before=len(case.requests)
            run=case.invoke(*args)
            assert run.returncode==0,run.stderr
            output=run.stdout if operation=='realtime-stream' else json.loads(run.stdout)
            payload(schemas[operation]['output'],output)
            binding=contract['api']['operations'][operation]
            assert len(case.requests)==before+1
            assert case.requests[-1][:2]==(binding['method'],binding['path'].replace('{session_id}',fixture.SID))
            if operation=='realtime-stream':
                assert json.loads(run.stdout.splitlines()[-1])['type']=='transcript.result'
        assert len(case.ws_requests)==1
    finally:
        case.tearDown()


def test_standalone_transcription_matches_actual_client_contract():
    with local_skill() as (case, cloud):
        output = invoke_contract(case, cloud, SID, OP, {'file': str(case.audio)})
        assert output == {'text': 'hello'}
        assert case.audio.read_bytes() in case.requests[0][2]


def test_standalone_invalid_audio_fails_before_api_execution():
    with local_skill() as (case, cloud):
        case.audio.write_bytes(b'not a WAV file')
        invoke_contract(case, cloud, SID, OP, {'file': str(case.audio)}, succeeds=False)
        assert case.requests == []


def test_service_audio_boundaries_are_enforced_by_the_isolated_client():
    with local_skill() as (case, cloud):
        limits = cloud[SID]['policy']['limits']
        assert (limits['container'], limits['encoding'], limits['channels'], limits['sample_bits']) == ('WAV', 'PCM', 1, 16)

        def recording(rate, seconds, channels=1, bits=16):
            buffer = io.BytesIO()
            with wave.open(buffer, 'wb') as audio:
                audio.setnchannels(channels)
                audio.setsampwidth(bits//8)
                audio.setframerate(rate)
                audio.writeframes(bytes(rate*seconds*channels*(bits//8)))
            return buffer.getvalue()

        rates = limits['sample_rate_hz']
        duration = limits['maximum_duration_seconds']
        for rate in (rates['minimum'], rates['maximum']):
            case.audio.write_bytes(recording(rate, duration))
            assert invoke_contract(case, cloud, SID, OP, {'file': str(case.audio)}) == {'text': 'hello'}
        before = len(case.requests)
        invalid = [recording(rates['minimum'], duration+1),
                   recording(rates['minimum']-1, 1), recording(rates['maximum']+1, 1),
                   recording(rates['minimum'], 1, channels=2), recording(rates['minimum'], 1, bits=8),
                   bytes(limits['maximum_file_bytes']+1)]
        for content in invalid:
            case.audio.write_bytes(content)
            invoke_contract(case, cloud, SID, OP, {'file': str(case.audio)}, succeeds=False)
        assert len(case.requests) == before
