#! /bin/bash
targetDirectory="/srv01/agrp/arkas/PtarmiganGridRuns"
sftp arkas@wipp-an1 << EOF
put E320_profile_MASTER.yml $targetDirectory
put luxe_MASTER.yml $targetDirectory
put luxe_tdr_MASTER.yml $targetDirectory
put gridScript.sh $targetDirectory
put sendJobs.sh $targetDirectory
put resendJobs.sh $targetDirectory
EOF
