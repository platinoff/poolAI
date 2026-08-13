d=open("crates/poolai-ui-core/src/i18n.rs","rb").read()
insert_pos=34963
new_keys=b'("admin.debug.migrationLabel", "GPU limits debug:"), ("admin.debug.migrationRefreshOk", "GPU limits debug refreshed"), ("admin.debug.migrationRefreshErr", "GPU limits debug refresh failed: ")'
d=d[:insert_pos]+new_keys+d[insert_pos:]
open("crates/poolai-ui-core/src/i18n.rs","wb").write(d)
print("Added admin.debug i18n keys at position",insert_pos)