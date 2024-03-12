#! /bin/bash
# dir="/afs/cern.ch/user/a/asantra/Light-By-Light/ptarmigan/examples"
# sftp asantra@lxplus.cern.ch << EOF
# put E320_profile.yml $dir
# EOF
dir="/storage/agrp/arkas/PtarmiganH52Root"
sftp arkas@wipp-an1 << EOF
put /Users/arkasantra/arka/Tom_Work_Theory/Analyzer/h5Format2Root.py $dir
EOF 
