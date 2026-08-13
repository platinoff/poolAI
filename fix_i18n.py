#!/usr/bin/env python3
import re

d = open('crates/poolai-ui-core/src/i18n.rs', 'rb').read()

# Find the admin.gpuLimits section and add admin.debug keys after it
# Look for the Ukrainian gpuLimits section end marker
idx = d.find(b'GPU \xd0\xbb\xd1\x96\xd0\xbc\xd1\x96\xd0\xb2: \"')
if idx >= 0:
    insert_pos = idx + 80
    new_keys = b'''(
    "admin.debug.migrationLabel", "GPU limits debug:"),
    ("admin.debug.migrationRefreshOk", "GPU limits debug refreshed"),
    ("admin.debug.migrationRefreshErr", "GPU limits debug refresh failed: ")
'''
    d = d[:insert_pos] + new_keys + d[insert_pos:]
    open('crates/poolai-ui-core/src/i18n.rs', 'wb').write(d)
    print('Added admin.debug i18n keys at position', insert_pos)
else:
    print('Could not find gpuLimits Ukrainian section')
    # Try alternative: find last admin.gpuLimits
    matches = list(re.finditer(b'admin\\.gpuLimits', d))
    if matches:
        last = matches[-1]
        print('Last admin.gpuLimits at position', last.start())