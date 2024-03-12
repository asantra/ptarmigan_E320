#! /bin/bash

### how many jobs you want to submit, if -1, then submits jobs you set
nJobs=${1:-"-1"}
#### runid for the output job name in the grid, increased by 1 for each job
runid=2
### submitting upto nJobs you wanted
itSt=2
itEnd=3

for ((iter=itSt; iter<=itEnd; iter++)); do
    ### a counter value
    b=1
    ### runId increased by one
    runid=$(( $runid + $b ))
    echo "runid: "$runid
    ### the place where the output and error file of the grid will live
    # DESTINATION="/storage/agrp/arkas/PtarmiganGridOutput"
    # DESTINATION="/storage/agrp/arkas/PtarmiganGridOutputVertical"
    DESTINATION="/storage/agrp/arkas/PtarmiganLUXEGridOutputHorizontal"
    # DESTINATION="/storage/agrp/arkas/PtarmiganLUXEGridOutputVertical"
    ### create the main directory if it does not exists
    mkdir -p ${DESTINATION}
    
    ### if main directory/run_id exists, delete
    if [[ -d "${DESTINATION}/run_$runid" ]]; then
        echo "Found a directory with output ${DESTINATION}/run_$runid! Deleting the previous one."
        rm -rf ${DESTINATION}/run_$runid
    fi

    # OUTDIRLOC="/storage/agrp/arkas/PtarmiganWorkArea"
    # OUTDIRLOC="/storage/agrp/arkas/PtarmiganWorkAreaVertical"
    OUTDIRLOC="/storage/agrp/arkas/PtarmiganLUXEWorkAreaHorizontal"
    # OUTDIRLOC="/storage/agrp/arkas/PtarmiganLUXEWorkAreaVertical"
    #### create the run directory
    mkdir -p ${DESTINATION}"/run_"$runid
    mkdir -p ${OUTDIRLOC}
    #### from where you are submitting jobs
    PRESENTDIRECTORY=${PWD}
    #### submit jobs to the PBS system
    qsub -l ncpus=1,mem=24gb,io=2 -v parname1=${runid},parname2=${PRESENTDIRECTORY},parname3=${OUTDIRLOC} -q N -N "run_"$runid -o "${DESTINATION}/run_"${runid} -e "${DESTINATION}/run_"${runid} gridScript.sh
    ### sleep for 1 s, so that there is no problem in submitting jobs to the grid
    sleep 1s
    ### if number of jobs required is reached then break the loop
    if [[ $runid -eq $nJobs ]]; then
        break
    fi
done
