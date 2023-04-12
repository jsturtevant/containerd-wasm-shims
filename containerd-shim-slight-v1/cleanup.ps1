ps *shim* | kill
ctr c rm slight
ctr snapshots rm slight
crictl rm --all
crictl rmp --all
rm -recurse -force C:\ProgramData\containerd\state\io.containerd.runtime.v2.task\default\slight