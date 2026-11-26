//! ESP-IDF image codecs stay in the activated SDK; orchestration/receipts are Rust.
//! Format source: esptool 5 bin_image.py/cmds.py and SDK gen_esp32part.py.
use crate::util::*;
use anyhow::{ensure, Result};
use serde_json::json;
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
const SDK_CODEC: &str = r#"
import contextlib,hashlib,importlib.util,io,json,os,pathlib,sys
import esptool,esptool.bin_image,esptool.cmds
request=json.load(sys.stdin)
assert esptool.__version__.split('.')[0]=='5','esptool 5.x required'
def digest(b):return hashlib.sha256(b).hexdigest()
def image(chip,data):
    i=esptool.bin_image.LoadFirmwareImage(chip,data)
    assert i.chip_id==esptool.CHIP_DEFS[chip].IMAGE_CHIP_ID,'image chip mismatch'
    assert i.checksum==i.calculate_checksum(),'image checksum mismatch'
    assert i.append_digest and i.stored_digest==i.calc_digest,'image digest required'
    assert len(data)==i.data_length+32,'signed/padded/trailing image unsupported'
    return i
def parts(data,offset,capacity):
    path=pathlib.Path(os.environ['IDF_PATH'])/'components/partition_table/gen_esp32part.py'
    spec=importlib.util.spec_from_file_location('native_partitions',path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    module.offset_part_table=offset;table=module.PartitionTable.from_binary(data);table.verify()
    assert table,'empty partition table'
    rows=[]
    for p in table:
        assert not p.encrypted and p.size>0 and p.offset+p.size<=capacity,'invalid/encrypted partition'
        rows.append(dict(name=p.name,type=p.type,subtype=p.subtype,offset=p.offset,size=p.size))
    return rows
def normalized(chip,data,settings):
    i=image(chip,data);out=bytearray(data);cls=esptool.CHIP_DEFS[chip]
    out[2]=esptool.cmds.FLASH_MODES[settings['flash_mode']]
    out[3]=cls.parse_flash_size_arg(settings['flash_size'])|cls.parse_flash_freq_arg(settings['flash_freq'])
    out[i.data_length:i.data_length+32]=hashlib.sha256(out[:i.data_length]).digest()
    return bytes(out)
def tools():
    package=pathlib.Path(esptool.__file__).parent
    files={str(p.relative_to(package)):digest(p.read_bytes()) for p in sorted(package.rglob('*.py'))}
    partition=pathlib.Path(os.environ['IDF_PATH'])/'components/partition_table/gen_esp32part.py'
    return dict(esptool_version=esptool.__version__,esptool_sha256=digest(json.dumps(files,sort_keys=True,separators=(',',':')).encode()),partition_tool_sha256=digest(partition.read_bytes()),python=sys.version.split()[0])
def checked(path,record):
    data=pathlib.Path(path).read_bytes()
    assert len(data)==record['size'] and digest(data)==record['sha256'],'input changed'
    return data
with contextlib.redirect_stdout(io.StringIO()):
    op=request['operation'];chip=request['chip'];cls=esptool.CHIP_DEFS[chip]
    if op=='inspect':
        for path in request['images']: image(chip,pathlib.Path(path).read_bytes())
        result=dict(base_address=cls.BOOTLOADER_FLASH_OFFSET,tools=tools(),partitions=parts(pathlib.Path(request['partition']).read_bytes(),request['offset'],request['capacity']))
    elif op=='merge':
        layout=request['layout'];root=pathlib.Path(request['root']);assert tools()==layout['tools'],'SDK tools changed'
        inputs=[(s,checked(root/s['path'],s)) for s in layout['segments']]
        data=esptool.cmds.merge_bin([(s['offset'],b) for s,b in inputs],chip,**layout['flash_settings'],target_offset=layout['base_address'])
        regions=[];cursor=0
        for s,b in inputs:
            start=s['offset']-layout['base_address'];expected=normalized(chip,b,layout['flash_settings']) if s['role']=='bootloader' else b
            assert data[cursor:start]==b'\xff'*(start-cursor) and data[start:start+len(b)]==expected,'native merge content mismatch'
            regions.append(dict(offset=s['offset'],size=len(b),sha256=digest(expected),role=s['role']));cursor=start+len(b)
        assert cursor==len(data),'native merge trailing data'
        pathlib.Path(request['output']).write_bytes(data)
        result=dict(regions=regions,image=dict(file='firmware.bin',size=len(data),sha256=digest(data)))
    elif op=='verify':
        layout=request['layout'];data=pathlib.Path(request['image']).read_bytes();assert cls.BOOTLOADER_FLASH_OFFSET==layout['base_address'],'native base mismatch'
        for s in layout['segments']:
            start=s['offset']-layout['base_address'];b=data[start:start+s['size']]
            if s['role'] in ('bootloader','app'):image(chip,b)
            if s['role']=='bootloader':assert normalized(chip,b,layout['flash_settings'])==b,'boot parameters mismatch'
            if s['role']=='partition-table':assert parts(b,layout['partition_table_offset'],layout['flash_capacity'])==layout['partitions'],'partition mismatch'
        result=dict(status='passed')
    else:raise ValueError('unknown SDK codec operation')
print(json.dumps(result))
"#;
fn sdk(request: &J) -> Result<J> {
    let mut child = Command::new("python")
        .args(["-I", "-c", SDK_CODEC])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(serde_json::to_string(request)?.as_bytes())?;
    let out = child.wait_with_output()?;
    ensure!(
        out.status.success(),
        "ESP-IDF image codec failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    parse(&String::from_utf8(out.stdout)?)
}
fn record(root: &Path, path: &Path) -> Result<J> {
    let relative = path.strip_prefix(root)?;
    let safe = safe(root, &relative.to_string_lossy())?;
    ensure!(!safe.is_symlink(), "Firmware input symlink");
    let size = std::fs::metadata(&safe)?.len();
    ensure!(
        size > 0 && size <= 256 * 1024 * 1024,
        "Invalid firmware input size"
    );
    Ok(json!({"path":relative.to_string_lossy(),"size":size,"sha256":digest(&safe)?}))
}
pub fn layout(root: &Path, t: &J, device: &J, ex: &str, variant: Option<&str>) -> Result<J> {
    let mut build = root
        .join(".codecraft/builds")
        .join(s(&t["device"])?)
        .join(ex);
    if let Some(v) = variant {
        build = build.join(v);
    }
    let meta = read(&build.join("flasher_args.json"))?;
    let config = read(&build.join("config/sdkconfig.json"))?;
    let description = read(&build.join("project_description.json"))?;
    let chip = &meta["extra_esptool_args"]["chip"];
    ensure!(
        *chip == t["toolchain"]["target"]
            && *chip == config["IDF_TARGET"]
            && *chip == description["target"],
        "Native chip differs from Target"
    );
    for flag in [
        "SECURE_BOOT",
        "SECURE_FLASH_ENC_ENABLED",
        "SECURE_SIGNED_APPS_NO_SECURE_BOOT",
        "SECURE_SIGNED_ON_BOOT_NO_SECURE_BOOT",
        "SECURE_SIGNED_ON_UPDATE_NO_SECURE_BOOT",
    ] {
        ensure!(
            config.get(flag).is_none_or(|v| v == false || v.is_null()),
            "Signed/encrypted firmware unsupported"
        );
    }
    let settings = &meta["flash_settings"];
    for (name, key) in [
        ("flash_mode", "ESPTOOLPY_FLASHMODE"),
        ("flash_freq", "ESPTOOLPY_FLASHFREQ"),
        ("flash_size", "ESPTOOLPY_FLASHSIZE"),
    ] {
        ensure!(
            settings[name] == config[key],
            "Native Flash settings differ"
        );
    }
    let capacity = s(&settings["flash_size"])?
        .strip_suffix("MB")
        .ok_or_else(|| anyhow::anyhow!("Explicit flash capacity required"))?
        .parse::<u64>()?
        * 1024
        * 1024;
    ensure!(
        capacity > 0 && capacity <= 256 * 1024 * 1024,
        "Invalid Flash capacity"
    );
    let fact = &device["facts"]["flash"];
    let physical = if fact["status"] == "unknown" {
        json!({"status":"unknown","reason":"Physical Flash capacity is unresolved"})
    } else {
        ensure!(fact["unit"] == "MB", "Unsupported Flash fact unit");
        let limit = fact["value"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Invalid Flash fact"))?
            * 1024
            * 1024;
        ensure!(
            capacity <= limit,
            "Configured Flash exceeds sourced Device capacity"
        );
        json!({"status":fact["status"],"bytes":limit,"source":fact["source"]})
    };
    let files = obj(&meta["flash_files"])?;
    let mut roles = std::collections::BTreeMap::new();
    for role in ["bootloader", "partition-table", "app"] {
        let e = &meta[role];
        ensure!(
            e["encrypted"] == false || e["encrypted"] == "false",
            "Encrypted input unsupported"
        );
        ensure!(
            files.get(s(&e["offset"])?) == Some(&e["file"]),
            "Native role mismatch"
        );
        roles.insert(s(&e["offset"])?.to_owned(), role);
    }
    let mut segments = vec![];
    for (offset, file) in files {
        let address = if let Some(h) = offset.strip_prefix("0x") {
            u64::from_str_radix(h, 16)?
        } else {
            offset.parse()?
        };
        let mut segment = record(root, &safe(&build, s(file)?)?)?;
        segment["offset"] = json!(address);
        segment["role"] = json!(roles.get(offset).copied().unwrap_or("data"));
        segments.push(segment);
    }
    segments.sort_by_key(|s| s["offset"].as_u64());
    let table = segments
        .iter()
        .find(|s| s["role"] == "partition-table")
        .ok_or_else(|| anyhow::anyhow!("Missing partition table"))?;
    ensure!(
        table["offset"] == config["PARTITION_TABLE_OFFSET"],
        "Partition offset mismatch"
    );
    let images: Vec<_> = segments
        .iter()
        .filter(|s| s["role"] == "app" || s["role"] == "bootloader")
        .map(|s| root.join(s["path"].as_str().unwrap()))
        .collect();
    let native = sdk(
        &json!({"operation":"inspect","chip":chip,"images":images,"partition":root.join(s(&table["path"])?),"offset":table["offset"],"capacity":capacity}),
    )?;
    for segment in &mut segments {
        if segment["role"] == "data"
            && arr(&native["partitions"])?
                .iter()
                .any(|p| p["type"] == 0 && p["offset"] == segment["offset"])
        {
            segment["role"] = json!("app");
        }
    }
    let mut metadata = vec![];
    for name in [
        "flasher_args.json",
        "config/sdkconfig.json",
        "project_description.json",
        "CMakeCache.txt",
    ] {
        metadata.push(record(root, &build.join(name))?);
    }
    let config_path = Path::new(s(&description["config_file"])?);
    metadata.push(record(
        root,
        &if config_path.is_absolute() {
            config_path.to_owned()
        } else {
            root.join(config_path)
        },
    )?);
    let end = segments
        .iter()
        .map(|s| s["offset"].as_u64().unwrap() + s["size"].as_u64().unwrap())
        .max()
        .unwrap_or(0);
    let document = json!({"schema_version":1,"chip":chip,"address_space":"flash","format":"bin","purpose":"factory","base_address":native["base_address"],"flash_capacity":capacity,"partition_table_offset":config["PARTITION_TABLE_OFFSET"],"flash_settings":settings,"physical_flash":physical,"fill":255,"segments":segments,"partitions":native["partitions"],"metadata":metadata,"tools":native["tools"],"write_range":{"start":native["base_address"],"end":end}});
    validate(&document)?;
    Ok(document)
}
pub fn validate(d: &J) -> Result<()> {
    ensure!(
        d["schema_version"] == 1
            && d["address_space"] == "flash"
            && d["fill"] == 255
            && d["format"] == "bin"
            && d["purpose"] == "factory",
        "Invalid layout identity"
    );
    let mut end = d["base_address"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("Invalid base"))?;
    let capacity = d["flash_capacity"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("Invalid capacity"))?;
    ensure!(
        capacity > 0 && capacity <= 256 * 1024 * 1024,
        "Invalid capacity"
    );
    let segments = arr(&d["segments"])?;
    for seg in segments {
        let offset = seg["offset"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Invalid offset"))?;
        let size = seg["size"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Invalid size"))?;
        ensure!(
            size > 0 && offset >= end && offset % 4096 == 0,
            "Overlapping/unaligned segment"
        );
        end = offset
            .checked_add(size)
            .ok_or_else(|| anyhow::anyhow!("Segment overflow"))?;
        ensure!(end <= capacity, "Image exceeds capacity");
        match s(&seg["role"])? {
            "bootloader" => ensure!(
                seg["offset"] == d["base_address"]
                    && end <= d["partition_table_offset"].as_u64().unwrap_or(0),
                "Bootloader outside reserved region"
            ),
            "partition-table" => ensure!(
                seg["offset"] == d["partition_table_offset"] && size <= 4096,
                "Invalid partition table placement"
            ),
            "app" | "data" => {
                let matches: Vec<_> = arr(&d["partitions"])?
                    .iter()
                    .filter(|p| {
                        p["offset"] == seg["offset"]
                            && p["offset"]
                                .as_u64()
                                .zip(p["size"].as_u64())
                                .and_then(|(a, b)| a.checked_add(b))
                                .is_some_and(|n| end <= n)
                    })
                    .collect();
                ensure!(
                    matches.len() == 1 && ((seg["role"] == "app") == (matches[0]["type"] == 0)),
                    "Image outside matching partition"
                );
            }
            _ => anyhow::bail!("Unknown segment role"),
        }
    }
    for role in ["bootloader", "partition-table"] {
        ensure!(
            segments.iter().filter(|s| s["role"] == role).count() == 1,
            "Missing/duplicate native role"
        );
    }
    ensure!(
        segments.iter().any(|s| s["role"] == "app")
            && d["write_range"] == json!({"start":d["base_address"],"end":end}),
        "Invalid write range"
    );
    Ok(())
}
pub fn merge(root: &Path, layout: &J, stage: &Path) -> Result<J> {
    validate(layout)?;
    for r in arr(&layout["metadata"])?
        .iter()
        .chain(arr(&layout["segments"])?.iter())
    {
        let current = record(root, &safe(root, s(&r["path"])?)?)?;
        for k in ["path", "size", "sha256"] {
            ensure!(current[k] == r[k], "Firmware input changed");
        }
    }
    sdk(
        &json!({"operation":"merge","chip":layout["chip"],"root":root,"layout":layout,"output":stage.join("firmware.bin")}),
    )
}
pub fn verify(bundle: &Path, manifest: &J) -> Result<J> {
    let d = &manifest["layout"];
    validate(d)?;
    ensure!(
        manifest["image"]["file"] == "firmware.bin",
        "Unexpected image file"
    );
    let image = safe(bundle, "firmware.bin")?;
    ensure!(
        !image.is_symlink() && !bundle.join("manifest.json").is_symlink(),
        "Release files cannot be links"
    );
    ensure!(
        std::fs::read_dir(bundle)?.count() == 2,
        "Unexpected release files"
    );
    let bytes = std::fs::read(image)?;
    ensure!(
        Some(bytes.len() as u64) == manifest["image"]["size"].as_u64()
            && hash(&bytes) == s(&manifest["image"]["sha256"])?,
        "Image size/hash mismatch"
    );
    let base = d["base_address"].as_u64().unwrap();
    ensure!(
        bytes.len() as u64 == d["write_range"]["end"].as_u64().unwrap() - base,
        "Image range mismatch"
    );
    let regions = arr(&manifest["regions"])?;
    let segments = arr(&d["segments"])?;
    ensure!(regions.len() == segments.len(), "Region coverage mismatch");
    let mut cursor = 0;
    for (r, segment) in regions.iter().zip(segments) {
        for k in ["offset", "size", "role"] {
            ensure!(r[k] == segment[k], "Region metadata mismatch");
        }
        let start = (r["offset"].as_u64().unwrap() - base) as usize;
        let end = start + r["size"].as_u64().unwrap() as usize;
        ensure!(
            end <= bytes.len() && bytes[cursor..start].iter().all(|b| *b == 255),
            "Padding mismatch"
        );
        ensure!(
            hash(&bytes[start..end]) == s(&r["sha256"])?,
            "Region hash mismatch"
        );
        if r["role"] != "bootloader" {
            ensure!(r["sha256"] == segment["sha256"], "Input hash mismatch");
        }
        cursor = end;
    }
    sdk(
        &json!({"operation":"verify","chip":d["chip"],"layout":d,"image":bundle.join("firmware.bin")}),
    )
}
