#!/usr/bin/python3

import xml.etree.ElementTree as ET
import os, shutil, subprocess, sys

from svdwash import (clusterfy, deprefix, peripheral_derivatives,
                     register_array, register_derivatives)

SVD2RUST='/home/mirror/svd2rust/target/debug/svd2rust'

scriptdir = os.path.dirname(sys.argv[0])
if scriptdir != '':
    os.chdir(scriptdir)

svd = ET.parse('STM32H503.svd')

alternates_remove = {
    'I3C_CR_ALTERNATE',
    'USART_CR1_disabled',
    'USART_ISR_disabled',
    'LPUART_CR1_disabled',
    'LPUART_ISR_disabled',
}
alternates_keep = {
    'USART_CR1_enabled': 'USART_CR1',
    'USART_ISR_enabled': 'USART_ISR',
    'LPUART_CR1_enabled': 'LPUART_CR1',
    'LPUART_ISR_enabled': 'LPUART_ISR',
}

deprefix(svd, alternates_remove, alternates_keep)

usb = svd.find(".//peripheral[name='USB']")
register_array(usb, 'CHEP0R', 'CHEPR[%s]', [f'CHEP{i}R' for i in range(8)]);

svd.write('washed.svd')

assert os.path.exists('wash-svd.py')

shutil.rmtree('raw', ignore_errors=True)
shutil.rmtree('src', ignore_errors=True)
os.mkdir('raw')
os.mkdir('src')

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
