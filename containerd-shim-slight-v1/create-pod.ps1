crictl.exe runp -r slight .\pod.json 
$POD_ID=(crictl.exe  runp -r slight .\pod.json)
$CONTAINER_ID=(crictl.exe create $POD_ID .\container-wasm.json .\pod.json)
crictl.exe start $CONTAINER_ID