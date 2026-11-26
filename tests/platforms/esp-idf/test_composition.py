"""Native selection, metadata projection, portable files and compiled graph agree."""
from contextlib import contextmanager
import copy
import json
from pathlib import Path
import shutil
import sys
import tempfile

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools/codecraft'))
sys.path.insert(0,str(ROOT/'platforms/esp-idf'))
from io_utils import ContractError,read_json,write_json
from model import Catalog
from project import generate_project,verify_project
import composition


@contextmanager
def application(target='esp-idf/sensecap-watcher',example='knob_input'):
    with tempfile.TemporaryDirectory(prefix='codecraft-composition-') as folder:
        output=Path(folder)/'app'
        generate_project(Catalog(ROOT),target,example,output)
        yield output,Catalog(output).target(target),example


def rejected(function,fragment):
    try:function()
    except ContractError as error:assert fragment in str(error),(fragment,str(error))
    else:raise AssertionError('Expected composition rejection')


def test_registered_examples_keep_native_dependencies_and_interface_scope():
    catalog=Catalog(ROOT);count=0
    for target in catalog.targets.values():
        if target['platform']!='esp-idf':continue
        composition.validate(ROOT,target)
        all_sources=set()
        for example in target['examples']:
            choice=composition.describe(ROOT,target,example);count+=1
            all_sources.update(choice['sources'])
            projected=catalog.selected_target(target['id'],example)
            for interface in projected['interfaces']:
                assert not interface['implementation']
                assert set(interface['example_implementation'])=={example}
            defaults=(ROOT/target['path']/'examples'/example/'sdkconfig.defaults').read_text()
            if 'CONFIG_SPIRAM=y' in defaults:
                assert 'esp_psram' in (ROOT/target['path']/'examples'/example/'main/CMakeLists.txt').read_text()
        assert all_sources==set((ROOT/target['path']/'bsp/src').glob('*.c'))
    assert count==29
    expectations={
      ('esp-idf/wio-tracker-l2','wifi_scan'):({'bsp_wifi.c'},{'wireless'}),
      ('esp-idf/sensecap-watcher','knob_input'):({'bsp_knob.c','bsp_io_expander.c'},{'gpio','input'}),
      ('esp-idf/sensecap-watcher','touch_read'):({'bsp_display.c','bsp_touch.c','bsp_io_expander.c'},{'display','gpio','touch'}),
      ('esp-idf/reterminal-e1001','epd_hello'):({'bsp_epd.c','bsp_led.c'},{'display','led'}),
      ('esp-idf/reterminal-sticky','button_hello'):({'bsp_button.c'},{'input'})}
    for (tid,example),(sources,interfaces) in expectations.items():
        target=catalog.target(tid)
        assert {p.name for p in composition.describe(ROOT,target,example)['sources']}==sources
        assert {item['id'] for item in catalog.selected_target(tid,example)['interfaces']}==interfaces


def test_every_native_example_exports_its_closed_composition_and_locks_the_profile():
    catalog=Catalog(ROOT)
    with tempfile.TemporaryDirectory(prefix='codecraft-composition-exports-') as folder:
        for target in catalog.targets.values():
            if target['platform']!='esp-idf':continue
            for example in target['examples']:
                output=Path(folder)/(target['device']+'-'+example)
                generate_project(catalog,target['id'],example,output)
                projected=Catalog(output).target(target['id'])
                composition.validate(output,projected)
                assert verify_project(output)['status']=='passed'
                assert not any((output/'tests').rglob('test_*.py'))
                profile=output/target['path']/'examples'/example/'bsp.cmake'
                lock=read_json(output/'codecraft.lock')
                assert profile.relative_to(output).as_posix() in lock['files']
                profile.write_text(profile.read_text()+'\n# unreviewed application edit\n')
                assert verify_project(output)['status']=='failed'


def test_composition_syntax_rejects_escape_duplicate_empty_and_unmodeled_commands():
    with tempfile.TemporaryDirectory() as folder:
        path=Path(folder)/'bsp.cmake'
        valid='set(CODECRAFT_BSP_SOURCES "src/bsp_button.c")\nset(CODECRAFT_BSP_REQUIRES freertos log)\n'
        path.write_text(valid);assert composition.settings(path)['CODECRAFT_BSP_SOURCES']==['src/bsp_button.c']
        invalid=[valid.replace('src/bsp_button.c','../external.c'),
                 valid.replace('src/bsp_button.c','${OTHER_SOURCE}'),
                 valid.replace('freertos log','freertos log log'),
                 valid.replace('freertos log',''),valid+valid,
                 valid+'include(other.cmake)\n',valid.replace('CODECRAFT_BSP_REQUIRES','UNKNOWN')]
        for text in invalid:
            path.write_text(text)
            rejected(lambda:composition.settings(path),'composition')


def test_omitted_native_module_and_broad_bsp_input_are_rejected():
    with application() as (root,target,example):
        profile=root/target['path']/'examples'/example/'bsp.cmake';text=profile.read_text()
        profile.write_text(text.replace('    "src/bsp_io_expander.c"\n',''))
        rejected(lambda:composition.validate(root,target),'no selected implementation')
        profile.write_text(text)
        source=ROOT/target['path']/'bsp/src/bsp_camera.c'
        shutil.copy2(source,root/target['path']/'bsp/src'/source.name)
        target['inputs'].append(target['path']+'/bsp')
        rejected(lambda:composition.validate(root,target),'source closure differs')


def test_missing_shared_component_declaration_and_broad_native_directories_fail():
    with application('esp-idf/reterminal-e1001','board_health') as (root,target,example):
        original=copy.deepcopy(target)
        target['example_dependencies'][example].remove('platforms/esp-idf/components/codecraft_pcf8563')
        rejected(lambda:composition.validate(root,target),'component dependencies differ')
        path=root/target['path']/'examples'/example/'CMakeLists.txt'
        path.write_text(path.read_text().replace('/components/codecraft_sht4x','/components'))
        rejected(lambda:composition.validate(root,original),'component directories differ')


def test_successful_compiler_output_must_still_match_the_registered_bsp_graph():
    catalog=Catalog(ROOT);target=catalog.target('esp-idf/sensecap-watcher');example='knob_input'
    selected=composition.describe(ROOT,target,example)
    with tempfile.TemporaryDirectory() as folder:
        output=Path(folder)
        value={'build_components':['main','bsp',*selected['requirements']],
               'build_component_info':{'bsp':{'sources':[str(path) for path in selected['sources']],
                                              'reqs':sorted(selected['requirements'])}}}
        write_json(output/'project_description.json',value)
        assert len(composition.verify_build(ROOT,target,example,output)['bsp_sources'])==2
        changed=copy.deepcopy(value);changed['build_component_info']['bsp']['sources'].append(str(ROOT/target['path']/'bsp/src/bsp_audio.c'))
        write_json(output/'project_description.json',changed)
        rejected(lambda:composition.verify_build(ROOT,target,example,output),'built BSP sources differ')
        changed=copy.deepcopy(value);changed['build_component_info']['bsp']['reqs'].append('esp_codec_dev')
        write_json(output/'project_description.json',changed)
        rejected(lambda:composition.verify_build(ROOT,target,example,output),'built BSP requirements differ')
