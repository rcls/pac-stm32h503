#!/usr/bin/python3

import os, shutil, subprocess
import xml.etree.ElementTree as ET

from typing import TypeAlias, TypeVar, TYPE_CHECKING

if TYPE_CHECKING:
    Element: TypeAlias = ET.Element[str]
    ElementTree: TypeAlias = ET.ElementTree[Element]
else:
    Element: TypeAlias = ET.Element
    ElementTree: TypeAlias = ET.ElementTree

def find(node: Element, f: str) -> Element:
    field = node.find(f)
    assert field is not None
    return field

def get_text(node: Element, f: str = 'name') -> str:
    field = node.find(f)
    assert field is not None and field.text is not None
    return field.text

def num_field(node: Element, field) -> int:
    e = node.find(field)
    assert e is not None and e.text is not None
    return int(e.text, 0)

def addressoffset(n: Element) -> int:
    return num_field(n, 'addressOffset')

T = TypeVar('T')
def not_none(v: T | None) -> T:
    assert v is not None
    return v

def deprefix(svd: ElementTree, alternates_remove: set[str],
             alternates_keep: dict[str, str]) -> None:
    for registers in svd.findall('.//registers'):
        for register in registers.findall('register'):
            name = get_text(register)
            if name in alternates_remove:
                assert not name in alternates_keep, f'!!! {name}'
                registers.remove(register)
                continue
            if name in alternates_keep:
                find(register, 'name').text = alternates_keep[name]
                continue
            assert not 'ALTERNATE' in name, f'??? {name}'

    for peripheral in svd.findall('.//peripheral'):
        print(get_text(peripheral))
        names = []
        for register in peripheral.findall('registers/register'):
            names.append(register.find('name'))
        common_prefix = None
        for name in names:
            n = name.text
            if common_prefix is None:
                common_prefix = n
            else:
                while not n.startswith(common_prefix):
                    common_prefix = common_prefix[:-1]
        print(f' {common_prefix}')
        if common_prefix is None or not '_' in common_prefix:
            continue
        common_prefix = common_prefix.rsplit('_', 1)[0] + '_'
        for name in names:
            name.text = name.text.removeprefix(common_prefix)

def register_array(peripheral: Element,
                   first, pattern, items, increment: int|None = None):
    assert first in items, f'{first} {items}'
    registers = peripheral.find('registers')
    assert registers is not None
    prototype = registers.find(f"register[name='{first}']")
    print(registers.find('register'))
    assert prototype is not None, [
        name.text for name in registers.findall('register/name')]
    find(prototype, 'name').text = pattern
    assert prototype.find('dim') == None
    assert prototype.find('dimIncrement') == None
    if increment is None:
        increment = num_field(prototype, 'size') // 8
    dim = len(items)
    ET.SubElement(prototype, 'dim').text = f'{dim}'
    ET.SubElement(prototype, 'dimIncrement').text = f'{increment}'
    children: list[Element] = registers.findall('register')
    assert type(children) == list
    for r in children:
        name = find(r, 'name')
        if name != first and name.text in items:
            registers.remove(r)

def register_derivatives(peripheral: Element,
                         prototype: str, derived: list[str]) -> None:
    registers = peripheral.find('registers')
    assert registers is not None
    proto = registers.find(f"register[name='{prototype}']")
    assert proto is not None
    for d in derived:
        reg = find(registers, f"register[name='{d}']")
        assert reg != proto, f'{prototype} {d} {proto} {reg}'
        reg.set('derivedFrom', prototype)
        reg.remove(find(reg, 'fields'))

def peripheral_derivatives(svd, prototype: str, derived: list[str]) -> None:
    peripherals = svd.find('.//peripherals')
    proto = peripherals.find(f"peripheral[name='{prototype}']")
    # print('Prototype {prototype} -> {proto}')
    assert proto is not None
    for d in derived:
        periph = peripherals.find(f"peripheral[name='{d}']")
        # print(f'Periph {d} -> {periph}')
        assert periph != proto
        periph.set('derivedFrom', prototype)
        for r in periph.findall('registers/register'):
            name = get_text(r)
            s = proto.find(f"registers/register[name='{name}']")
            assert s != None, name
            assert num_field(r, 'size') == num_field(s, 'size')
        periph.remove(periph.find('registers'))

def clusterfy(peripheral: Element, name: str, fields: list[str],
              replaced: list[list[str]], proto_index: int = 0) -> None:
    registers = find(peripheral, 'registers')

    reg_by_name = {}
    for reg in registers.findall('register'):
        reg_by_name[get_text(reg)] = reg
    #print(reg_by_name)

    for r in replaced:
        assert len(r) == len(fields)
    assert len(replaced) >= 2, "We don't handle singletons..."

    rep_regs: list[list[Element | None]]
    rep_regs = [list(map(reg_by_name.get, rr)) for rr in replaced]
    offsets = [addressoffset(not_none(l[0])) for l in rep_regs]
    #print(rep_regs)

    # Check that the register have regular strides.
    base = offsets[0]
    stride = offsets[1] - base
    # Check that the addresses all match for clusterification.
    proto_regs = rep_regs[proto_index]
    for i, rr in enumerate(rep_regs):
        for r, s in zip(rr, proto_regs):
            if r == None:
                continue
            assert s is not None
            assert addressoffset(r) - addressoffset(s) \
                == stride * (i - proto_index)
            assert num_field(r, 'size') == num_field(s, 'size')
            assert get_text(r, 'resetValue') == get_text(s, 'resetValue') \
                or num_field(s, 'resetValue') == 0
    # Now build the cluster.
    # Ugh...  place it in order.
    index = None
    for n, e in enumerate(registers):
        if e == rep_regs[0][0]:
            index = n
            break
    else:
        assert None, 'Bugger'
    cluster = ET.Element('cluster')
    registers.insert(index, cluster)
    for rr in rep_regs:
        for r in rr:
            if r is not None:
                registers.remove(r)
    ET.SubElement(cluster, 'dim').text = str(len(rep_regs))
    ET.SubElement(cluster, 'dimIncrement').text = str(stride)
    ET.SubElement(cluster, 'name').text = name
    ET.SubElement(cluster, 'description').text = f'Cluster for {name}'
    ET.SubElement(cluster, 'addressOffset').text = f'{base:#x}'
    proto_base = offsets[proto_index]
    for r, name in zip(rep_regs[proto_index], fields):
        assert r is not None
        cluster.append(r)
        find(r, 'name').text = name
        find(r, 'addressOffset').text = hex(addressoffset(r) - proto_base)

def output_and_generate(svd: ElementTree):
    svd.write('washed.svd')
    assert os.path.exists('svdwash.py')
    assert os.path.exists('wash-svd.py')

    shutil.rmtree('raw', ignore_errors=True)
    shutil.rmtree('src', ignore_errors=True)
    os.mkdir('raw')
    os.mkdir('src')

    SVD2RUST='/home/mirror/svd2rust/target/debug/svd2rust'
    subprocess.run([SVD2RUST, '--ident-formats-theme', 'legacy',
                    '-f', 'register_accessor:::',
                    '-f', 'field_accessor:::',
                    '-f', 'enum_value:::',
                    '-f', 'enum_value_accessor:::',
                    '-f', 'cluster_accessor:::',
                    #'-f', 'peripheral_mod:::',
                    '-o', 'raw', '-i', 'washed.svd'],
                   check=True)
    subprocess.run(['form', '-i', 'raw/lib.rs', '-o', 'src'])
    subprocess.run(
        ['rustfmt', '--edition', '2021', '--emit', 'files', 'src/lib.rs'])
