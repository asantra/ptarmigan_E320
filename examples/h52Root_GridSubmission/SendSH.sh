#! /bin/bash
targetDirectory="/srv01/agrp/arkas/PtarmiganH52Root"
sftp arkas@wipp-an1 << EOF
put Gridh5Format2Root.py $targetDirectory
put GridTextFormat2Root.py $targetDirectory
put gridScript.sh $targetDirectory
put runLocal.sh $targetDirectory
put sendJobs.sh $targetDirectory
put resendJobs.sh $targetDirectory
EOF
