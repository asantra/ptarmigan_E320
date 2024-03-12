#! /bin/bash

### how many jobs you want to submit, if -1, then submits jobs you set
fileExtns=${1:-"ER"}
#### runid for the output job name in the grid, increased by 1 for each job
runid=12

### submitting upto nJobs you wanted
itSt=12
itEnd=13

for ((iter=itSt; iter<=itEnd; iter++)); do
    ### a counter value
    b=1
    ### runId increased by one
    runid=$(( $runid + $b ))
    echo "runid: "$runid
    ### the place where the output and error file of the grid will live
    # DESTINATION="/storage/agrp/arkas/PtarmiganGridOutput"
    # DESTINATION="/storage/agrp/arkas/PtarmiganGridOutputVertical"
    # DESTINATION="/storage/agrp/arkas/PtarmiganLUXEGridOutputHorizontal"
    DESTINATION="/storage/agrp/arkas/PtarmiganLUXEGridOutputVertical"


    # OUTDIRLOC="/storage/agrp/arkas/PtarmiganWorkArea"
    # OUTDIRLOC="/storage/agrp/arkas/PtarmiganWorkAreaVertical"
    # OUTDIRLOC="/storage/agrp/arkas/PtarmiganLUXEWorkAreaHorizontal"
    OUTDIRLOC="/storage/agrp/arkas/PtarmiganLUXEWorkAreaVertical"
    #### from where you are submitting jobs
    PRESENTDIRECTORY=${PWD}
    
    flag=0
    if ls ${DESTINATION}/run_$runid/*${fileExtns} &> /dev/null
    then
        if grep -rnwq ${DESTINATION}/run_$runid/*${fileExtns} -e "error"
        then
            flag=1
        elif grep -rnwq ${DESTINATION}/run_$runid/*${fileExtns} -e "Killed"
        then
            flag=1
        elif grep -rnwq ${DESTINATION}/run_$runid/*${fileExtns} -e "Aborting"
        then
            flag=1
        elif grep -rnwq ${DESTINATION}/run_$runid/*${fileExtns} -e "Caught Geant4 exception"
        then
            flag=1
        elif grep -rnwq ${DESTINATION}/run_$runid/*${fileExtns} -e "Interrupted"
        then
            flag=1
        elif grep -rnwq ${DESTINATION}/run_$runid/*${fileExtns} -e "Aborted"
        then
            flag=1
        else
            flag=0
        fi
    else
        flag=2
    fi
    
    #### if there is error, resubmit them
    if [[ $flag -eq 1 ]]
    then 
        echo "At least one error for ${DESTINATION}/run_$runid"
        
        if [[ -d "${DESTINATION}/run_$runid" ]]; then
            echo "Found a directory with output ${DESTINATION}/run_$runid! Deleting the previous one."
            rm -rf ${DESTINATION}/run_$runid
        fi
        
        #### create the run directory
        mkdir -p ${DESTINATION}"/run_"$runid
        #### from where you are submitting jobs
        PRESENTDIRECTORY=${PWD}
        #### submit jobs to the PBS system
        qsub -l ncpus=1,mem=24gb,io=2 -v parname1=${runid},parname2=${PRESENTDIRECTORY},parname3=${OUTDIRLOC} -q N -N "run_"$runid -o "${DESTINATION}/run_"${runid} -e "${DESTINATION}/run_"${runid} gridScript.sh
        ### sleep for 1 s, so that there is no problem in submitting jobs to the grid
        sleep 1s
    elif [[ $flag -eq 2 ]]
    then
        echo "This folder does not exist: ${DESTINATION}/run_$runid"
    else
        echo "No problem in this set: ${DESTINATION}/run_$runid"
    fi
done