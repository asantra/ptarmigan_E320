#! /bin/bash

### how many jobs you want to submit, if -1, then submits jobs you set
nJobs=${1:-"-1"}
version=${2:-"1"}
#### runid for the output job name in the grid, increased by 1 for each job
runid=0

### submitting upto nJobs you wanted
itSt=1
itEnd=250

for ((iter=itSt; iter<=itEnd; iter++)); do
    ### a counter value
    b=1
    ### runId increased by one
    runid=$(( $runid + $b ))
    echo "runid: "$runid
    
    time python GridTextFormat2Root.py -x 10 -g 10 -v ${version} -i ${runid}
    ### sleep for 1 s, so that there is no problem in submitting jobs to the grid
    sleep 1s
    ### if number of jobs required is reached then break the loop
    if [[ $runid -eq $nJobs ]]; then
        break
    fi
done
